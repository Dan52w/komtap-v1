use crate::db::DbConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

const METODOS_PAGO_VALIDOS: [&str; 5] = ["Efectivo", "Transferencia", "Tarjeta", "Mixto", "Credito"];

fn validar_metodo_pago(metodo: &str) -> Result<(), String> {
    if !METODOS_PAGO_VALIDOS.contains(&metodo) {
        return Err("Método de pago no válido".to_string());
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct ItemVentaInput {
    pub producto_id: i64,
    pub cantidad: i64,
    pub precio_unitario: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NuevaVenta {
    pub items: Vec<ItemVentaInput>,
    pub descuento: Option<f64>,
    pub metodo_pago: String,
    pub monto_pagado: Option<f64>, // si es None, se asume pago total del neto
    /// Nota libre y opcional (ej. nombre/teléfono) útil solo si la venta queda
    /// con saldo pendiente y se necesita identificar a quién cobrarle luego.
    pub referencia_cliente: Option<String>,
    pub notas: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Venta {
    pub id: i64,
    pub fecha: String,
    pub subtotal: f64,
    pub descuento: f64,
    pub total_neto: f64,
    pub monto_pagado: f64,
    pub saldo_pendiente: f64,
    pub metodo_pago: String,
    pub estado_pago: String,
    pub referencia_cliente: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemVenta {
    pub id: i64,
    pub producto_id: i64,
    pub producto_nombre: String,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AbonoVentaInput {
    pub monto: f64,
    pub metodo_pago: String,
    pub numero_referencia: Option<String>,
    pub nota: Option<String>,
}

fn estado_pago_para(saldo_pendiente: f64, total_neto: f64) -> &'static str {
    if saldo_pendiente <= 0.0 {
        "Pagado"
    } else if saldo_pendiente < total_neto {
        "Parcial"
    } else {
        "Pendiente"
    }
}

/// Registra una venta. Valida stock disponible por ítem antes de confirmar;
/// el descuento de inventario ocurre automáticamente vía trigger
/// `trg_descontar_stock_venta` al insertar en `items_venta`.
#[tauri::command]
pub fn crear_venta(state: State<DbConnection>, venta: NuevaVenta) -> Result<i64, String> {
    if venta.items.is_empty() {
        return Err("La venta debe tener al menos un artículo".to_string());
    }
    validar_metodo_pago(&venta.metodo_pago)?;

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Validar stock disponible de cada ítem.
    for item in &venta.items {
        if item.cantidad <= 0 {
            return Err("La cantidad de cada ítem debe ser mayor que cero".to_string());
        }
        let stock_actual: i64 = tx.query_row(
            "SELECT stock_actual FROM productos WHERE id = ?1", [item.producto_id], |row| row.get(0),
        ).map_err(|_| format!("Producto {} no encontrado", item.producto_id))?;

        if item.cantidad > stock_actual {
            return Err(format!(
                "Stock insuficiente para el producto {} (disponible: {stock_actual}, solicitado: {})",
                item.producto_id, item.cantidad
            ));
        }
    }

    let subtotal: f64 = venta.items.iter()
        .map(|i| (i.cantidad as f64 * i.precio_unitario * 100.0).round() / 100.0)
        .sum();
    let descuento = venta.descuento.unwrap_or(0.0).max(0.0);
    let total_neto = (subtotal - descuento).max(0.0);
    let monto_pagado = venta.monto_pagado.unwrap_or(total_neto).min(total_neto).max(0.0);
    let saldo_pendiente = total_neto - monto_pagado;

    tx.execute(
        "INSERT INTO ventas (subtotal, descuento, total_neto, monto_pagado, saldo_pendiente, metodo_pago, estado_pago, referencia_cliente, notas, fecha)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now','localtime'))",
        (
            subtotal, descuento, total_neto, monto_pagado, saldo_pendiente,
            &venta.metodo_pago, estado_pago_para(saldo_pendiente, total_neto), &venta.referencia_cliente, &venta.notas,
        ),
    ).map_err(|e| e.to_string())?;

    let venta_id = tx.last_insert_rowid();

    for item in &venta.items {
        let costo_unitario: f64 = tx.query_row(
            "SELECT costo_unitario FROM productos WHERE id = ?1", [item.producto_id], |row| row.get(0),
        ).map_err(|e| e.to_string())?;
        let item_subtotal = (item.cantidad as f64 * item.precio_unitario * 100.0).round() / 100.0;

        tx.execute(
            "INSERT INTO items_venta (venta_id, producto_id, cantidad, precio_unitario, costo_unitario, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (venta_id, item.producto_id, item.cantidad, item.precio_unitario, costo_unitario, item_subtotal),
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(venta_id)
}

#[tauri::command]
pub fn listar_ventas(state: State<DbConnection>, limite: Option<i64>, offset: Option<i64>, mes: Option<String>) -> Result<Vec<Venta>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let limite = limite.unwrap_or(50).clamp(1, 500);
    let offset = offset.unwrap_or(0).max(0);

    let mut stmt = conn.prepare(
        "SELECT id, fecha, subtotal, descuento, total_neto,
                monto_pagado, saldo_pendiente, metodo_pago, estado_pago, referencia_cliente
         FROM ventas
         WHERE (?1 IS NULL OR substr(fecha, 1, 7) = ?1)
         ORDER BY fecha DESC LIMIT ?2 OFFSET ?3"
    ).map_err(|e| e.to_string())?;

    let ventas = stmt.query_map((mes, limite, offset), |row| {
        Ok(Venta {
            id: row.get(0)?,
            fecha: row.get(1)?,
            subtotal: row.get(2)?,
            descuento: row.get(3)?,
            total_neto: row.get(4)?,
            monto_pagado: row.get(5)?,
            saldo_pendiente: row.get(6)?,
            metodo_pago: row.get(7)?,
            estado_pago: row.get(8)?,
            referencia_cliente: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(ventas)
}

#[tauri::command]
pub fn listar_items_venta(state: State<DbConnection>, venta_id: i64) -> Result<Vec<ItemVenta>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT iv.id, iv.producto_id, p.nombre, iv.cantidad, iv.precio_unitario, iv.subtotal
         FROM items_venta iv JOIN productos p ON p.id = iv.producto_id
         WHERE iv.venta_id = ?1"
    ).map_err(|e| e.to_string())?;

    let items = stmt.query_map([venta_id], |row| {
        Ok(ItemVenta {
            id: row.get(0)?,
            producto_id: row.get(1)?,
            producto_nombre: row.get(2)?,
            cantidad: row.get(3)?,
            precio_unitario: row.get(4)?,
            subtotal: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(items)
}

#[tauri::command]
pub fn registrar_abono_venta(state: State<DbConnection>, venta_id: i64, abono: AbonoVentaInput) -> Result<i64, String> {
    validar_metodo_pago(&abono.metodo_pago)?;

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let (saldo_pendiente, total_neto): (f64, f64) = tx.query_row(
        "SELECT saldo_pendiente, total_neto FROM ventas WHERE id = ?1",
        [venta_id], |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| "Venta no encontrada".to_string())?;

    if abono.monto <= 0.0 || abono.monto > saldo_pendiente {
        return Err("El abono debe ser mayor que cero y no superar el saldo pendiente".to_string());
    }

    tx.execute(
        "INSERT INTO abonos_venta (venta_id, monto, metodo_pago, numero_referencia, nota, fecha)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now','localtime'))",
        (venta_id, abono.monto, &abono.metodo_pago, &abono.numero_referencia, &abono.nota),
    ).map_err(|e| e.to_string())?;

    let nuevo_saldo = saldo_pendiente - abono.monto;
    let nuevo_pagado_delta = abono.monto;
    tx.execute(
        "UPDATE ventas SET saldo_pendiente = ?1, monto_pagado = monto_pagado + ?2, estado_pago = ?3 WHERE id = ?4",
        (nuevo_saldo, nuevo_pagado_delta, estado_pago_para(nuevo_saldo, total_neto), venta_id),
    ).map_err(|e| e.to_string())?;

    let abono_id = tx.last_insert_rowid();
    tx.commit().map_err(|e| e.to_string())?;
    Ok(abono_id)
}

#[derive(Serialize, Deserialize)]
pub struct TopPrenda {
    pub producto_id: i64,
    pub nombre: String,
    pub unidades_vendidas: i64,
    pub ingreso_total: f64,
    pub margen_estimado: f64,
}

/// HU-05: top de prendas más vendidas. `dias` filtra por ventana relativa
/// (ej. 1 = hoy, 30 = último mes); None = histórico completo.
#[tauri::command]
pub fn top_prendas_vendidas(state: State<DbConnection>, dias: Option<i64>, limite: Option<i64>) -> Result<Vec<TopPrenda>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let limite = limite.unwrap_or(10).clamp(1, 100);

    let condicion_fecha = match dias {
        Some(d) => format!("AND v.fecha >= datetime('now','localtime','-{d} days')"),
        None => String::new(),
    };

    let sql = format!(
        "SELECT iv.producto_id, p.nombre,
                SUM(iv.cantidad) AS unidades_vendidas,
                SUM(iv.subtotal) AS ingreso_total,
                SUM((iv.precio_unitario - iv.costo_unitario) * iv.cantidad) AS margen_estimado
         FROM items_venta iv
         JOIN ventas v ON v.id = iv.venta_id
         JOIN productos p ON p.id = iv.producto_id
         WHERE 1=1 {condicion_fecha}
         GROUP BY iv.producto_id
         ORDER BY unidades_vendidas DESC
         LIMIT ?1"
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let resultado = stmt.query_map([limite], |row| {
        Ok(TopPrenda {
            producto_id: row.get(0)?,
            nombre: row.get(1)?,
            unidades_vendidas: row.get(2)?,
            ingreso_total: row.get(3)?,
            margen_estimado: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(resultado)
}

#[derive(Serialize, Deserialize)]
pub struct VentaActualizada {
    pub items: Vec<ItemVentaInput>,
    pub descuento: Option<f64>,
    pub metodo_pago: String,
    pub monto_pagado: Option<f64>,
    pub referencia_cliente: Option<String>,
}

#[tauri::command]
pub fn actualizar_venta(state: State<DbConnection>, venta_id: i64, venta: VentaActualizada) -> Result<(), String> {
    if venta.items.is_empty() {
        return Err("La venta debe tener al menos un artículo".to_string());
    }
    validar_metodo_pago(&venta.metodo_pago)?;

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // El trigger trg_restaurar_stock_cancelacion_venta repone el stock al borrar.
    tx.execute("DELETE FROM items_venta WHERE venta_id = ?1", [venta_id]).map_err(|e| e.to_string())?;

    for item in &venta.items {
        if item.cantidad <= 0 {
            return Err("La cantidad de cada ítem debe ser mayor que cero".to_string());
        }
        let stock_actual: i64 = tx.query_row(
            "SELECT stock_actual FROM productos WHERE id = ?1", [item.producto_id], |row| row.get(0),
        ).map_err(|_| format!("Producto {} no encontrado", item.producto_id))?;
        if item.cantidad > stock_actual {
            return Err(format!("Stock insuficiente para el producto {}", item.producto_id));
        }
    }

    let subtotal: f64 = venta.items.iter()
        .map(|i| (i.cantidad as f64 * i.precio_unitario * 100.0).round() / 100.0)
        .sum();
    let descuento = venta.descuento.unwrap_or(0.0).max(0.0);
    let total_neto = (subtotal - descuento).max(0.0);
    let monto_pagado = venta.monto_pagado.unwrap_or(total_neto).min(total_neto).max(0.0);
    let saldo_pendiente = total_neto - monto_pagado;

    tx.execute(
        "UPDATE ventas SET subtotal=?1, descuento=?2, total_neto=?3, monto_pagado=?4, saldo_pendiente=?5,
                metodo_pago=?6, estado_pago=?7, referencia_cliente=?8 WHERE id=?9",
        (subtotal, descuento, total_neto, monto_pagado, saldo_pendiente,
         &venta.metodo_pago, estado_pago_para(saldo_pendiente, total_neto), &venta.referencia_cliente, venta_id),
    ).map_err(|e| e.to_string())?;

    for item in &venta.items {
        let costo_unitario: f64 = tx.query_row(
            "SELECT costo_unitario FROM productos WHERE id = ?1", [item.producto_id], |row| row.get(0),
        ).map_err(|e| e.to_string())?;
        let item_subtotal = (item.cantidad as f64 * item.precio_unitario * 100.0).round() / 100.0;
        tx.execute(
            "INSERT INTO items_venta (venta_id, producto_id, cantidad, precio_unitario, costo_unitario, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (venta_id, item.producto_id, item.cantidad, item.precio_unitario, costo_unitario, item_subtotal),
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())
}