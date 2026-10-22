//! Reglas de negocio de ventas (POS): cálculo de totales, validación de
//! stock y orquestación de la venta dentro de una transacción.

use crate::models::{AbonoVentaInput, ItemVentaInput, NuevaVenta, VentaActualizada};use crate::repos::{clientes, productos, ventas};
use crate::services::dinero::{redondear2, subtotal_linea};
use crate::services::reglas::{estado_pago, validar_metodo_pago_venta};
use rusqlite::Connection;

pub struct Totales {
    pub subtotal: f64,
    pub descuento: f64,
    pub total_neto: f64,
    pub monto_pagado: f64,
    pub saldo_pendiente: f64,
}

/// Calcula los totales de una venta aplicando las reglas del negocio:
/// - el descuento nunca es negativo ni supera el subtotal;
/// - el monto pagado se limita al rango [0, total_neto];
/// - todo queda redondeado a 2 decimales.
pub fn calcular_totales(
    items: &[ItemVentaInput],
    descuento: Option<f64>,
    monto_pagado: Option<f64>,
) -> Result<Totales, String> {
    let subtotal = redondear2(
        items
            .iter()
            .map(|i| i.cantidad as f64 * i.precio_unitario)
            .sum(),
    );
    let descuento = redondear2(descuento.unwrap_or(0.0).max(0.0).min(subtotal));
    let total_neto = redondear2(subtotal - descuento);
    let monto_pagado = redondear2(monto_pagado.unwrap_or(total_neto).clamp(0.0, total_neto));
    let saldo_pendiente = redondear2(total_neto - monto_pagado);
    Ok(Totales {
        subtotal,
        descuento,
        total_neto,
        monto_pagado,
        saldo_pendiente,
    })
}

fn validar_items(conn: &Connection, items: &[ItemVentaInput]) -> Result<(), String> {
    for item in items {
        if item.cantidad <= 0 {
            return Err("La cantidad de cada ítem debe ser mayor que cero".to_string());
        }
        if item.precio_unitario < 0.0 {
            return Err("El precio de cada ítem no puede ser negativo".to_string());
        }
        let stock = productos::obtener_stock(conn, item.producto_id)
            .map_err(crate::errors::bd)?
            .ok_or_else(|| format!("Producto {} no encontrado", item.producto_id))?;
        if item.cantidad > stock {
            return Err(format!(
                "Stock insuficiente para el producto {} (disponible: {stock}, solicitado: {})",
                item.producto_id, item.cantidad
            ));
        }
    }
    Ok(())
}

fn validar_cliente(conn: &Connection, cliente_id: Option<i64>) -> Result<(), String> {
    if let Some(id) = cliente_id {
        let existe = clientes::obtener(conn, id)
            .map_err(crate::errors::bd)?
            .is_some();
        if !existe {
            return Err("El cliente seleccionado no existe".to_string());
        }
    }
    Ok(())
}

/// Registra una venta. El descuento de inventario ocurre vía trigger
/// `trg_descontar_stock_venta` al insertar en `items_venta`.
pub fn crear(conn: &Connection, venta: &NuevaVenta) -> Result<i64, String> {
    if venta.items.is_empty() {
        return Err("La venta debe tener al menos un artículo".to_string());
    }
    validar_metodo_pago_venta(&venta.metodo_pago)?;
    validar_items(conn, &venta.items)?;
    validar_cliente(conn, venta.cliente_id)?;

    let totales = calcular_totales(&venta.items, venta.descuento, venta.monto_pagado)?;
    let venta_id = ventas::insertar_venta(
        conn,
        totales.subtotal,
        totales.descuento,
        totales.total_neto,
        totales.monto_pagado,
        totales.saldo_pendiente,
        &venta.metodo_pago,
        estado_pago(totales.saldo_pendiente, totales.total_neto),
        &venta.cliente_id,
        &venta.referencia_cliente,
        &venta.notas,
    )
    .map_err(crate::errors::bd)?;

    for item in &venta.items {
        let costo = productos::obtener_costo(conn, item.producto_id)
            .map_err(crate::errors::bd)?
            .ok_or_else(|| format!("Producto {} no encontrado", item.producto_id))?;
        ventas::insertar_item(
            conn,
            venta_id,
            item.producto_id,
            item.cantidad,
            item.precio_unitario,
            costo,
            subtotal_linea(item.cantidad, item.precio_unitario),
        )
        .map_err(crate::errors::bd)?;
    }

    Ok(venta_id)
}

/// Actualiza una venta existente (cabecera + ítems).
pub fn actualizar(conn: &Connection, venta_id: i64, venta: &VentaActualizada) -> Result<(), String> {
    let actual = ventas::obtener(conn, venta_id)
        .map_err(crate::errors::bd)?
        .ok_or_else(|| "Venta no encontrada".to_string())?;
    if venta.items.is_empty() {
        return Err("La venta debe tener al menos un artículo".to_string());
    }
    validar_metodo_pago_venta(&venta.metodo_pago)?;
    // El trigger repone el stock al borrar los ítems viejos, así que la
    // validación se hace contra el stock ya restaurado… en la práctica el
    // borrado ocurre dentro de `reemplazar`, después de validar. Para no
    // rechazar ediciones legítimas, aquí se valida contra el stock actual
    // más lo que la propia venta tenía reservado.
    validar_items_para_edicion(conn, venta_id, &venta.items)?;
    // Lo que la edición no trae se conserva: el cliente y lo ya pagado no se
    // pueden borrar por accidente desde el formulario de edición.
    let cliente_id = venta.cliente_id.or(actual.cliente_id);
    validar_cliente(conn, cliente_id)?;
    // Si la venta tiene abonos registrados, lo pagado es su suma (fuente de
    // verdad del dinero recibido); si no, se conserva lo que ya estaba.
    let abonos = ventas::sumar_abonos(conn, venta_id).map_err(crate::errors::bd)?;
    let monto_pagado = if abonos > 0.0 {
        Some(abonos)
    } else {
        Some(venta.monto_pagado.unwrap_or(actual.monto_pagado))
    };

    let totales = calcular_totales(&venta.items, venta.descuento, monto_pagado)?;
    let filas = ventas::reemplazar(
        conn,
        venta_id,
        totales.subtotal,
        totales.descuento,
        totales.total_neto,
        totales.monto_pagado,
        totales.saldo_pendiente,
        &venta.metodo_pago,
        estado_pago(totales.saldo_pendiente, totales.total_neto),
        &cliente_id,
        &venta.referencia_cliente,
    )
    .map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Venta no encontrada".to_string());
    }

    for item in &venta.items {
        let costo = productos::obtener_costo(conn, item.producto_id)
            .map_err(crate::errors::bd)?
            .ok_or_else(|| format!("Producto {} no encontrado", item.producto_id))?;
        ventas::insertar_item(
            conn,
            venta_id,
            item.producto_id,
            item.cantidad,
            item.precio_unitario,
            costo,
            subtotal_linea(item.cantidad, item.precio_unitario),
        )
        .map_err(crate::errors::bd)?;
    }

    Ok(())
}

/// Valida stock para una edición: el stock disponible real es el actual más
/// las unidades que la venta ya tenía apartadas (el borrado las devuelve).
fn validar_items_para_edicion(
    conn: &Connection,
    venta_id: i64,
    items: &[ItemVentaInput],
) -> Result<(), String> {
    let reservado = ventas::listar_items(conn, venta_id).map_err(crate::errors::bd)?;
    for item in items {
        if item.cantidad <= 0 {
            return Err("La cantidad de cada ítem debe ser mayor que cero".to_string());
        }
        if item.precio_unitario < 0.0 {
            return Err("El precio de cada ítem no puede ser negativo".to_string());
        }
        let stock = productos::obtener_stock(conn, item.producto_id)
            .map_err(crate::errors::bd)?
            .ok_or_else(|| format!("Producto {} no encontrado", item.producto_id))?;
        let ya_reservado: i64 = reservado
            .iter()
            .filter(|r| r.producto_id == item.producto_id)
            .map(|r| r.cantidad)
            .sum();
        if item.cantidad > stock + ya_reservado {
            return Err(format!(
                "Stock insuficiente para el producto {} (disponible: {}, solicitado: {})",
                item.producto_id,
                stock + ya_reservado,
                item.cantidad
            ));
        }
    }
    Ok(())
}

pub fn eliminar(conn: &Connection, venta_id: i64) -> Result<(), String> {
    let filas = ventas::eliminar(conn, venta_id).map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Venta no encontrada".to_string());
    }
    Ok(())
}

/// Registra un abono a una venta a crédito y recalcula su saldo.
pub fn registrar_abono(
    conn: &Connection,
    venta_id: i64,
    abono: &AbonoVentaInput,
) -> Result<i64, String> {
    validar_metodo_pago_venta(&abono.metodo_pago)?;

    let (saldo_pendiente, total_neto) = ventas::obtener_saldo(conn, venta_id)
        .map_err(crate::errors::bd)?
        .ok_or_else(|| "Venta no encontrada".to_string())?;

    let monto = redondear2(abono.monto);
    if monto <= 0.0 || monto > saldo_pendiente {
        return Err("El abono debe ser mayor que cero y no superar el saldo pendiente".to_string());
    }

    let abono_id = ventas::insertar_abono(
        conn,
        venta_id,
        monto,
        &abono.metodo_pago,
        &abono.numero_referencia,
        &abono.nota,
    )
    .map_err(crate::errors::bd)?;

    let nuevo_saldo = redondear2(saldo_pendiente - monto);
    ventas::aplicar_abono(
        conn,
        venta_id,
        nuevo_saldo,
        monto,
        estado_pago(nuevo_saldo, total_neto),
    )
    .map_err(crate::errors::bd)?;

    Ok(abono_id)
}
