//! Persistencia de gastos operativos.

use crate::models::{Gasto, GastoActualizado, NuevoGasto};
use rusqlite::{Connection, Result as SqlResult, Row};

fn mapear(fila: &Row) -> SqlResult<Gasto> {
    Ok(Gasto {
        id: fila.get(0)?,
        descripcion: fila.get(1)?,
        monto: fila.get(2)?,
        fecha: fila.get(3)?,
        categoria: fila.get(4)?,
        metodo_pago: fila.get(5)?,
    })
}

pub fn insertar(conn: &Connection, gasto: &NuevoGasto, categoria: &str, metodo_pago: &str) -> SqlResult<i64> {
    match &gasto.fecha {
        Some(fecha) => {
            conn.execute(
                "INSERT INTO otros_gastos (descripcion, monto, fecha, categoria, metodo_pago)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                (&gasto.descripcion, gasto.monto, fecha, categoria, metodo_pago),
            )?;
        }
        None => {
            // IMPORTANTE: se usa 'localtime' para que la fecha del gasto coincida
            // con la fecha local del sistema (Colombia = UTC-5). Sin este modificador,
            // date('now') devuelve la fecha en UTC, y un gasto registrado entre las
            // 7pm y medianoche hora local quedaba guardado con la fecha del día
            // siguiente, haciendo que no apareciera en el dashboard al filtrar "Hoy".
            conn.execute(
                "INSERT INTO otros_gastos (descripcion, monto, fecha, categoria, metodo_pago)
                 VALUES (?1, ?2, date('now','localtime'), ?3, ?4)",
                (&gasto.descripcion, gasto.monto, categoria, metodo_pago),
            )?;
        }
    }
    Ok(conn.last_insert_rowid())
}

pub fn listar(conn: &Connection, mes: &str, limite: i64, offset: i64) -> SqlResult<Vec<Gasto>> {
    let mut stmt = conn.prepare(
        "SELECT id, descripcion, monto, fecha, categoria, metodo_pago
         FROM otros_gastos WHERE substr(fecha, 1, 7) = ?1
         ORDER BY fecha DESC, id DESC LIMIT ?2 OFFSET ?3",
    )?;
    let filas: Vec<Gasto> = stmt
        .query_map((mes, limite, offset), mapear)?
        .filter_map(|r| r.ok())
        .collect();
    Ok(filas)
}

pub fn actualizar(conn: &Connection, id: i64, gasto: &GastoActualizado) -> SqlResult<usize> {
    conn.execute(
        "UPDATE otros_gastos SET descripcion=?1, monto=?2, fecha=?3, categoria=?4, metodo_pago=?5 WHERE id=?6",
        (
            &gasto.descripcion,
            gasto.monto,
            &gasto.fecha,
            &gasto.categoria,
            &gasto.metodo_pago,
            id,
        ),
    )
}

pub fn eliminar(conn: &Connection, id: i64) -> SqlResult<usize> {
    conn.execute("DELETE FROM otros_gastos WHERE id = ?1", [id])
}
