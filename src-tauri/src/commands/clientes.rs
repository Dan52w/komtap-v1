//! Comandos de clientes (para ventas a crédito identificadas).

use crate::db::DbConnection;
use crate::models::Cliente;
use crate::repos::clientes as repo;
use tauri::State;

fn validar(cliente: &Cliente) -> Result<(), String> {
    if cliente.nombre.trim().is_empty() {
        return Err("El nombre del cliente es obligatorio".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn crear_cliente(state: State<DbConnection>, cliente: Cliente) -> Result<i64, String> {
    validar(&cliente)?;
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::insertar(&conn, &cliente).map_err(|e| crate::errors::base_datos(e, None))
}

#[tauri::command]
pub fn listar_clientes(state: State<DbConnection>) -> Result<Vec<Cliente>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar(&conn).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn actualizar_cliente(state: State<DbConnection>, cliente: Cliente) -> Result<(), String> {
    let id = cliente
        .id
        .ok_or_else(|| "El cliente no tiene identificador".to_string())?;
    validar(&cliente)?;
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let filas = repo::actualizar(&conn, &cliente, id).map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Cliente no encontrado".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn eliminar_cliente(state: State<DbConnection>, cliente_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    if repo::contar_ventas(&conn, cliente_id).map_err(crate::errors::bd)? > 0 {
        return Err("No se puede eliminar el cliente porque tiene ventas asociadas".to_string());
    }
    let filas = repo::eliminar(&conn, cliente_id).map_err(|e| {
        crate::errors::base_datos(
            e,
            "No se puede eliminar el cliente: tiene registros relacionados.",
        )
    })?;
    if filas == 0 {
        return Err("Cliente no encontrado".to_string());
    }
    Ok(())
}
