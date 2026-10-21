//! Construcción centralizada de mensajes de error.
//!
//! Los comandos Tauri devuelven `Result<T, String>` porque el frontend
//! muestra el mensaje tal cual al usuario. Estas ayudas garantizan que
//! todos los módulos hablen el mismo idioma y que los errores técnicos
//! de SQLite se traduzcan a mensajes comprensibles en español.

use rusqlite::Error as RusqliteError;

/// Error al tomar el mutex de la conexión (nunca debería ocurrir en uso normal).
pub fn bloqueo(e: impl std::fmt::Display) -> String {
    format!("Error interno accediendo a la base de datos: {e}")
}

/// Traduce un error de rusqlite a un mensaje para el usuario final:
/// - violación UNIQUE → aviso de duplicado;
/// - violación FOREIGN KEY → `mensaje_fk` si se provee, o un genérico;
/// - cualquier otro → mensaje técnico (útil para diagnóstico).
pub fn base_datos(e: RusqliteError, mensaje_fk: Option<&str>) -> String {
    const SQLITE_CONSTRAINT_UNIQUE: i32 = 1555;
    const SQLITE_CONSTRAINT_FOREIGNKEY: i32 = 787;
    if let RusqliteError::SqliteFailure(ffi, detalle) = &e {
        match ffi.extended_code {
            SQLITE_CONSTRAINT_UNIQUE => {
                return "Ya existe un registro con esos datos.".to_string();
            }
            SQLITE_CONSTRAINT_FOREIGNKEY => {
                return mensaje_fk
                    .unwrap_or("No se puede completar la operación: hay registros relacionados.")
                    .to_string();
            }
            _ => {
                if let Some(d) = detalle {
                    return format!("Error de base de datos: {d}");
                }
            }
        }
    }
    format!("Error de base de datos: {e}")
}

/// Atajo de [`base_datos`] cuando no hay un mensaje de FK específico.
pub fn bd(e: RusqliteError) -> String {
    base_datos(e, None)
}
