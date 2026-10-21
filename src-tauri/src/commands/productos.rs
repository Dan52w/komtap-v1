//! Comandos de inventario / productos.

use crate::db::DbConnection;
use crate::models::{FiltroProductos, Producto};
use crate::repos::productos as repo;
use crate::services::inventario;
use tauri::State;

#[tauri::command]
pub fn crear_producto(state: State<DbConnection>, producto: Producto) -> Result<i64, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    inventario::crear(&conn, &producto)
}

#[tauri::command]
pub fn listar_productos(
    state: State<DbConnection>,
    filtro: Option<FiltroProductos>,
) -> Result<Vec<Producto>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar(&conn, &filtro.unwrap_or_default()).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn actualizar_producto(state: State<DbConnection>, producto: Producto) -> Result<(), String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    inventario::actualizar(&conn, &producto)
}

#[tauri::command]
pub fn ajustar_stock_manual(
    state: State<DbConnection>,
    producto_id: i64,
    delta: i64,
    motivo: String,
) -> Result<i64, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let nuevo_stock = inventario::ajustar_stock(&tx, producto_id, delta, &motivo)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(nuevo_stock)
}

#[tauri::command]
pub fn eliminar_producto(state: State<DbConnection>, producto_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    inventario::eliminar(&conn, producto_id)
}
