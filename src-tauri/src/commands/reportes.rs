use crate::db::DbConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

/// "dia" | "mes" | "anio"
fn patron_para_periodo(periodo: &str) -> Result<&'static str, String> {
    match periodo {
        "dia" => Ok("%Y-%m-%d"),
        "mes" => Ok("%Y-%m"),
        "anio" => Ok("%Y"),
        _ => Err("Periodo no válido (use 'dia', 'mes' o 'anio')".to_string()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DashboardFinanciero {
    pub dinero_ingresado: f64,   // Total recibido por ventas ejecutadas (pagos recibidos)
    pub dinero_gastado: f64,     // Abonos a proveedores + gastos operativos
    pub ganancia_bruta: f64,     // Ventas - costo de las prendas vendidas
    pub ganancia_neta_real: f64, // Ganancia bruta - gastos operativos
    pub valor_inventario: f64,   // Stock actual valorado a costo
}

#[tauri::command]
pub fn dashboard_financiero(state: State<DbConnection>, periodo: String) -> Result<DashboardFinanciero, String> {
    let patron = patron_para_periodo(&periodo)?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let dinero_ingresado: f64 = conn.query_row(
        &format!("SELECT COALESCE(SUM(monto_pagado), 0) FROM ventas WHERE strftime('{patron}', fecha) = strftime('{patron}', 'now', 'localtime')"),
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let abonos_a_proveedor: f64 = conn.query_row(
        &format!("SELECT COALESCE(SUM(monto), 0) FROM abonos_proveedor WHERE strftime('{patron}', fecha) = strftime('{patron}', 'now', 'localtime')"),
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let gastos_operativos: f64 = conn.query_row(
        &format!("SELECT COALESCE(SUM(monto), 0) FROM otros_gastos WHERE strftime('{patron}', fecha) = strftime('{patron}', 'now', 'localtime')"),
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let costo_vendido: f64 = conn.query_row(
        &format!(
            "SELECT COALESCE(SUM(iv.costo_unitario * iv.cantidad), 0)
             FROM items_venta iv JOIN ventas v ON v.id = iv.venta_id
             WHERE strftime('{patron}', v.fecha) = strftime('{patron}', 'now', 'localtime')"
        ),
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let ventas_netas: f64 = conn.query_row(
        &format!("SELECT COALESCE(SUM(total_neto), 0) FROM ventas WHERE strftime('{patron}', fecha) = strftime('{patron}', 'now', 'localtime')"),
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let valor_inventario: f64 = conn.query_row(
        "SELECT COALESCE(SUM(stock_actual * costo_unitario), 0) FROM productos",
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let ganancia_bruta = ventas_netas - costo_vendido;
    let ganancia_neta_real = ganancia_bruta - gastos_operativos;

    Ok(DashboardFinanciero {
        dinero_ingresado,
        dinero_gastado: abonos_a_proveedor + gastos_operativos,
        ganancia_bruta,
        ganancia_neta_real,
        valor_inventario,
    })
}

#[tauri::command]
pub fn dashboard_financiero_mes(state: State<DbConnection>, mes: String) -> Result<DashboardFinanciero, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let dinero_ingresado: f64 = conn.query_row(
        "SELECT COALESCE(SUM(monto_pagado), 0) FROM ventas WHERE substr(fecha, 1, 7) = ?1",
        [&mes], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let abonos_a_proveedor: f64 = conn.query_row(
        "SELECT COALESCE(SUM(monto), 0) FROM abonos_proveedor WHERE substr(fecha, 1, 7) = ?1",
        [&mes], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let gastos_operativos: f64 = conn.query_row(
        "SELECT COALESCE(SUM(monto), 0) FROM otros_gastos WHERE substr(fecha, 1, 7) = ?1",
        [&mes], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let costo_vendido: f64 = conn.query_row(
        "SELECT COALESCE(SUM(iv.costo_unitario * iv.cantidad), 0)
         FROM items_venta iv JOIN ventas v ON v.id = iv.venta_id
         WHERE substr(v.fecha, 1, 7) = ?1",
        [&mes], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let ventas_netas: f64 = conn.query_row(
        "SELECT COALESCE(SUM(total_neto), 0) FROM ventas WHERE substr(fecha, 1, 7) = ?1",
        [&mes], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let valor_inventario: f64 = conn.query_row(
        "SELECT COALESCE(SUM(stock_actual * costo_unitario), 0) FROM productos",
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let ganancia_bruta = ventas_netas - costo_vendido;
    let ganancia_neta_real = ganancia_bruta - gastos_operativos;

    Ok(DashboardFinanciero {
        dinero_ingresado,
        dinero_gastado: abonos_a_proveedor + gastos_operativos,
        ganancia_bruta,
        ganancia_neta_real,
        valor_inventario,
    })
}

#[derive(Serialize, Deserialize)]
pub struct VentasPorMetodoPago {
    pub metodo_pago: String,
    pub total: f64,
}

#[tauri::command]
pub fn ventas_por_metodo_pago(state: State<DbConnection>, mes: String) -> Result<Vec<VentasPorMetodoPago>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT metodo_pago, COALESCE(SUM(total_neto), 0)
         FROM ventas WHERE substr(fecha, 1, 7) = ?1
         GROUP BY metodo_pago ORDER BY 2 DESC"
    ).map_err(|e| e.to_string())?;

    let resultado = stmt.query_map([mes], |row| {
        Ok(VentasPorMetodoPago { metodo_pago: row.get(0)?, total: row.get(1)? })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(resultado)
}

#[derive(Serialize, Deserialize)]
pub struct ProductoCritico {
    pub id: i64,
    pub nombre: String,
    pub talla: String,
    pub stock_actual: i64,
    pub stock_minimo: i64,
    pub estado_stock: String, // "Agotado" | "Stock Bajo"
}

#[tauri::command]
pub fn productos_stock_critico(state: State<DbConnection>) -> Result<Vec<ProductoCritico>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, nombre, talla, stock_actual, stock_minimo
         FROM productos WHERE stock_actual <= stock_minimo AND estado = 'activo'
         ORDER BY stock_actual ASC"
    ).map_err(|e| e.to_string())?;

    let resultado = stmt.query_map([], |row| {
        let stock_actual: i64 = row.get(3)?;
        let stock_minimo: i64 = row.get(4)?;
        Ok(ProductoCritico {
            id: row.get(0)?,
            nombre: row.get(1)?,
            talla: row.get(2)?,
            stock_actual,
            stock_minimo,
            estado_stock: if stock_actual == 0 { "Agotado".to_string() } else { "Stock Bajo".to_string() },
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(resultado)
}

#[derive(Serialize, Deserialize)]
pub struct GastoOperativoReporte {
    pub descripcion: String,
    pub monto: f64,
    pub fecha: String,
    pub categoria: String,
}

/// Datos crudos para el reporte PDF mensual (HU-09). La generación del PDF
/// en sí se hace en el frontend / capa de exportación con estos datos.
#[tauri::command]
pub fn desglose_gastos_mes(state: State<DbConnection>, mes: String) -> Result<Vec<GastoOperativoReporte>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT descripcion, monto, fecha, categoria FROM otros_gastos
         WHERE substr(fecha, 1, 7) = ?1 ORDER BY fecha DESC"
    ).map_err(|e| e.to_string())?;

    let resultado = stmt.query_map([mes], |row| {
        Ok(GastoOperativoReporte {
            descripcion: row.get(0)?,
            monto: row.get(1)?,
            fecha: row.get(2)?,
            categoria: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(resultado)
}
