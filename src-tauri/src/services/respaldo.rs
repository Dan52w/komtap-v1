//! Respaldo y restauración de la base de datos.
//!
//! Usa la API de backup en línea de SQLite (`rusqlite::backup`): funciona
//! con la base abierta y en modo WAL, sin cerrar la conexión ni reiniciar
//! la app. Al restaurar se re-ejecutan las migraciones por si el respaldo
//! viene de una versión anterior del esquema.

use rusqlite::{backup::Backup, Connection};
use std::path::Path;
use std::time::Duration;

/// Copia la base de datos actual al archivo `destino`.
pub fn exportar(conn: &Connection, destino: &Path) -> Result<(), String> {
    let mut dst =
        Connection::open(destino).map_err(|e| format!("No se pudo crear el respaldo: {e}"))?;
    let backup = Backup::new(conn, &mut dst).map_err(crate::errors::bd)?;
    backup
        .run_to_completion(100, Duration::from_millis(50), None)
        .map_err(|e| format!("Error durante el respaldo: {e}"))?;
    Ok(())
}

/// Restaura la base de datos desde el archivo `origen`, validando antes que
/// sea un respaldo válido de Komtap. No toca los datos actuales si el archivo
/// no es válido.
pub fn importar(conn: &mut Connection, origen: &Path) -> Result<(), String> {
    if !origen.exists() {
        return Err("El archivo de respaldo no existe".to_string());
    }
    let src = Connection::open(origen).map_err(|_| "No se pudo abrir el archivo de respaldo".to_string())?;

    let integro: String = src
        .query_row("PRAGMA quick_check", [], |fila| fila.get(0))
        .map_err(|_| "El archivo de respaldo está corrupto".to_string())?;
    if integro != "ok" {
        return Err("El archivo de respaldo está corrupto".to_string());
    }
    let es_komtap: i64 = src
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('productos', 'ventas', 'licencia_sistema')",
            [],
            |fila| fila.get(0),
        )
        .map_err(|_| "El archivo no es un respaldo válido de Komtap".to_string())?;
    if es_komtap != 3 {
        return Err("El archivo no es un respaldo válido de Komtap".to_string());
    }

    let backup = Backup::new(&src, conn).map_err(crate::errors::bd)?;
    backup
        .run_to_completion(100, Duration::from_millis(50), None)
        .map_err(|e| format!("Error durante la restauración: {e}"))?;

    // El respaldo puede venir de una versión anterior: re-ejecutar
    // migraciones (son idempotentes). `backup` retiene &mut conn: hay que
    // soltarlo antes de volver a usar `conn`.
    drop(backup);
    crate::db::ejecutar_migraciones(conn);
    Ok(())
}
