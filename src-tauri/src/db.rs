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
