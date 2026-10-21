//! Consultas agregadas para el dashboard financiero y los reportes.
//!
//! Los patrones de fecha se construyen con valores de una lista cerrada
//! ("dia" | "mes" | "anio"), nunca con entrada libre del usuario.

use crate::models::{DashboardFinanciero, GastoOperativoReporte, ProductoCritico, VentasPorMetodoPago};
use rusqlite::{Connection, Result as SqlResult};

/// "dia" | "mes" | "anio" → patrón strftime. Rechaza cualquier otro valor.
fn patron_para_periodo(periodo: &str) -> SqlResult<&'static str> {
    match periodo {
        "dia" => Ok("%Y-%m-%d"),
        "mes" => Ok("%Y-%m"),
        "anio" => Ok("%Y"),
        _ => Err(rusqlite::Error::InvalidParameterName(
            "Periodo no válido (use 'dia', 'mes' o 'anio')".to_string(),
        )),
    }
}

fn suma(conn: &Connection, sql: &str) -> SqlResult<f64> {
    conn.query_row(sql, [], |fila| fila.get(0))
}

fn suma_mes(conn: &Connection, sql: &str, mes: &str) -> SqlResult<f64> {
    conn.query_row(sql, [mes], |fila| fila.get(0))
}

/// Agregados del periodo relativo actual ("dia" | "mes" | "anio").
pub fn dashboard_periodo(conn: &Connection, periodo: &str) -> SqlResult<DashboardFinanciero> {
    let patron = patron_para_periodo(periodo)?;
    let en_periodo = |tabla: &str, columna_fecha: &str, columna: &str| {
        format!(
            "SELECT COALESCE(SUM({columna}), 0) FROM {tabla} \
             WHERE strftime('{patron}', {columna_fecha}) = strftime('{patron}', 'now', 'localtime')"
        )
    };

    let dinero_ingresado = suma(conn, &en_periodo("ventas", "fecha", "monto_pagado"))?;
    let abonos_a_proveedor = suma(conn, &en_periodo("abonos_proveedor", "fecha", "monto"))?;
    let gastos_operativos = suma(conn, &en_periodo("otros_gastos", "fecha", "monto"))?;
    let ventas_netas = suma(conn, &en_periodo("ventas", "fecha", "total_neto"))?;
    let costo_vendido = suma(
        conn,
        &format!(
            "SELECT COALESCE(SUM(iv.costo_unitario * iv.cantidad), 0)
             FROM items_venta iv JOIN ventas v ON v.id = iv.venta_id
             WHERE strftime('{patron}', v.fecha) = strftime('{patron}', 'now', 'localtime')"
        ),
    )?;
    let valor_inventario = suma(
        conn,
        "SELECT COALESCE(SUM(stock_actual * costo_unitario), 0) FROM productos",
    )?;

    Ok(armar_dashboard(
        dinero_ingresado,
        abonos_a_proveedor,
        gastos_operativos,
        ventas_netas,
        costo_vendido,
        valor_inventario,
    ))
}

/// Agregados de un mes calendario concreto ("YYYY-MM").
pub fn dashboard_mes(conn: &Connection, mes: &str) -> SqlResult<DashboardFinanciero> {
    let dinero_ingresado = suma_mes(
        conn,
        "SELECT COALESCE(SUM(monto_pagado), 0) FROM ventas WHERE substr(fecha, 1, 7) = ?1",
        mes,
    )?;
    let abonos_a_proveedor = suma_mes(
        conn,
        "SELECT COALESCE(SUM(monto), 0) FROM abonos_proveedor WHERE substr(fecha, 1, 7) = ?1",
        mes,
    )?;
    let gastos_operativos = suma_mes(
        conn,
        "SELECT COALESCE(SUM(monto), 0) FROM otros_gastos WHERE substr(fecha, 1, 7) = ?1",
        mes,
    )?;
    let ventas_netas = suma_mes(
        conn,
        "SELECT COALESCE(SUM(total_neto), 0) FROM ventas WHERE substr(fecha, 1, 7) = ?1",
        mes,
    )?;
    let costo_vendido = suma_mes(
        conn,
        "SELECT COALESCE(SUM(iv.costo_unitario * iv.cantidad), 0)
         FROM items_venta iv JOIN ventas v ON v.id = iv.venta_id
         WHERE substr(v.fecha, 1, 7) = ?1",
        mes,
    )?;
    let valor_inventario = suma(
        conn,
        "SELECT COALESCE(SUM(stock_actual * costo_unitario), 0) FROM productos",
    )?;

    Ok(armar_dashboard(
        dinero_ingresado,
        abonos_a_proveedor,
        gastos_operativos,
        ventas_netas,
        costo_vendido,
        valor_inventario,
    ))
}

fn armar_dashboard(
    dinero_ingresado: f64,
    abonos_a_proveedor: f64,
    gastos_operativos: f64,
    ventas_netas: f64,
    costo_vendido: f64,
    valor_inventario: f64,
) -> DashboardFinanciero {
    let ganancia_bruta = ventas_netas - costo_vendido;
    DashboardFinanciero {
        dinero_ingresado,
        dinero_gastado: abonos_a_proveedor + gastos_operativos,
        ganancia_bruta,
        ganancia_neta_real: ganancia_bruta - gastos_operativos,
        valor_inventario,
    }
}

pub fn ventas_por_metodo_pago(conn: &Connection, mes: &str) -> SqlResult<Vec<VentasPorMetodoPago>> {
    let mut stmt = conn.prepare(
        "SELECT metodo_pago, COALESCE(SUM(total_neto), 0)
         FROM ventas WHERE substr(fecha, 1, 7) = ?1
         GROUP BY metodo_pago ORDER BY 2 DESC",
    )?;
    Ok(stmt
        .query_map([mes], |fila| {
            Ok(VentasPorMetodoPago {
                metodo_pago: fila.get(0)?,
                total: fila.get(1)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect())
}

pub fn productos_stock_critico(conn: &Connection) -> SqlResult<Vec<ProductoCritico>> {
    let mut stmt = conn.prepare(
        "SELECT id, nombre, talla, stock_actual, stock_minimo
         FROM productos WHERE stock_actual <= stock_minimo AND estado = 'activo'
         ORDER BY stock_actual ASC",
    )?;
    Ok(stmt
        .query_map([], |fila| {
            let stock_actual: i64 = fila.get(3)?;
            Ok(ProductoCritico {
                id: fila.get(0)?,
                nombre: fila.get(1)?,
                talla: fila.get(2)?,
                stock_actual,
                stock_minimo: fila.get(4)?,
                estado_stock: if stock_actual == 0 {
                    "Agotado".to_string()
                } else {
                    "Stock Bajo".to_string()
                },
            })
        })?
        .filter_map(|r| r.ok())
        .collect())
}

/// Datos crudos para el reporte PDF mensual. La generación del PDF en sí
/// se hace en el frontend / capa de exportación con estos datos.
pub fn desglose_gastos_mes(conn: &Connection, mes: &str) -> SqlResult<Vec<GastoOperativoReporte>> {
    let mut stmt = conn.prepare(
        "SELECT descripcion, monto, fecha, categoria FROM otros_gastos
         WHERE substr(fecha, 1, 7) = ?1 ORDER BY fecha DESC",
    )?;
    Ok(stmt
        .query_map([mes], |fila| {
            Ok(GastoOperativoReporte {
                descripcion: fila.get(0)?,
                monto: fila.get(1)?,
                fecha: fila.get(2)?,
                categoria: fila.get(3)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect())
}
