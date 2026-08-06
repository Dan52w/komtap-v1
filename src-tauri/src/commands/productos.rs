use crate::db::DbConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Producto {
    pub id: Option<i64>,
    pub sku: Option<String>,
    pub nombre: String,
    pub categoria: String,
    pub talla: String,
    pub color: Option<String>,
    pub costo_unitario: f64,
    pub precio_venta: f64,
    pub stock_actual: i64,
    pub stock_minimo: i64,
    pub estado: String,
}

#[derive(Serialize, Deserialize)]
pub struct FiltroProductos {
    pub categoria: Option<String>,
    pub talla: Option<String>,
    pub nombre: Option<String>,
    /// Código de barra / SKU. Coincidencia parcial para permitir escaneo progresivo.
    pub sku: Option<String>,
    /// "agotado" | "bajo" | "disponible" | None = todos
    pub estado_stock: Option<String>,
}

fn mapear_producto(row: &rusqlite::Row) -> rusqlite::Result<Producto> {
    Ok(Producto {
        id: Some(row.get(0)?),
        sku: row.get(1)?,
        nombre: row.get(2)?,
        categoria: row.get(3)?,
        talla: row.get(4)?,
        color: row.get(5)?,
        costo_unitario: row.get(6)?,
        precio_venta: row.get(7)?,
        stock_actual: row.get(8)?,
        stock_minimo: row.get(9)?,
        estado: row.get(10)?,
    })
}

const COLUMNAS: &str = "id, sku, nombre, categoria, talla, color, costo_unitario, precio_venta, stock_actual, stock_minimo, estado";

#[tauri::command]
pub fn crear_producto(state: State<DbConnection>, producto: Producto) -> Result<i64, String> {
    if producto.nombre.trim().is_empty() {
        return Err("El nombre de la prenda es obligatorio".to_string());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO productos (sku, nombre, categoria, talla, color, costo_unitario, precio_venta, stock_actual, stock_minimo, estado)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &producto.sku, &producto.nombre, &producto.categoria, &producto.talla, &producto.color,
            producto.costo_unitario, producto.precio_venta, producto.stock_actual, producto.stock_minimo, &producto.estado,
        ),
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn listar_productos(state: State<DbConnection>, filtro: Option<FiltroProductos>) -> Result<Vec<Producto>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let filtro = filtro.unwrap_or(FiltroProductos {
        categoria: None,
        talla: None,
        nombre: None,
        sku: None,
        estado_stock: None,
    });

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

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let productos = stmt
        .query_map((&filtro.categoria, &filtro.talla, &filtro.nombre, &filtro.sku), mapear_producto)
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(productos)
}

#[tauri::command]
pub fn actualizar_producto(state: State<DbConnection>, producto: Producto) -> Result<(), String> {
    let id = producto.id.ok_or("El producto no tiene identificador")?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE productos SET sku=?1, nombre=?2, categoria=?3, talla=?4, color=?5,
                costo_unitario=?6, precio_venta=?7, stock_minimo=?8, estado=?9, updated_at=CURRENT_TIMESTAMP
         WHERE id=?10",
        (
            &producto.sku, &producto.nombre, &producto.categoria, &producto.talla, &producto.color,
            producto.costo_unitario, producto.precio_venta, producto.stock_minimo, &producto.estado, id,
        ),
    ).map_err(|e| e.to_string())?;
    Ok(())
}

/// Ajuste manual de stock (pérdidas, defectos, correcciones internas).
/// `delta` puede ser positivo o negativo; se valida que el resultado no sea negativo.
#[tauri::command]
pub fn ajustar_stock_manual(state: State<DbConnection>, producto_id: i64, delta: i64, motivo: String) -> Result<i64, String> {
    if motivo.trim().is_empty() {
        return Err("Debe indicar un motivo para el ajuste de stock".to_string());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let stock_actual: i64 = conn.query_row(
        "SELECT stock_actual FROM productos WHERE id = ?1", [producto_id], |row| row.get(0),
    ).map_err(|_| "Producto no encontrado".to_string())?;

    let nuevo_stock = stock_actual + delta;
    if nuevo_stock < 0 {
        return Err("El ajuste dejaría el stock en un valor negativo".to_string());
    }

    conn.execute(
        "UPDATE productos SET stock_actual = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        (nuevo_stock, producto_id),
    ).map_err(|e| e.to_string())?;

    Ok(nuevo_stock)
}

#[tauri::command]
pub fn eliminar_producto(state: State<DbConnection>, producto_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let ventas_asociadas: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items_venta WHERE producto_id = ?1", [producto_id], |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    if ventas_asociadas > 0 {
        return Err("No se puede eliminar: el producto tiene ventas asociadas. Márquelo como inactivo.".to_string());
    }

    let compras_asociadas: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items_factura_proveedor WHERE producto_id = ?1", [producto_id], |row| row.get(0),
    ).map_err(|e| e.to_string())?;
    if compras_asociadas > 0 {
        return Err("No se puede eliminar: el producto tiene compras registradas a proveedores. Márquelo como inactivo.".to_string());
    }

    conn.execute("DELETE FROM productos WHERE id = ?1", [producto_id]).map_err(|e| {
        crate::db::friendly_delete_error(
            e,
            "No se puede eliminar: el producto todavía tiene registros relacionados. Márquelo como inactivo.",
        )
    })?;
    Ok(())
}

