//! Reglas de negocio de compras a proveedores (facturas y abonos).

use crate::models::{AbonoInput, ItemFacturaInput, NuevaFactura};
use crate::repos::{productos, proveedores};
use crate::services::dinero::{redondear2, subtotal_linea};
use crate::services::reglas::{estado_pago, validar_metodo_pago_compra};
use rusqlite::Connection;

fn validar_item(item: &ItemFacturaInput) -> Result<(), String> {
    if item.cantidad <= 0 {
        return Err("La cantidad de cada ítem debe ser mayor que cero".to_string());
    }
    if item.costo_unitario < 0.0 {
        return Err("El costo unitario de cada ítem no puede ser negativo".to_string());
    }
    if let Some(pv) = item.precio_venta_sugerido {
        if pv < 0.0 {
            return Err("El precio de venta sugerido no puede ser negativo".to_string());
        }
    }
    if item.producto_id.is_none() {
        let nombre = item.nombre_nuevo.as_deref().unwrap_or("").trim();
        if nombre.is_empty() {
            return Err("Falta el nombre de la prenda nueva".to_string());
        }
    }
    Ok(())
}

/// Registra una compra (factura de proveedor). Los ítems alimentan el
/// inventario automáticamente vía trigger `trg_aumentar_stock_compra`.
pub fn crear_factura(conn: &Connection, factura: &NuevaFactura) -> Result<i64, String> {
    if factura.items.is_empty() {
        return Err("La compra debe tener al menos un ítem".to_string());
    }
    if proveedores::obtener_proveedor(conn, factura.proveedor_id)
        .map_err(crate::errors::bd)?
        .is_none()
    {
        return Err("El proveedor seleccionado no existe".to_string());
    }
    for item in &factura.items {
        validar_item(item)?;
        if let Some(id) = item.producto_id {
            if productos::obtener(conn, id)
                .map_err(crate::errors::bd)?
                .is_none()
            {
                return Err(format!("Producto {id} no encontrado"));
            }
        }
    }
    if let Some(ref abono) = factura.abono_inicial {
        validar_metodo_pago_compra(&abono.metodo_pago)?;
        if abono.monto <= 0.0 {
            return Err("El abono inicial debe ser mayor que cero".to_string());
        }
    }

    let monto_total = redondear2(
        factura
            .items
            .iter()
            .map(|i| i.cantidad as f64 * i.costo_unitario)
            .sum(),
    );

    if let Some(ref abono) = factura.abono_inicial {
        if redondear2(abono.monto) > monto_total {
            return Err("El abono inicial no puede superar el total de la compra".to_string());
        }
    }

    let abono_inicial = redondear2(factura.abono_inicial.as_ref().map(|a| a.monto).unwrap_or(0.0));
    let saldo_inicial = redondear2(monto_total - abono_inicial);

    let factura_id = proveedores::insertar_factura(
        conn,
        factura.proveedor_id,
        &factura.numero_factura,
        &factura.fecha_emision,
        &factura.fecha_limite_pago,
        monto_total,
        saldo_inicial,
        estado_pago(saldo_inicial, monto_total),
    )
    .map_err(crate::errors::bd)?;

    for item in &factura.items {
        let producto_id = match item.producto_id {
            Some(id) => id,
            None => {
                let nombre = item.nombre_nuevo.clone().unwrap_or_default();
                productos::insertar(
                    conn,
                    &crate::models::Producto {
                        id: None,
                        sku: item.sku_nuevo.clone(),
                        nombre,
                        categoria: item
                            .categoria_nueva
                            .clone()
                            .unwrap_or_else(|| "General".to_string()),
                        talla: item
                            .talla_nueva
                            .clone()
                            .unwrap_or_else(|| "Única".to_string()),
                        color: item.color_nuevo.clone(),
                        costo_unitario: item.costo_unitario,
                        precio_venta: item.precio_venta_sugerido.unwrap_or(0.0),
                        stock_actual: 0,
                        stock_minimo: item.stock_minimo_nuevo.unwrap_or(3).max(0),
                        estado: "activo".to_string(),
                    },
                )
                .map_err(|e| crate::errors::base_datos(e, None))?
            }
        };

        proveedores::insertar_item_factura(
            conn,
            factura_id,
            producto_id,
            item.cantidad,
            item.costo_unitario,
            &item.precio_venta_sugerido,
            subtotal_linea(item.cantidad, item.costo_unitario),
        )
        .map_err(crate::errors::bd)?;
    }

    if let Some(abono) = &factura.abono_inicial {
        let abono_normalizado = AbonoInput {
            monto: redondear2(abono.monto),
            ..abono.clone()
        };
        proveedores::insertar_abono(conn, factura_id, &abono_normalizado).map_err(crate::errors::bd)?;
    }

    Ok(factura_id)
}

/// Registra un abono a una factura de proveedor y recalcula su saldo.
pub fn registrar_abono(
    conn: &Connection,
    factura_id: i64,
    abono: &AbonoInput,
) -> Result<i64, String> {
    validar_metodo_pago_compra(&abono.metodo_pago)?;

    let (saldo_pendiente, monto_total) = proveedores::obtener_saldo_factura(conn, factura_id)
        .map_err(crate::errors::bd)?
        .ok_or_else(|| "Factura no encontrada".to_string())?;

    let monto = redondear2(abono.monto);
    if monto <= 0.0 || monto > saldo_pendiente {
        return Err("El abono debe ser mayor que cero y no superar el saldo pendiente".to_string());
    }

    let abono_normalizado = AbonoInput {
        monto,
        ..abono.clone()
    };
    let abono_id =
        proveedores::insertar_abono(conn, factura_id, &abono_normalizado).map_err(crate::errors::bd)?;

    let nuevo_saldo = redondear2(saldo_pendiente - monto);
    proveedores::actualizar_saldo_factura(
        conn,
        factura_id,
        nuevo_saldo,
        estado_pago(nuevo_saldo, monto_total),
    )
    .map_err(crate::errors::bd)?;

    Ok(abono_id)
}
