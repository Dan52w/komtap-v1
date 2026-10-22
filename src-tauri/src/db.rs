use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

pub struct DbConnection(pub Mutex<Connection>);

/// Ejecuta las migraciones en orden. Son idempotentes: pueden correrse
/// cualquier cantidad de veces sin duplicar nada.
pub(crate) fn ejecutar_migraciones(conn: &Connection) {
    // 001: esquema base.
    conn.execute_batch(include_str!("migrations/001_init.sql"))
        .expect("error ejecutando migraciones");

    // ALTER TABLE no soporta IF NOT EXISTS en SQLite: se verifica antes.
    // OJO: tiene que ir ANTES de la 002, porque 002 crea el índice
    // idx_ventas_cliente sobre esta columna.
    let tiene_cliente_id: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM pragma_table_info('ventas') WHERE name = 'cliente_id'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);
    if !tiene_cliente_id {
        conn.execute(
            "ALTER TABLE ventas ADD COLUMN cliente_id INTEGER REFERENCES clientes(id) ON DELETE SET NULL",
            [],
        )
        .expect("error agregando columna cliente_id a ventas");
    }

    // 002: clientes + auditoría de ajustes de stock (todo con IF NOT EXISTS).
    conn.execute_batch(include_str!("migrations/002_mejoras.sql"))
        .expect("error ejecutando migraciones");
}

pub fn init_db(app_handle: &tauri::AppHandle) -> Connection {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("no se pudo obtener el directorio de datos");

    std::fs::create_dir_all(&app_dir).expect("no se pudo crear el directorio");
    let db_path = app_dir.join("komtap.db");

    let conn = Connection::open(db_path).expect("no se pudo abrir la base de datos");
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")
        .unwrap();

    ejecutar_migraciones(&conn);

    conn
}
