//! Comandos de licencia y autenticación.
//!
//! Adaptadores delgados: abren la conexión (y la transacción cuando la
//! operación escribe en varias tablas) y delegan toda la lógica a
//! `crate::services`.

use crate::db::DbConnection;
use crate::models::{EstadoLicencia, Usuario};
use crate::services::{licencia, usuarios};
use tauri::State;

#[tauri::command]
pub fn estado_licencia(state: State<DbConnection>) -> Result<EstadoLicencia, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    licencia::estado(&conn)
}

#[tauri::command]
pub fn activar_licencia(state: State<DbConnection>, codigo: String) -> Result<EstadoLicencia, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let estado = licencia::activar(&tx, &codigo)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(estado)
}

#[tauri::command]
pub fn existe_usuario_registrado(state: State<DbConnection>) -> Result<bool, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    usuarios::existe_registrado(&conn)
}

#[tauri::command]
pub fn registrar_usuario(
    state: State<DbConnection>,
    username: String,
    password: String,
    codigo_admin: String,
    nombre_completo: Option<String>,
) -> Result<Usuario, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let usuario = usuarios::registrar(&tx, &username, &password, &codigo_admin, nombre_completo)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(usuario)
}

#[tauri::command]
pub fn iniciar_sesion(
    state: State<DbConnection>,
    username: String,
    password: String,
) -> Result<Usuario, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    usuarios::iniciar_sesion(&conn, &username, &password)
}
