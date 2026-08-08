use crate::db::DbConnection;
use bcrypt::{hash, verify, DEFAULT_COST};
use tauri::State;

#[tauri::command]
pub fn existe_codigo_admin(state: State<DbConnection>) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM configuracion_seguridad WHERE id = 1)",
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn establecer_codigo_admin(state: State<DbConnection>, codigo: String) -> Result<(), String> {
    if codigo.trim().len() < 4 {
        return Err("El código debe tener al menos 4 caracteres".to_string());
    }
    let hash_codigo = hash(codigo.trim(), DEFAULT_COST).map_err(|e| e.to_string())?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO configuracion_seguridad (id, codigo_admin_hash) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET codigo_admin_hash = excluded.codigo_admin_hash",
        [&hash_codigo],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn verificar_codigo_admin(state: State<DbConnection>, codigo: String) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let hash_guardado: Option<String> = conn.query_row(
        "SELECT codigo_admin_hash FROM configuracion_seguridad WHERE id = 1",
        [], |row| row.get(0),
    ).ok();
    match hash_guardado {
        Some(h) => verify(codigo.trim(), &h).map_err(|e| e.to_string()),
        None => Ok(false),
    }
}