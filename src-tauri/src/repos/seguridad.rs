//! Persistencia del código de seguridad del administrador
//! (protege editar/eliminar ventas). Solo se guarda su hash bcrypt.

use rusqlite::{Connection, Result as SqlResult};

pub fn existe_codigo(conn: &Connection) -> SqlResult<bool> {
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM configuracion_seguridad WHERE id = 1)",
        [],
        |fila| fila.get(0),
    )
}

pub fn guardar_hash(conn: &Connection, hash: &str) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO configuracion_seguridad (id, codigo_admin_hash) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET codigo_admin_hash = excluded.codigo_admin_hash",
        [hash],
    )?;
    Ok(())
}

pub fn obtener_hash(conn: &Connection) -> SqlResult<Option<String>> {
    let mut stmt =
        conn.prepare("SELECT codigo_admin_hash FROM configuracion_seguridad WHERE id = 1")?;
    let mut filas = stmt.query_map([], |fila| fila.get(0))?;
    Ok(filas.next().transpose()?)
}
