//! Persistencia de productos (catálogo e inventario).
//!
//! El control de stock por ventas/compras ocurre vía triggers SQL
//! (`trg_descontar_stock_venta`, `trg_aumentar_stock_compra`,
//! `trg_restaurar_stock_cancelacion_venta`); aquí solo se lee y escribe
//! el estado resultante, y se auditan los ajustes manuales.

use crate::models::{FiltroProductos, Producto};
use rusqlite::{Connection, Result as SqlResult, Row};

const COLUMNAS: &str = "id, sku, nombre, categoria, talla, color, costo_unitario, precio_venta, stock_actual, stock_minimo, estado";

fn mapear(fila: &Row) -> SqlResult<Producto> {
    Ok(Producto {
        id: Some(fila.get(0)?),
        sku: fila.get(1)?,
        nombre: fila.get(2)?,
        categoria: fila.get(3)?,
        talla: fila.get(4)?,
        color: fila.get(5)?,
        costo_unitario: fila.get(6)?,
        precio_venta: fila.get(7)?,
        stock_actual: fila.get(8)?,
        stock_minimo: fila.get(9)?,
        estado: fila.get(10)?,
    })
}

pub fn insertar(conn: &Connection, p: &Producto) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO productos (sku, nombre, categoria, talla, color, costo_unitario, precio_venta, stock_actual, stock_minimo, estado)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &p.sku, &p.nombre, &p.categoria, &p.talla, &p.color,
            p.costo_unitario, p.precio_venta, p.stock_actual, p.stock_minimo, &p.estado,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn listar(conn: &Connection, filtro: &FiltroProductos) -> SqlResult<Vec<Producto>> {
    let condicion_stock = match filtro.estado_stock.as_deref() {
        Some("agotado") => "AND stock_actual = 0",
        Some("bajo") => "AND stock_actual > 0 AND stock_actual <= stock_minimo",
        Some("disponible") => "AND stock_actual > stock_minimo",
        _ => "",
    };

    let sql = format!(
        "SELECT {COLUMNAS} FROM productos
         WHERE (?1 IS NULL OR categoria = ?1)
           AND (?2 IS NULL OR talla = ?2)
           AND (?3 IS NULL OR nombre LIKE '%' || ?3 || '%')
           AND (?4 IS NULL OR sku LIKE '%' || ?4 || '%')
           {condicion_stock}
         ORDER BY nombre"
    );

    let mut stmt = conn.prepare(&sql)?;
    let productos = stmt
        .query_map(
            (&filtro.categoria, &filtro.talla, &filtro.nombre, &filtro.sku),
            mapear,
        )?
        .filter_map(|r| r.ok())
        .collect();
    Ok(productos)
}

pub fn obtener(conn: &Connection, id: i64) -> SqlResult<Option<Producto>> {
    let mut stmt = conn.prepare(&format!("SELECT {COLUMNAS} FROM productos WHERE id = ?1"))?;
    let mut filas = stmt.query_map([id], mapear)?;
    Ok(filas.next().transpose()?)
}

pub fn obtener_stock(conn: &Connection, id: i64) -> SqlResult<Option<i64>> {
    let mut stmt = conn.prepare("SELECT stock_actual FROM productos WHERE id = ?1")?;
    let mut filas = stmt.query_map([id], |fila| fila.get(0))?;
    Ok(filas.next().transpose()?)
}

pub fn obtener_costo(conn: &Connection, id: i64) -> SqlResult<Option<f64>> {
    let mut stmt = conn.prepare("SELECT costo_unitario FROM productos WHERE id = ?1")?;
    let mut filas = stmt.query_map([id], |fila| fila.get(0))?;
    Ok(filas.next().transpose()?)
}

pub fn actualizar(conn: &Connection, p: &Producto, id: i64) -> SqlResult<usize> {
    conn.execute(
        "UPDATE productos SET sku=?1, nombre=?2, categoria=?3, talla=?4, color=?5,
                costo_unitario=?6, precio_venta=?7, stock_minimo=?8, estado=?9, updated_at=CURRENT_TIMESTAMP
         WHERE id=?10",
        (
            &p.sku, &p.nombre, &p.categoria, &p.talla, &p.color,
            p.costo_unitario, p.precio_venta, p.stock_minimo, &p.estado, id,
        ),
    )
}

/// Fija el stock a un valor absoluto (el cálculo del nuevo valor y su
/// validación viven en el servicio; aquí solo se persiste).
pub fn fijar_stock(conn: &Connection, id: i64, nuevo_stock: i64) -> SqlResult<usize> {
    conn.execute(
        "UPDATE productos SET stock_actual = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        (nuevo_stock, id),
    )
}

/// Registra un ajuste manual en la tabla de auditoría `ajustes_stock`.
pub fn auditar_ajuste(
    conn: &Connection,
    producto_id: i64,
    delta: i64,
    stock_anterior: i64,
    stock_nuevo: i64,
    motivo: &str,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO ajustes_stock (producto_id, delta, stock_anterior, stock_nuevo, motivo)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (producto_id, delta, stock_anterior, stock_nuevo, motivo),
    )?;
    Ok(())
}

/// Devuelve cuántas ventas y cuántas compras usan el producto (para decidir
/// si se puede eliminar o solo inactivar).
pub fn contar_usos(conn: &Connection, id: i64) -> SqlResult<(i64, i64)> {
    let ventas: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items_venta WHERE producto_id = ?1",
        [id],
        |fila| fila.get(0),
    )?;
    let compras: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items_factura_proveedor WHERE producto_id = ?1",
        [id],
        |fila| fila.get(0),
    )?;
    Ok((ventas, compras))
}

pub fn eliminar(conn: &Connection, id: i64) -> SqlResult<usize> {
    conn.execute("DELETE FROM productos WHERE id = ?1", [id])
}
