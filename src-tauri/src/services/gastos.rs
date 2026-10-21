//! Reglas de negocio de gastos operativos.

use crate::models::{GastoActualizado, NuevoGasto};
use crate::repos::gastos;
use crate::services::reglas::{validar_categoria_gasto, validar_mes, validar_metodo_pago_venta};
use rusqlite::Connection;

pub fn crear(conn: &Connection, gasto: &NuevoGasto) -> Result<i64, String> {
    let descripcion = gasto.descripcion.trim();
    if descripcion.is_empty() {
        return Err("La descripción del gasto es obligatoria".to_string());
    }
    if gasto.monto <= 0.0 {
        return Err("El monto del gasto debe ser mayor que cero".to_string());
    }

    let categoria = gasto.categoria.clone().unwrap_or_else(|| "General".to_string());
    validar_categoria_gasto(&categoria)?;
    let metodo_pago = gasto.metodo_pago.clone().unwrap_or_else(|| "Efectivo".to_string());
    validar_metodo_pago_venta(&metodo_pago)?;

    gastos::insertar(conn, gasto, &categoria, &metodo_pago).map_err(crate::errors::bd)
}

pub fn listar(conn: &Connection, mes: &str, limite: i64, offset: i64) -> Result<Vec<crate::models::Gasto>, String> {
    validar_mes(mes)?;
    let limite = limite.clamp(1, 200);
    let offset = offset.max(0);
    gastos::listar(conn, mes, limite, offset).map_err(crate::errors::bd)
}

pub fn actualizar(conn: &Connection, id: i64, gasto: &GastoActualizado) -> Result<(), String> {
    if gasto.descripcion.trim().is_empty() {
        return Err("La descripción del gasto es obligatoria".to_string());
    }
    if gasto.monto <= 0.0 {
        return Err("El monto del gasto debe ser mayor que cero".to_string());
    }
    validar_categoria_gasto(&gasto.categoria)?;
    validar_metodo_pago_venta(&gasto.metodo_pago)?;

    let filas = gastos::actualizar(conn, id, gasto).map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Gasto no encontrado".to_string());
    }
    Ok(())
}

pub fn eliminar(conn: &Connection, id: i64) -> Result<(), String> {
    let filas = gastos::eliminar(conn, id).map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Gasto no encontrado".to_string());
    }
    Ok(())
}
