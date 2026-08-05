use crate::db::DbConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

const CATEGORIAS_VALIDAS: [&str; 6] = ["Arriendo", "Servicios", "Nómina", "Mantenimiento", "Varios", "General"];

#[derive(Serialize, Deserialize)]
pub struct Gasto {
    pub id: i64,
    pub descripcion: String,
    pub monto: f64,
    pub fecha: String,
    pub categoria: String,
    pub metodo_pago: String,
}

#[derive(Serialize, Deserialize)]
pub struct NuevoGasto {
    pub descripcion: String,
    pub monto: f64,
    pub fecha: Option<String>,
    pub categoria: Option<String>,
    pub metodo_pago: Option<String>,
}

#[tauri::command]
pub fn crear_gasto(state: State<DbConnection>, gasto: NuevoGasto) -> Result<i64, String> {
    if gasto.descripcion.trim().is_empty() {
        return Err("La descripción del gasto es obligatoria".to_string());
    }
    if gasto.monto <= 0.0 {
        return Err("El monto del gasto debe ser mayor que cero".to_string());
    }

    let categoria = gasto.categoria.unwrap_or_else(|| "General".to_string());
    if !CATEGORIAS_VALIDAS.contains(&categoria.as_str()) {
        return Err("Categoría de gasto no válida".to_string());
    }
    let metodo_pago = gasto.metodo_pago.unwrap_or_else(|| "Efectivo".to_string());

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    match gasto.fecha {
        Some(fecha) => {
            conn.execute(
                "INSERT INTO otros_gastos (descripcion, monto, fecha, categoria, metodo_pago)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                (&gasto.descripcion, gasto.monto, &fecha, &categoria, &metodo_pago),
            ).map_err(|e| e.to_string())?;
        }
        None => {
            conn.execute(
                "INSERT INTO otros_gastos (descripcion, monto, fecha, categoria, metodo_pago)
                 VALUES (?1, ?2, date('now'), ?3, ?4)",
                (&gasto.descripcion, gasto.monto, &categoria, &metodo_pago),
            ).map_err(|e| e.to_string())?;
        }
    }

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn listar_gastos(state: State<DbConnection>, mes: String, limite: Option<i64>, offset: Option<i64>) -> Result<Vec<Gasto>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let limite = limite.unwrap_or(50).clamp(1, 200);
    let offset = offset.unwrap_or(0).max(0);
    let mut stmt = conn.prepare(
        "SELECT id, descripcion, monto, fecha, categoria, metodo_pago
         FROM otros_gastos WHERE substr(fecha, 1, 7) = ?1
         ORDER BY fecha DESC, id DESC LIMIT ?2 OFFSET ?3"
    ).map_err(|e| e.to_string())?;

    let gastos = stmt.query_map((mes, limite, offset), |row| {
        Ok(Gasto {
            id: row.get(0)?,
            descripcion: row.get(1)?,
            monto: row.get(2)?,
            fecha: row.get(3)?,
            categoria: row.get(4)?,
            metodo_pago: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(gastos)
}

#[tauri::command]
pub fn eliminar_gasto(state: State<DbConnection>, gasto_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM otros_gastos WHERE id = ?1", [gasto_id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
