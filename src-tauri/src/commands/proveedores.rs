//! Comandos de proveedores, compras (facturas) y abonos a proveedores.

use crate::db::DbConnection;
use crate::models::{
    AbonoInput, AbonoProveedor, Factura, FacturaPendiente, ItemFacturaDetalle, NuevaFactura,
    Proveedor, ResumenProveedor,
};
use crate::repos::proveedores as repo;
use crate::services::compras;
use tauri::State;

#[tauri::command]
pub fn crear_proveedor(state: State<DbConnection>, proveedor: Proveedor) -> Result<i64, String> {
    if proveedor.nombre.trim().is_empty() {
        return Err("El nombre del proveedor es obligatorio".to_string());
    }
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::insertar_proveedor(&conn, &proveedor).map_err(|e| crate::errors::base_datos(e, None))
}

#[tauri::command]
pub fn listar_proveedores(state: State<DbConnection>) -> Result<Vec<Proveedor>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_proveedores(&conn).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn actualizar_proveedor(state: State<DbConnection>, proveedor: Proveedor) -> Result<(), String> {
    let id = proveedor
        .id
        .ok_or_else(|| "El proveedor no tiene identificador".to_string())?;
    if proveedor.nombre.trim().is_empty() {
        return Err("El nombre del proveedor es obligatorio".to_string());
    }
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let filas = repo::actualizar_proveedor(&conn, &proveedor, id).map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Proveedor no encontrado".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn eliminar_proveedor(state: State<DbConnection>, proveedor_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;

    if repo::contar_facturas(&conn, proveedor_id).map_err(crate::errors::bd)? > 0 {
        return Err("No se puede eliminar el proveedor porque tiene facturas/compras asociadas".to_string());
    }

    let filas = repo::eliminar_proveedor(&conn, proveedor_id).map_err(|e| {
        crate::errors::base_datos(
            e,
            Some("No se puede eliminar el proveedor: tiene registros relacionados (facturas u otros datos)."),
        )
    })?;
    if filas == 0 {
        return Err("Proveedor no encontrado".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn crear_factura_proveedor(
    state: State<DbConnection>,
    factura: NuevaFactura,
) -> Result<i64, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let id = compras::crear_factura(&tx, &factura)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(id)
}

#[tauri::command]
pub fn listar_facturas_proveedor(
    state: State<DbConnection>,
    proveedor_id: i64,
) -> Result<Vec<Factura>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_facturas_por_proveedor(&conn, proveedor_id).map_err(crate::errors::bd)
}

/// Todas las facturas con saldo pendiente (cualquier proveedor), con el
/// nombre del proveedor incluido. Reemplaza las N+1 consultas del frontend.
#[tauri::command]
pub fn listar_facturas_pendientes(state: State<DbConnection>) -> Result<Vec<FacturaPendiente>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_facturas_pendientes(&conn).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn listar_abonos_factura_proveedor(
    state: State<DbConnection>,
    factura_id: i64,
) -> Result<Vec<AbonoProveedor>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_abonos_factura(&conn, factura_id).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn listar_items_factura_proveedor(
    state: State<DbConnection>,
    factura_id: i64,
) -> Result<Vec<ItemFacturaDetalle>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_items_factura(&conn, factura_id).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn registrar_abono_proveedor(
    state: State<DbConnection>,
    factura_id: i64,
    abono: AbonoInput,
) -> Result<i64, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let id = compras::registrar_abono(&tx, factura_id, &abono)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(id)
}

#[tauri::command]
pub fn deuda_total_global(state: State<DbConnection>) -> Result<f64, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::deuda_total_global(&conn).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn resumen_por_proveedor(state: State<DbConnection>) -> Result<Vec<ResumenProveedor>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::resumen_por_proveedor(&conn).map_err(crate::errors::bd)
}
