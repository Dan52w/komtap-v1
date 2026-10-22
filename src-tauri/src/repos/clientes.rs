//! Persistencia de clientes (para ventas a crédito identificadas).

use crate::models::Cliente;
use rusqlite::{Connection, Result as SqlResult, Row};

fn mapear(fila: &Row) -> SqlResult<Cliente> {
    Ok(Cliente {
        id: Some(fila.get(0)?),
        nombre: fila.get(1)?,
        telefono: fila.get(2)?,
        notas: fila.get(3)?,
    })
}

pub fn insertar(conn: &Connection, c: &Cliente) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO clientes (nombre, telefono, notas) VALUES (?1, ?2, ?3)",
        (&c.nombre, &c.telefono, &c.notas),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn listar(conn: &Connection) -> SqlResult<Vec<Cliente>> {
    let mut stmt = conn.prepare("SELECT id, nombre, telefono, notas FROM clientes ORDER BY nombre")?;
    let filas: Vec<Cliente> = stmt.query_map([], mapear)?.filter_map(|r| r.ok()).collect();
    Ok(filas)
}

pub fn obtener(conn: &Connection, id: i64) -> SqlResult<Option<Cliente>> {
    let mut stmt = conn.prepare("SELECT id, nombre, telefono, notas FROM clientes WHERE id = ?1")?;
    let mut filas = stmt.query_map([id], mapear)?;
    Ok(filas.next().transpose()?)
}

pub fn actualizar(conn: &Connection, c: &Cliente, id: i64) -> SqlResult<usize> {
    conn.execute(
        "UPDATE clientes SET nombre=?1, telefono=?2, notas=?3 WHERE id=?4",
        (&c.nombre, &c.telefono, &c.notas, id),
    )
}

/// ¿Tiene ventas asociadas? (Si las tiene, no se puede eliminar.)
pub fn contar_ventas(conn: &Connection, id: i64) -> SqlResult<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM ventas WHERE cliente_id = ?1",
        [id],
        |fila| fila.get(0),
    )
}

pub fn eliminar(conn: &Connection, id: i64) -> SqlResult<usize> {
    conn.execute("DELETE FROM clientes WHERE id = ?1", [id])
}
