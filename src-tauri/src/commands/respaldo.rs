//! Comandos de respaldo y restauración de la base de datos.
//!
//! El frontend pide la ruta con el diálogo nativo (`@tauri-apps/plugin-dialog`)
//! y la pasa aquí. La copia usa la API de backup en línea de SQLite: no hay
//! que cerrar la app ni la conexión.

use crate::db::DbConnection;
use crate::services::respaldo as servicio;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub fn exportar_respaldo(state: State<DbConnection>, ruta: String) -> Result<(), String> {
    let ruta = Path::new(&ruta);
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    servicio::exportar(&conn, ruta)
}

#[tauri::command]
pub fn importar_respaldo(state: State<DbConnection>, ruta: String) -> Result<(), String> {
    let ruta = Path::new(&ruta);
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    servicio::importar(&mut conn, ruta)
}
