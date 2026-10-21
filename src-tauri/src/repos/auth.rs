//! Persistencia de licencia, códigos de activación usados y usuarios.

use crate::models::EstadoLicencia;
use rusqlite::{Connection, Result as SqlResult};

/// Lee el registro único de licencia (id = 1), si existe.
pub fn obtener_licencia(conn: &Connection) -> SqlResult<Option<(String, String, Option<String>)>> {
    let mut stmt = conn.prepare(
        "SELECT estado, tipo_licencia, fecha_expiracion FROM licencia_sistema WHERE id = 1",
    )?;
    let mut filas = stmt.query_map([], |fila| {
        Ok((fila.get(0)?, fila.get(1)?, fila.get(2)?))
    })?;
    Ok(filas.next().transpose()?)
}

/// ¿La licencia temporal sigue vigente según la hora local del sistema?
pub fn licencia_temporal_vigente(conn: &Connection) -> SqlResult<bool> {
    conn.query_row(
        "SELECT fecha_expiracion > datetime('now','localtime') FROM licencia_sistema WHERE id = 1",
        [],
        |fila| fila.get(0),
    )
}

pub fn marcar_licencia_expirada(conn: &Connection) -> SqlResult<usize> {
    conn.execute(
        "UPDATE licencia_sistema SET estado = 'expirada' WHERE id = 1",
        [],
    )
}

/// Inserta o reemplaza el registro único de licencia (id = 1).
pub fn guardar_licencia(
    conn: &Connection,
    codigo: &str,
    tipo: &str,
    fecha_expiracion: &Option<String>,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO licencia_sistema (id, codigo_activacion, tipo_licencia, fecha_expiracion, estado)
         VALUES (1, ?1, ?2, ?3, 'activa')
         ON CONFLICT(id) DO UPDATE SET
            codigo_activacion = excluded.codigo_activacion,
            tipo_licencia = excluded.tipo_licencia,
            fecha_expiracion = excluded.fecha_expiracion,
            estado = 'activa'",
        (codigo, tipo, fecha_expiracion),
    )?;
    Ok(())
}

/// Fecha de expiración calculada por SQLite para una licencia temporal.
pub fn calcular_expiracion(conn: &Connection, dias: u16) -> SqlResult<String> {
    conn.query_row(
        "SELECT datetime('now','localtime', ?1)",
        [format!("+{} days", dias)],
        |fila| fila.get(0),
    )
}

pub fn codigo_ya_usado(conn: &Connection, codigo: &str) -> SqlResult<bool> {
    conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM codigos_activacion_usados WHERE codigo = ?1)",
        [codigo],
        |fila| fila.get(0),
    )
}

pub fn marcar_codigo_usado(conn: &Connection, codigo: &str) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO codigos_activacion_usados (codigo) VALUES (?1)",
        [codigo],
    )?;
    Ok(())
}

// ===== Usuarios =====

pub fn contar_usuarios(conn: &Connection) -> SqlResult<i64> {
    conn.query_row("SELECT COUNT(*) FROM usuarios", [], |fila| fila.get(0))
}

pub fn insertar_usuario(
    conn: &Connection,
    username: &str,
    password_hash: &str,
    nombre_completo: &Option<String>,
) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO usuarios (username, password_hash, nombre_completo) VALUES (?1, ?2, ?3)",
        (username, password_hash, nombre_completo),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn obtener_credencial(conn: &Connection, username: &str) -> SqlResult<Option<(i64, String)>> {
    let mut stmt = conn.prepare("SELECT id, password_hash FROM usuarios WHERE username = ?1")?;
    let mut filas = stmt.query_map([username], |fila| Ok((fila.get(0)?, fila.get(1)?)))?;
    Ok(filas.next().transpose()?)
}

/// Construye el [`EstadoLicencia`] a partir de la fila cruda. Separa la
/// lectura (repositorio) de la interpretación (servicio).
pub fn estado_desde_fila(
    fila: Option<(String, String, Option<String>)>,
    temporal_vigente: bool,
) -> EstadoLicencia {
    let Some((estado, tipo_licencia, fecha_expiracion)) = fila else {
        return EstadoLicencia {
            activa: false,
            tipo_licencia: None,
            fecha_expiracion: None,
        };
    };
    if estado != "activa" {
        return EstadoLicencia {
            activa: false,
            tipo_licencia: Some(tipo_licencia),
            fecha_expiracion,
        };
    }
    if tipo_licencia == "permanente" {
        return EstadoLicencia {
            activa: true,
            tipo_licencia: Some(tipo_licencia),
            fecha_expiracion,
        };
    }
    EstadoLicencia {
        activa: temporal_vigente,
        tipo_licencia: Some(tipo_licencia),
        fecha_expiracion,
    }
}
