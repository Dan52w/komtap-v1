//! Comandos de ventas (POS), abonos a crédito y ranking de prendas.

use crate::db::DbConnection;
use crate::models::{AbonoVenta, AbonoVentaInput, ItemVenta, NuevaVenta, TopPrenda, Venta, VentaActualizada};
use crate::repos::ventas as repo;
use crate::services::ventas as servicio;
use tauri::State;

#[tauri::command]
pub fn crear_venta(state: State<DbConnection>, venta: NuevaVenta) -> Result<i64, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let id = servicio::crear(&tx, &venta)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(id)
}

#[tauri::command]
pub fn listar_ventas(
    state: State<DbConnection>,
    limite: Option<i64>,
    offset: Option<i64>,
    mes: Option<String>,
) -> Result<Vec<Venta>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let limite = limite.unwrap_or(50).clamp(1, 500);
    let offset = offset.unwrap_or(0).max(0);
    if let Some(ref m) = mes {
        crate::services::reglas::validar_mes(m)?;
    }
    repo::listar(&conn, &mes, limite, offset).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn listar_items_venta(state: State<DbConnection>, venta_id: i64) -> Result<Vec<ItemVenta>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_items(&conn, venta_id).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn actualizar_venta(
    state: State<DbConnection>,
    venta_id: i64,
    venta: VentaActualizada,
) -> Result<(), String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    servicio::actualizar(&tx, venta_id, &venta)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(())
}

#[tauri::command]
pub fn eliminar_venta(state: State<DbConnection>, venta_id: i64) -> Result<(), String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    servicio::eliminar(&tx, venta_id)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(())
}

#[tauri::command]
pub fn registrar_abono_venta(
    state: State<DbConnection>,
    venta_id: i64,
    abono: AbonoVentaInput,
) -> Result<i64, String> {
    let mut conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let tx = conn.transaction().map_err(crate::errors::bd)?;
    let id = servicio::registrar_abono(&tx, venta_id, &abono)?;
    tx.commit().map_err(crate::errors::bd)?;
    Ok(id)
}

#[tauri::command]
pub fn listar_abonos_venta(state: State<DbConnection>, venta_id: i64) -> Result<Vec<AbonoVenta>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::listar_abonos(&conn, venta_id).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn top_prendas_vendidas(
    state: State<DbConnection>,
    dias: Option<i64>,
    limite: Option<i64>,
) -> Result<Vec<TopPrenda>, String> {
    if let Some(d) = dias {
        if d < 0 || d > 3650 {
            return Err("El rango de días no es válido".to_string());
        }
    }
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    let limite = limite.unwrap_or(10).clamp(1, 100);
    repo::top_prendas(&conn, dias, limite).map_err(crate::errors::bd)
}
