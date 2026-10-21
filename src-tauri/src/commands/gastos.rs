//! Comandos de gastos operativos.

use crate::db::DbConnection;
use crate::models::{Gasto, GastoActualizado, NuevoGasto};
use crate::services::gastos as servicio;
use tauri::State;

#[tauri::command]
pub fn crear_gasto(state: State<DbConnection>, gasto: NuevoGasto) -> Result<i64, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    servicio::crear(&conn, &gasto)
}

#[tauri::command]
pub fn listar_gastos(
    state: State<DbConnection>,
    mes: String,
    limite: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Gasto>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    servicio::listar(
        &conn,
        &mes,
        limite.unwrap_or(50),
        offset.unwrap_or(0),
    )
}

#[tauri::command]
pub fn actualizar_gasto(
    state: State<DbConnection>,
    gasto_id: i64,
    gasto: GastoActualizado,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    servicio::actualizar(&conn, gasto_id, &gasto)
}

#[tauri::command]
pub fn eliminar_gasto(state: State<DbConnection>, gasto_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    servicio::eliminar(&conn, gasto_id)
}
