//! Persistencia de proveedores, facturas de compra y abonos a proveedores.

use crate::models::{
    AbonoInput, AbonoProveedor, Factura, FacturaPendiente, ItemFacturaDetalle, Proveedor,
    ResumenProveedor,
};
use rusqlite::{Connection, Result as SqlResult, Row};

// ===== Proveedores =====

fn mapear_proveedor(fila: &Row) -> SqlResult<Proveedor> {
    Ok(Proveedor {
        id: Some(fila.get(0)?),
        nombre: fila.get(1)?,
        telefono: fila.get(2)?,
        direccion: fila.get(3)?,
        correo: fila.get(4)?,
        notas: fila.get(5)?,
    })
}

pub fn insertar_proveedor(conn: &Connection, p: &Proveedor) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO proveedores (nombre, telefono, direccion, correo, notas) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&p.nombre, &p.telefono, &p.direccion, &p.correo, &p.notas),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_proveedores(conn: &Connection) -> SqlResult<Vec<Proveedor>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, telefono, direccion, correo, notas FROM proveedores ORDER BY nombre",
    )?;
    let filas: Vec<Proveedor> = stmt
        .query_map([], mapear_proveedor)?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

pub fn obtener_proveedor(conn: &Connection, id: i64) -> SqlResult<Option<Proveedor>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, telefono, direccion, correo, notas FROM proveedores WHERE id = ?1",
    )?;
    let mut filas = stmt.query_map([id], mapear_proveedor)?;
    Ok(filas.next().transpose()?)
}

pub fn actualizar_proveedor(conn: &Connection, p: &Proveedor, id: i64) -> SqlResult<usize> {
    conn.execute(
        "UPDATE proveedores SET nombre=?1, telefono=?2, direccion=?3, correo=?4, notas=?5 WHERE id=?6",
        (&p.nombre, &p.telefono, &p.direccion, &p.correo, &p.notas, id),
    )
}

pub fn contar_facturas(conn: &Connection, proveedor_id: i64) -> SqlResult<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM facturas_proveedor WHERE proveedor_id = ?1",
        [proveedor_id],
        |fila| fila.get(0),
    )
}

pub fn eliminar_proveedor(conn: &Connection, id: i64) -> SqlResult<usize> {
    conn.execute("DELETE FROM proveedores WHERE id = ?1", [id])
}

pub fn resumen_por_proveedor(conn: &Connection) -> SqlResult<Vec<ResumenProveedor>> {
    let mut stmt = conn.prepare(
        "SELECT p.id, p.nombre, COALESCE(SUM(f.saldo_pendiente), 0) AS deuda_total
         FROM proveedores p
         LEFT JOIN facturas_proveedor f ON f.proveedor_id = p.id
         GROUP BY p.id
         ORDER BY deuda_total DESC",
    )?;
    let filas: Vec<ResumenProveedor> = stmt
        .query_map([], |fila| {
            Ok(ResumenProveedor {
                proveedor_id: fila.get(0)?,
                nombre: fila.get(1)?,
                deuda_total: fila.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

pub fn deuda_total_global(conn: &Connection) -> SqlResult<f64> {
    conn.query_row(
        "SELECT COALESCE(SUM(saldo_pendiente), 0) FROM facturas_proveedor",
        [],
        |fila| fila.get(0),
    )
}

// ===== Facturas de compra =====

fn mapear_factura(fila: &Row) -> SqlResult<Factura> {
    Ok(Factura {
        id: fila.get(0)?,
        proveedor_id: fila.get(1)?,
        numero_factura: fila.get(2)?,
        fecha_emision: fila.get(3)?,
        fecha_limite_pago: fila.get(4)?,
        monto_total: fila.get(5)?,
        saldo_pendiente: fila.get(6)?,
        estado_pago: fila.get(7)?,
    })
}

pub fn insertar_factura(
    conn: &Connection,
    proveedor_id: i64,
    numero_factura: &Option<String>,
    fecha_emision: &str,
    fecha_limite_pago: &Option<String>,
    monto_total: f64,
    saldo_pendiente: f64,
    estado_pago: &str,
) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO facturas_proveedor (proveedor_id, numero_factura, fecha_emision, fecha_limite_pago, monto_total, saldo_pendiente, estado_pago)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            proveedor_id,
            numero_factura,
            fecha_emision,
            fecha_limite_pago,
            monto_total,
            saldo_pendiente,
            estado_pago,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn insertar_item_factura(
    conn: &Connection,
    factura_id: i64,
    producto_id: i64,
    cantidad: i64,
    costo_unitario: f64,
    precio_venta_sugerido: &Option<f64>,
    subtotal: f64,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO items_factura_proveedor (factura_id, producto_id, cantidad, costo_unitario, precio_venta_sugerido, subtotal)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            factura_id,
            producto_id,
            cantidad,
            costo_unitario,
            precio_venta_sugerido,
            subtotal,
        ),
    )?;
    Ok(())
}

pub fn listar_facturas_por_proveedor(conn: &Connection, proveedor_id: i64) -> SqlResult<Vec<Factura>> {
    let mut stmt = conn.prepare(
        "SELECT id, proveedor_id, numero_factura, fecha_emision, fecha_limite_pago, monto_total, saldo_pendiente, estado_pago
         FROM facturas_proveedor WHERE proveedor_id = ?1 ORDER BY fecha_emision DESC",
    )?;
    let filas: Vec<Factura> = stmt
        .query_map([proveedor_id], mapear_factura)?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

/// Todas las facturas con saldo pendiente, con el nombre del proveedor.
/// Evita el patrón N+1 (una consulta por proveedor) que hacía el frontend.
pub fn listar_facturas_pendientes(conn: &Connection) -> SqlResult<Vec<FacturaPendiente>> {
    let mut stmt = conn.prepare(
        "SELECT f.id, f.proveedor_id, p.nombre, f.numero_factura, f.fecha_emision,
                f.fecha_limite_pago, f.monto_total, f.saldo_pendiente, f.estado_pago
         FROM facturas_proveedor f
         JOIN proveedores p ON p.id = f.proveedor_id
         WHERE f.saldo_pendiente > 0
         ORDER BY f.fecha_limite_pago ASC NULLS LAST, f.fecha_emision DESC",
    )?;
    let filas: Vec<FacturaPendiente> = stmt
        .query_map([], |fila| {
            Ok(FacturaPendiente {
                id: fila.get(0)?,
                proveedor_id: fila.get(1)?,
                proveedor_nombre: fila.get(2)?,
                numero_factura: fila.get(3)?,
                fecha_emision: fila.get(4)?,
                fecha_limite_pago: fila.get(5)?,
                monto_total: fila.get(6)?,
                saldo_pendiente: fila.get(7)?,
                estado_pago: fila.get(8)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

pub fn obtener_saldo_factura(conn: &Connection, factura_id: i64) -> SqlResult<Option<(f64, f64)>> {
    let mut stmt = conn.prepare(
        "SELECT saldo_pendiente, monto_total FROM facturas_proveedor WHERE id = ?1",
    )?;
    let mut filas = stmt.query_map([factura_id], |fila| Ok((fila.get(0)?, fila.get(1)?)))?;
    Ok(filas.next().transpose()?)
}

pub fn actualizar_saldo_factura(
    conn: &Connection,
    factura_id: i64,
    nuevo_saldo: f64,
    estado_pago: &str,
) -> SqlResult<usize> {
    conn.execute(
        "UPDATE facturas_proveedor SET saldo_pendiente = ?1, estado_pago = ?2 WHERE id = ?3",
        (nuevo_saldo, estado_pago, factura_id),
    )
}

pub fn listar_items_factura(conn: &Connection, factura_id: i64) -> SqlResult<Vec<ItemFacturaDetalle>> {
    let mut stmt = conn.prepare(
        "SELECT i.id, i.producto_id, p.nombre, i.cantidad, i.costo_unitario, i.precio_venta_sugerido, i.subtotal
         FROM items_factura_proveedor i JOIN productos p ON p.id = i.producto_id
         WHERE i.factura_id = ?1 ORDER BY i.id",
    )?;
    let filas: Vec<ItemFacturaDetalle> = stmt
        .query_map([factura_id], |fila| {
            Ok(ItemFacturaDetalle {
                id: fila.get(0)?,
                producto_id: fila.get(1)?,
                producto_nombre: fila.get(2)?,
                cantidad: fila.get(3)?,
                costo_unitario: fila.get(4)?,
                precio_venta_sugerido: fila.get(5)?,
                subtotal: fila.get(6)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

// ===== Abonos a proveedores =====

pub fn insertar_abono(
    conn: &Connection,
    factura_id: i64,
    abono: &AbonoInput,
) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO abonos_proveedor (factura_id, monto, metodo_pago, banco, numero_referencia, nota, fecha)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, COALESCE(?7, datetime('now','localtime')))",
        (
            factura_id,
            abono.monto,
            &abono.metodo_pago,
            &abono.banco,
            &abono.numero_referencia,
            &abono.nota,
            &abono.fecha,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn listar_abonos_factura(conn: &Connection, factura_id: i64) -> SqlResult<Vec<AbonoProveedor>> {
    let mut stmt = conn.prepare(
        "SELECT id, factura_id, monto, fecha, metodo_pago, banco, numero_referencia, nota
         FROM abonos_proveedor WHERE factura_id = ?1 ORDER BY fecha DESC, id DESC",
    )?;
    let filas: Vec<AbonoProveedor> = stmt
        .query_map([factura_id], |fila| {
            Ok(AbonoProveedor {
                id: fila.get(0)?,
                factura_id: fila.get(1)?,
                monto: fila.get(2)?,
                fecha: fila.get(3)?,
                metodo_pago: fila.get(4)?,
                banco: fila.get(5)?,
                numero_referencia: fila.get(6)?,
                nota: fila.get(7)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}
