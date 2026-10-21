//! Comandos del código de seguridad del administrador.

use crate::db::DbConnection;
use crate::repos::seguridad as repo;
use bcrypt::{hash, verify, DEFAULT_COST};
use tauri::State;

#[tauri::command]
pub fn existe_codigo_admin(state: State<DbConnection>) -> Result<bool, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::existe_codigo(&conn).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn establecer_codigo_admin(state: State<DbConnection>, codigo: String) -> Result<(), String> {
    if codigo.trim().len() < 4 {
        return Err("El código debe tener al menos 4 caracteres".to_string());
    }
    let hash_codigo = hash(codigo.trim(), DEFAULT_COST).map_err(|e| e.to_string())?;
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::guardar_hash(&conn, &hash_codigo).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn verificar_codigo_admin(state: State<DbConnection>, codigo: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    match repo::obtener_hash(&conn).map_err(crate::errors::bd)? {
        Some(h) => verify(codigo.trim(), &h).map_err(|e| e.to_string()),
        None => Ok(false),
    }
}
