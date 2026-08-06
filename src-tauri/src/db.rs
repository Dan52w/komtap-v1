use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

pub struct DbConnection(pub Mutex<Connection>);

pub fn init_db(app_handle: &tauri::AppHandle) -> Connection {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("no se pudo obtener el directorio de datos");

    std::fs::create_dir_all(&app_dir).expect("no se pudo crear el directorio");
    let db_path = app_dir.join("komtap.db");

    let conn = Connection::open(db_path).expect("no se pudo abrir la base de datos");
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;").unwrap();

    let schema = include_str!("migrations/001_init.sql");
    conn.execute_batch(schema).expect("error ejecutando migraciones");

    conn
}

/// Convierte un error de rusqlite en un mensaje amigable cuando se trata
/// específicamente de una violación de foreign key (código extendido 787).
/// Para cualquier otro error, deja pasar el mensaje original de SQLite.
pub fn friendly_delete_error(e: rusqlite::Error, mensaje_fk: &str) -> String {
    if let rusqlite::Error::SqliteFailure(ffi_err, _) = &e {
        const SQLITE_CONSTRAINT_FOREIGNKEY: i32 = 787;
        if ffi_err.extended_code == SQLITE_CONSTRAINT_FOREIGNKEY {
            return mensaje_fk.to_string();
        }
    }
    e.to_string()
}