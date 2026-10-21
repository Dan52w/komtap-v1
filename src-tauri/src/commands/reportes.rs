//! Comandos de reportes y dashboard financiero.

use crate::db::DbConnection;
use crate::models::{DashboardFinanciero, GastoOperativoReporte, ProductoCritico, VentasPorMetodoPago};
use crate::repos::reportes as repo;
use crate::services::reglas;
use tauri::State;

#[tauri::command]
pub fn dashboard_financiero(
    state: State<DbConnection>,
    periodo: String,
) -> Result<DashboardFinanciero, String> {
    if !["dia", "mes", "anio"].contains(&periodo.as_str()) {
        return Err("Periodo no válido (use 'dia', 'mes' o 'anio')".to_string());
    }
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::dashboard_periodo(&conn, &periodo).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn dashboard_financiero_mes(
    state: State<DbConnection>,
    mes: String,
) -> Result<DashboardFinanciero, String> {
    reglas::validar_mes(&mes)?;
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::dashboard_mes(&conn, &mes).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn ventas_por_metodo_pago(
    state: State<DbConnection>,
    mes: String,
) -> Result<Vec<VentasPorMetodoPago>, String> {
    reglas::validar_mes(&mes)?;
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::ventas_por_metodo_pago(&conn, &mes).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn productos_stock_critico(state: State<DbConnection>) -> Result<Vec<ProductoCritico>, String> {
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::productos_stock_critico(&conn).map_err(crate::errors::bd)
}

#[tauri::command]
pub fn desglose_gastos_mes(
    state: State<DbConnection>,
    mes: String,
) -> Result<Vec<GastoOperativoReporte>, String> {
    reglas::validar_mes(&mes)?;
    let conn = state.0.lock().map_err(crate::errors::bloqueo)?;
    repo::desglose_gastos_mes(&conn, &mes).map_err(crate::errors::bd)
}
