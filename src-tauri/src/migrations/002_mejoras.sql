-- ============================================================================
-- 002. MEJORAS: clientes, auditoría de ajustes de stock
-- ============================================================================
-- NOTA: el ALTER TABLE de `ventas` no es idempotente en SQLite, así que se
-- ejecuta desde db.rs verificando antes con PRAGMA table_info. Todo lo demás
-- usa IF NOT EXISTS y puede correrse cualquier cantidad de veces.

-- Clientes registrados (para ventas a crédito identificadas)
CREATE TABLE IF NOT EXISTS clientes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    telefono TEXT,
    notas TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Auditoría de ajustes manuales de stock (pérdidas, defectos, correcciones).
-- Antes el motivo se pedía pero no se guardaba en ningún lado.
CREATE TABLE IF NOT EXISTS ajustes_stock (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    producto_id INTEGER NOT NULL,
    delta INTEGER NOT NULL,
    stock_anterior INTEGER NOT NULL,
    stock_nuevo INTEGER NOT NULL,
    motivo TEXT NOT NULL,
    fecha TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (producto_id) REFERENCES productos(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_ventas_cliente ON ventas(cliente_id);
CREATE INDEX IF NOT EXISTS idx_ajustes_stock_producto ON ajustes_stock(producto_id);
CREATE INDEX IF NOT EXISTS idx_clientes_nombre ON clientes(nombre);
