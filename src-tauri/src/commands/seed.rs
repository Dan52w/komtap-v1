//! Comando de la semilla de datos de ejemplo.

use crate::db::DbConnection;
use crate::models::ResumenSeed;
use crate::services::seed as servicio;
use tauri::State;

#[tauri::command]
pub fn cargar_datos_ejemplo(state: State<DbConnection>) -> Result<ResumenSeed, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let resumen = servicio::cargar_datos_ejemplo(&tx)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(resumen)
}
