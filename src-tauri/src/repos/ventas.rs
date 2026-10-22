//! Persistencia de ventas (POS), sus ítems y abonos a crédito.

use crate::models::{AbonoVenta, ItemVenta, TopPrenda, Venta};
use rusqlite::{Connection, Result as SqlResult, Row};

fn mapear_venta(fila: &Row) -> SqlResult<Venta> {
    Ok(Venta {
        id: fila.get(0)?,
        fecha: fila.get(1)?,
        subtotal: fila.get(2)?,
        descuento: fila.get(3)?,
        total_neto: fila.get(4)?,
        monto_pagado: fila.get(5)?,
        saldo_pendiente: fila.get(6)?,
        metodo_pago: fila.get(7)?,
        estado_pago: fila.get(8)?,
        cliente_id: fila.get(9)?,
        cliente_nombre: fila.get(10)?,
        referencia_cliente: fila.get(11)?,
    })
}

const COLUMNAS_VENTA: &str = "v.id, v.fecha, v.subtotal, v.descuento, v.total_neto, v.monto_pagado, v.saldo_pendiente, v.metodo_pago, v.estado_pago, v.cliente_id, c.nombre, v.referencia_cliente";
const JOIN_CLIENTE: &str = "ventas v LEFT JOIN clientes c ON c.id = v.cliente_id";

pub fn existe(conn: &Connection, venta_id: i64) -> SqlResult<bool> {
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM ventas WHERE id = ?1)",
        [venta_id],
        |fila| fila.get(0),
    )
}

pub fn insertar_venta(
    conn: &Connection,
    subtotal: f64,
    descuento: f64,
    total_neto: f64,
    monto_pagado: f64,
    saldo_pendiente: f64,
    metodo_pago: &str,
    estado_pago: &str,
    cliente_id: &Option<i64>,
    referencia_cliente: &Option<String>,
    notas: &Option<String>,
) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO ventas (subtotal, descuento, total_neto, monto_pagado, saldo_pendiente, metodo_pago, estado_pago, cliente_id, referencia_cliente, notas, fecha)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, datetime('now','localtime'))",
        (
            subtotal,
            descuento,
            total_neto,
            monto_pagado,
            saldo_pendiente,
            metodo_pago,
            estado_pago,
            cliente_id,
            referencia_cliente,
            notas,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn insertar_item(
    conn: &Connection,
    venta_id: i64,
    producto_id: i64,
    cantidad: i64,
    precio_unitario: f64,
    costo_unitario: f64,
    subtotal: f64,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO items_venta (venta_id, producto_id, cantidad, precio_unitario, costo_unitario, subtotal)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            venta_id,
            producto_id,
            cantidad,
            precio_unitario,
            costo_unitario,
            subtotal,
        ),
    )?;
    Ok(())
}

pub fn listar(
    conn: &Connection,
    mes: &Option<String>,
    limite: i64,
    offset: i64,
) -> SqlResult<Vec<Venta>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {COLUMNAS_VENTA} FROM {JOIN_CLIENTE}
         WHERE (?1 IS NULL OR substr(v.fecha, 1, 7) = ?1)
         ORDER BY v.fecha DESC LIMIT ?2 OFFSET ?3"
    ))?;
    let filas: Vec<Venta> = stmt
        .query_map((mes, limite, offset), mapear_venta)?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

pub fn listar_items(conn: &Connection, venta_id: i64) -> SqlResult<Vec<ItemVenta>> {
    let mut stmt = conn.prepare(
        "SELECT iv.id, iv.producto_id, p.nombre, iv.cantidad, iv.precio_unitario, iv.subtotal
         FROM items_venta iv JOIN productos p ON p.id = iv.producto_id
         WHERE iv.venta_id = ?1",
    )?;
    let filas: Vec<ItemVenta> = stmt
        .query_map([venta_id], |fila| {
            Ok(ItemVenta {
                id: fila.get(0)?,
                producto_id: fila.get(1)?,
                producto_nombre: fila.get(2)?,
                cantidad: fila.get(3)?,
                precio_unitario: fila.get(4)?,
                subtotal: fila.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

/// Reemplaza la cabecera y los ítems de una venta existente.
/// El trigger `trg_restaurar_stock_cancelacion_venta` repone el stock al
/// borrar los ítems viejos, y `trg_descontar_stock_venta` lo descuenta al
/// insertar los nuevos.
pub fn reemplazar(
    conn: &Connection,
    venta_id: i64,
    subtotal: f64,
    descuento: f64,
    total_neto: f64,
    monto_pagado: f64,
    saldo_pendiente: f64,
    metodo_pago: &str,
    estado_pago: &str,
    cliente_id: &Option<i64>,
    referencia_cliente: &Option<String>,
) -> SqlResult<usize> {
    conn.execute("DELETE FROM items_venta WHERE venta_id = ?1", [venta_id])?;
    conn.execute(
        "UPDATE ventas SET subtotal=?1, descuento=?2, total_neto=?3, monto_pagado=?4, saldo_pendiente=?5,
                metodo_pago=?6, estado_pago=?7, cliente_id=?8, referencia_cliente=?9 WHERE id=?10",
        (
            subtotal,
            descuento,
            total_neto,
            monto_pagado,
            saldo_pendiente,
            metodo_pago,
            estado_pago,
            cliente_id,
            referencia_cliente,
            venta_id,
        ),
    )
}

pub fn eliminar(conn: &Connection, venta_id: i64) -> SqlResult<usize> {
    conn.execute("DELETE FROM ventas WHERE id = ?1", [venta_id])
}

// ===== Abonos a ventas a crédito =====

pub fn insertar_abono(
    conn: &Connection,
    venta_id: i64,
    monto: f64,
    metodo_pago: &str,
    numero_referencia: &Option<String>,
    nota: &Option<String>,
) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO abonos_venta (venta_id, monto, metodo_pago, numero_referencia, nota, fecha)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now','localtime'))",
        (venta_id, monto, metodo_pago, numero_referencia, nota),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_abonos(conn: &Connection, venta_id: i64) -> SqlResult<Vec<AbonoVenta>> {
    let mut stmt = conn.prepare(
        "SELECT id, venta_id, monto, fecha, metodo_pago, numero_referencia, nota
         FROM abonos_venta WHERE venta_id = ?1 ORDER BY fecha DESC, id DESC",
    )?;
    let filas: Vec<AbonoVenta> = stmt
        .query_map([venta_id], |fila| {
            Ok(AbonoVenta {
                id: fila.get(0)?,
                venta_id: fila.get(1)?,
                monto: fila.get(2)?,
                fecha: fila.get(3)?,
                metodo_pago: fila.get(4)?,
                numero_referencia: fila.get(5)?,
                nota: fila.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

pub fn obtener_saldo(conn: &Connection, venta_id: i64) -> SqlResult<Option<(f64, f64)>> {
    let mut stmt =
        conn.prepare("SELECT saldo_pendiente, total_neto FROM ventas WHERE id = ?1")?;
    let mut filas = stmt.query_map([venta_id], |fila| Ok((fila.get(0)?, fila.get(1)?)))?;
    Ok(filas.next().transpose()?)
}

pub fn aplicar_abono(
    conn: &Connection,
    venta_id: i64,
    nuevo_saldo: f64,
    delta_pagado: f64,
    estado_pago: &str,
) -> SqlResult<usize> {
    conn.execute(
        "UPDATE ventas SET saldo_pendiente = ?1, monto_pagado = monto_pagado + ?2, estado_pago = ?3 WHERE id = ?4",
        (nuevo_saldo, delta_pagado, estado_pago, venta_id),
    )
}

// ===== Ranking =====

/// Top de prendas más vendidas. `dias` filtra por ventana relativa
/// (ej. 1 = hoy, 30 = último mes); None = histórico completo.
pub fn top_prendas(conn: &Connection, dias: Option<i64>, limite: i64) -> SqlResult<Vec<TopPrenda>> {
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

    let mut stmt = conn.prepare(&sql)?;
    let filas: Vec<TopPrenda> = stmt
        .query_map([limite], |fila| {
            Ok(TopPrenda {
                producto_id: fila.get(0)?,
                nombre: fila.get(1)?,
                unidades_vendidas: fila.get(2)?,
                ingreso_total: fila.get(3)?,
                margen_estimado: fila.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}
