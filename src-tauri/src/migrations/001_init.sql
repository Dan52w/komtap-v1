PRAGMA foreign_keys = ON;

-- ============================================================================
-- 1. MÓDULO DE SEGURIDAD, AUTENTICACIÓN Y LICENCIAMIENTO
-- ============================================================================

-- Configuración y control de licencia del sistema (Registro único)
CREATE TABLE IF NOT EXISTS licencia_sistema (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- Garantiza un solo registro de activación
    codigo_activacion TEXT NOT NULL UNIQUE,
    tipo_licencia TEXT NOT NULL CHECK (tipo_licencia IN ('temporal', 'permanente')),
    fecha_activacion TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    fecha_expiracion TEXT, -- NULL si es permanente, formato 'YYYY-MM-DD HH:MM:SS' si es temporal
    estado TEXT NOT NULL DEFAULT 'activa' CHECK (estado IN ('activa', 'expirada', 'revocada')),
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Usuario administrador local único
CREATE TABLE IF NOT EXISTS usuarios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    nombre_completo TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- 2. MÓDULO DE INVENTARIO Y CATÁLOGO DE PRENDAS
-- ============================================================================

CREATE TABLE IF NOT EXISTS productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sku TEXT UNIQUE, -- Código interno o código de barras (opcional)
    nombre TEXT NOT NULL, -- Ej: "Jeans Skinny Dama"
    categoria TEXT NOT NULL DEFAULT 'General', -- Ej: 'Camisetas', 'Pantalones', 'Accesorios'
    talla TEXT NOT NULL, -- Ej: 'XS', 'S', 'M', 'L', 'XL', '32'
    color TEXT, -- Ej: 'Negro', 'Azul Oscuro'
    costo_unitario REAL NOT NULL DEFAULT 0 CHECK (costo_unitario >= 0), -- Precio de compra base
    precio_venta REAL NOT NULL DEFAULT 0 CHECK (precio_venta >= 0), -- PVP al público
    stock_actual INTEGER NOT NULL DEFAULT 0 CHECK (stock_actual >= 0),
    stock_minimo INTEGER NOT NULL DEFAULT 3 CHECK (stock_minimo >= 0), -- Umbral para alertas
    estado TEXT NOT NULL DEFAULT 'activo' CHECK (estado IN ('activo', 'inactivo')),
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- 3. MÓDULO DE PROVEEDORES Y COMPRAS DE MERCANCÍA
-- ============================================================================

CREATE TABLE IF NOT EXISTS proveedores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL UNIQUE,
    telefono TEXT,
    direccion TEXT,
    correo TEXT,
    notas TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS facturas_proveedor (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    proveedor_id INTEGER NOT NULL,
    numero_factura TEXT,
    fecha_emision TEXT NOT NULL DEFAULT (date('now')),
    fecha_limite_pago TEXT,
    monto_total REAL NOT NULL DEFAULT 0 CHECK (monto_total >= 0),
    saldo_pendiente REAL NOT NULL DEFAULT 0 CHECK (saldo_pendiente >= 0),
    estado_pago TEXT NOT NULL DEFAULT 'Pendiente' CHECK (estado_pago IN ('Pendiente', 'Parcial', 'Pagado')),
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (proveedor_id) REFERENCES proveedores(id) ON DELETE RESTRICT
);

-- Desglose de prendas compradas en la factura
CREATE TABLE IF NOT EXISTS items_factura_proveedor (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    factura_id INTEGER NOT NULL,
    producto_id INTEGER NOT NULL, -- Vinculado directamente al inventario
    cantidad INTEGER NOT NULL CHECK (cantidad > 0),
    costo_unitario REAL NOT NULL CHECK (costo_unitario >= 0),
    precio_venta_sugerido REAL CHECK (precio_venta_sugerido >= 0),
    subtotal REAL NOT NULL CHECK (subtotal >= 0),
    FOREIGN KEY (factura_id) REFERENCES facturas_proveedor(id) ON DELETE CASCADE,
    FOREIGN KEY (producto_id) REFERENCES productos(id) ON DELETE RESTRICT
);

-- Historial de abonos a compras de proveedores
CREATE TABLE IF NOT EXISTS abonos_proveedor (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    factura_id INTEGER NOT NULL,
    monto REAL NOT NULL CHECK (monto > 0),
    fecha TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    metodo_pago TEXT NOT NULL DEFAULT 'Efectivo', -- 'Efectivo', 'Transferencia', 'Cheque', 'Otro'
    banco TEXT,
    numero_referencia TEXT,
    nota TEXT,
    FOREIGN KEY (factura_id) REFERENCES facturas_proveedor(id) ON DELETE CASCADE
);

-- ============================================================================
-- 4. MÓDULO DE VENTAS (POS)
-- ============================================================================
-- Nota: no se lleva un catálogo de clientes; solo se registran las ventas,
-- los artículos vendidos y (si aplica) el saldo pendiente de esa venta puntual.

CREATE TABLE IF NOT EXISTS ventas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    subtotal REAL NOT NULL CHECK (subtotal >= 0),
    descuento REAL NOT NULL DEFAULT 0 CHECK (descuento >= 0),
    total_neto REAL NOT NULL CHECK (total_neto >= 0),
    monto_pagado REAL NOT NULL DEFAULT 0 CHECK (monto_pagado >= 0),
    saldo_pendiente REAL NOT NULL DEFAULT 0 CHECK (saldo_pendiente >= 0),
    metodo_pago TEXT NOT NULL DEFAULT 'Efectivo', -- 'Efectivo', 'Transferencia', 'Tarjeta', 'Mixto', 'Credito'
    estado_pago TEXT NOT NULL DEFAULT 'Pagado' CHECK (estado_pago IN ('Pagado', 'Pendiente', 'Parcial')),
    referencia_cliente TEXT, -- Nota libre y opcional (ej. nombre/teléfono) solo para ventas con saldo pendiente
    notas TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Detalle de ítems vendidos (guarda costo histórico para cálculo exacto de ganancia)
CREATE TABLE IF NOT EXISTS items_venta (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL,
    producto_id INTEGER NOT NULL,
    cantidad INTEGER NOT NULL CHECK (cantidad > 0),
    precio_unitario REAL NOT NULL CHECK (precio_unitario >= 0),
    costo_unitario REAL NOT NULL CHECK (costo_unitario >= 0), -- Snapshot del costo al momento de vender
    subtotal REAL NOT NULL CHECK (subtotal >= 0),
    FOREIGN KEY (venta_id) REFERENCES ventas(id) ON DELETE CASCADE,
    FOREIGN KEY (producto_id) REFERENCES productos(id) ON DELETE RESTRICT
);

-- Registros de abonos para ventas a crédito
CREATE TABLE IF NOT EXISTS abonos_venta (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    venta_id INTEGER NOT NULL,
    monto REAL NOT NULL CHECK (monto > 0),
    fecha TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    metodo_pago TEXT NOT NULL DEFAULT 'Efectivo',
    numero_referencia TEXT,
    nota TEXT,
    FOREIGN KEY (venta_id) REFERENCES ventas(id) ON DELETE CASCADE
);

-- ============================================================================
-- 5. MÓDULO DE OTROS GASTOS OPERATIVOS
-- ============================================================================

CREATE TABLE IF NOT EXISTS otros_gastos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    descripcion TEXT NOT NULL, -- Ej: "Pago de servicios públicos", "Bolsas de empaque"
    monto REAL NOT NULL CHECK (monto > 0),
    fecha TEXT NOT NULL DEFAULT (date('now')),
    categoria TEXT NOT NULL DEFAULT 'General', -- Ej: 'Arriendo', 'Servicios', 'Nómina', 'Mantenimiento', 'Otros'
    metodo_pago TEXT NOT NULL DEFAULT 'Efectivo',
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- 6. TRIGGERS AUTOMÁTICOS PARA CONTROL DE INVENTARIO
-- ============================================================================

-- Descuenta del stock automáticamente al realizar una venta
CREATE TRIGGER IF NOT EXISTS trg_descontar_stock_venta
AFTER INSERT ON items_venta
BEGIN
    UPDATE productos
    SET stock_actual = stock_actual - NEW.cantidad, updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.producto_id;
END;

-- Incrementa el stock y actualiza el costo base automáticamente al ingresar una compra
CREATE TRIGGER IF NOT EXISTS trg_aumentar_stock_compra
AFTER INSERT ON items_factura_proveedor
BEGIN
    UPDATE productos
    SET stock_actual = stock_actual + NEW.cantidad,
        costo_unitario = NEW.costo_unitario,
        precio_venta = COALESCE(NULLIF(NEW.precio_venta_sugerido, 0), precio_venta),
        updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.producto_id;
END;

-- Reinvierte el stock si una venta es cancelada / eliminada
CREATE TRIGGER IF NOT EXISTS trg_restaurar_stock_cancelacion_venta
AFTER DELETE ON items_venta
BEGIN
    UPDATE productos
    SET stock_actual = stock_actual + OLD.cantidad,
        updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.producto_id;
END;

-- ============================================================================
-- 7. ÍNDICES DE RENDIMIENTO (OPTIMIZACIÓN PARA CONSULTAS Y DASHBOARD)
-- ============================================================================

-- Búsqueda rápida de productos y alertas de stock
CREATE INDEX IF NOT EXISTS idx_productos_busqueda ON productos(nombre, categoria, talla);
CREATE INDEX IF NOT EXISTS idx_productos_stock ON productos(stock_actual, stock_minimo);
CREATE INDEX IF NOT EXISTS idx_productos_sku ON productos(sku);

-- Reportes de ventas y más vendidos
CREATE INDEX IF NOT EXISTS idx_ventas_fecha ON ventas(fecha);
CREATE INDEX IF NOT EXISTS idx_items_venta_producto ON items_venta(producto_id);
CREATE INDEX IF NOT EXISTS idx_items_venta_venta ON items_venta(venta_id);

-- Reportes de compras y proveedores
CREATE INDEX IF NOT EXISTS idx_facturas_proveedor ON facturas_proveedor(proveedor_id);
CREATE INDEX IF NOT EXISTS idx_items_factura ON items_factura_proveedor(factura_id);
CREATE INDEX IF NOT EXISTS idx_abonos_proveedor ON abonos_proveedor(factura_id);

-- Reportes contables y gastos
CREATE INDEX IF NOT EXISTS idx_gastos_fecha ON otros_gastos(fecha);
CREATE INDEX IF NOT EXISTS idx_licencia_estado ON licencia_sistema(estado, fecha_expiracion);

-- Registro de códigos de activación ya consumidos. Evita que un mismo código
-- (especialmente uno temporal) pueda reactivarse indefinidamente para
-- reiniciar el periodo de vigencia.
CREATE TABLE IF NOT EXISTS codigos_activacion_usados (
    codigo TEXT PRIMARY KEY,
    fecha_uso TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);

-- Backfill: si ya existía una licencia activa de una instalación previa a este
-- cambio, registra su código como usado para que no quede canjeable en otro lado.
INSERT OR IGNORE INTO codigos_activacion_usados (codigo)
SELECT codigo_activacion FROM licencia_sistema WHERE codigo_activacion IS NOT NULL;