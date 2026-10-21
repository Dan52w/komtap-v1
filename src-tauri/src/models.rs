//! Contratos de datos (DTOs) entre el frontend y los comandos Tauri.
//!
//! Viven aquí —y no junto a los comandos— para que los repositorios y los
//! servicios puedan usarlos sin depender de la capa de comandos (los
//! comandos son solo adaptadores delgados: validan la entrada, llaman al
//! servicio y devuelven el resultado).

use serde::{Deserialize, Serialize};

// ===== Autenticación y licencia =====

#[derive(Serialize, Deserialize)]
pub struct Usuario {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct EstadoLicencia {
    pub activa: bool,
    pub tipo_licencia: Option<String>,
    pub fecha_expiracion: Option<String>,
}

// ===== Productos / inventario =====

#[derive(Serialize, Deserialize)]
pub struct Producto {
    pub id: Option<i64>,
    pub sku: Option<String>,
    pub nombre: String,
    pub categoria: String,
    pub talla: String,
    pub color: Option<String>,
    pub costo_unitario: f64,
    pub precio_venta: f64,
    pub stock_actual: i64,
    pub stock_minimo: i64,
    pub estado: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FiltroProductos {
    pub categoria: Option<String>,
    pub talla: Option<String>,
    pub nombre: Option<String>,
    /// Código de barra / SKU. Coincidencia parcial para permitir escaneo progresivo.
    pub sku: Option<String>,
    /// "agotado" | "bajo" | "disponible" | None = todos
    pub estado_stock: Option<String>,
}

// ===== Proveedores y compras =====

#[derive(Serialize, Deserialize)]
pub struct Proveedor {
    pub id: Option<i64>,
    pub nombre: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub correo: Option<String>,
    pub notas: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemFacturaInput {
    /// Si `producto_id` es None, se crea un producto nuevo con los datos aquí provistos.
    pub producto_id: Option<i64>,
    pub nombre_nuevo: Option<String>,
    pub sku_nuevo: Option<String>,
    pub categoria_nueva: Option<String>,
    pub talla_nueva: Option<String>,
    pub color_nuevo: Option<String>,
    pub cantidad: i64,
    pub costo_unitario: f64,
    pub precio_venta_sugerido: Option<f64>,
    pub stock_minimo_nuevo: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AbonoInput {
    pub monto: f64,
    pub metodo_pago: String,
    pub banco: Option<String>,
    pub numero_referencia: Option<String>,
    pub nota: Option<String>,
    pub fecha: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NuevaFactura {
    pub proveedor_id: i64,
    pub numero_factura: Option<String>,
    pub fecha_emision: String,
    pub fecha_limite_pago: Option<String>,
    pub items: Vec<ItemFacturaInput>,
    pub abono_inicial: Option<AbonoInput>,
}

#[derive(Serialize, Deserialize)]
pub struct Factura {
    pub id: i64,
    pub proveedor_id: i64,
    pub numero_factura: Option<String>,
    pub fecha_emision: String,
    pub fecha_limite_pago: Option<String>,
    pub monto_total: f64,
    pub saldo_pendiente: f64,
    pub estado_pago: String,
}

#[derive(Serialize, Deserialize)]
pub struct AbonoProveedor {
    pub id: i64,
    pub factura_id: i64,
    pub monto: f64,
    pub fecha: String,
    pub metodo_pago: String,
    pub banco: Option<String>,
    pub numero_referencia: Option<String>,
    pub nota: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemFacturaDetalle {
    pub id: i64,
    pub producto_id: i64,
    pub producto_nombre: String,
    pub cantidad: i64,
    pub costo_unitario: f64,
    pub precio_venta_sugerido: Option<f64>,
    pub subtotal: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ResumenProveedor {
    pub proveedor_id: i64,
    pub nombre: String,
    pub deuda_total: f64,
}

/// Factura con saldo pendiente y el nombre del proveedor, para el reporte
/// global sin consultas N+1 desde el frontend.
#[derive(Serialize, Deserialize)]
pub struct FacturaPendiente {
    pub id: i64,
    pub proveedor_id: i64,
    pub proveedor_nombre: String,
    pub numero_factura: Option<String>,
    pub fecha_emision: String,
    pub fecha_limite_pago: Option<String>,
    pub monto_total: f64,
    pub saldo_pendiente: f64,
    pub estado_pago: String,
}

// ===== Ventas / POS =====

#[derive(Serialize, Deserialize)]
pub struct ItemVentaInput {
    pub producto_id: i64,
    pub cantidad: i64,
    pub precio_unitario: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NuevaVenta {
    pub items: Vec<ItemVentaInput>,
    pub descuento: Option<f64>,
    pub metodo_pago: String,
    pub monto_pagado: Option<f64>, // si es None, se asume pago total del neto
    /// Cliente registrado (opcional). Si es None, puede usarse `referencia_cliente`
    /// como nota libre.
    pub cliente_id: Option<i64>,
    /// Nota libre y opcional (ej. nombre/teléfono) útil solo si la venta queda
    /// con saldo pendiente y se necesita identificar a quién cobrarle luego.
    pub referencia_cliente: Option<String>,
    pub notas: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Venta {
    pub id: i64,
    pub fecha: String,
    pub subtotal: f64,
    pub descuento: f64,
    pub total_neto: f64,
    pub monto_pagado: f64,
    pub saldo_pendiente: f64,
    pub metodo_pago: String,
    pub estado_pago: String,
    pub cliente_id: Option<i64>,
    pub cliente_nombre: Option<String>,
    pub referencia_cliente: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemVenta {
    pub id: i64,
    pub producto_id: i64,
    pub producto_nombre: String,
    pub cantidad: i64,
    pub precio_unitario: f64,
    pub subtotal: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AbonoVentaInput {
    pub monto: f64,
    pub metodo_pago: String,
    pub numero_referencia: Option<String>,
    pub nota: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AbonoVenta {
    pub id: i64,
    pub venta_id: i64,
    pub monto: f64,
    pub fecha: String,
    pub metodo_pago: String,
    pub numero_referencia: Option<String>,
    pub nota: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VentaActualizada {
    pub items: Vec<ItemVentaInput>,
    pub descuento: Option<f64>,
    pub metodo_pago: String,
    pub monto_pagado: Option<f64>,
    pub cliente_id: Option<i64>,
    pub referencia_cliente: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TopPrenda {
    pub producto_id: i64,
    pub nombre: String,
    pub unidades_vendidas: i64,
    pub ingreso_total: f64,
    pub margen_estimado: f64,
}

// ===== Gastos operativos =====

#[derive(Serialize, Deserialize)]
pub struct Gasto {
    pub id: i64,
    pub descripcion: String,
    pub monto: f64,
    pub fecha: String,
    pub categoria: String,
    pub metodo_pago: String,
}

#[derive(Serialize, Deserialize)]
pub struct NuevoGasto {
    pub descripcion: String,
    pub monto: f64,
    pub fecha: Option<String>,
    pub categoria: Option<String>,
    pub metodo_pago: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GastoActualizado {
    pub descripcion: String,
    pub monto: f64,
    pub fecha: String,
    pub categoria: String,
    pub metodo_pago: String,
}

// ===== Reportes =====

#[derive(Serialize, Deserialize)]
pub struct DashboardFinanciero {
    pub dinero_ingresado: f64,   // Total recibido por ventas ejecutadas (pagos recibidos)
    pub dinero_gastado: f64,     // Abonos a proveedores + gastos operativos
    pub ganancia_bruta: f64,     // Ventas - costo de las prendas vendidas
    pub ganancia_neta_real: f64, // Ganancia bruta - gastos operativos
    pub valor_inventario: f64,   // Stock actual valorado a costo
}

#[derive(Serialize, Deserialize)]
pub struct VentasPorMetodoPago {
    pub metodo_pago: String,
    pub total: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ProductoCritico {
    pub id: i64,
    pub nombre: String,
    pub talla: String,
    pub stock_actual: i64,
    pub stock_minimo: i64,
    pub estado_stock: String, // "Agotado" | "Stock Bajo"
}

#[derive(Serialize, Deserialize)]
pub struct GastoOperativoReporte {
    pub descripcion: String,
    pub monto: f64,
    pub fecha: String,
    pub categoria: String,
}

// ===== Clientes =====

#[derive(Serialize, Deserialize)]
pub struct Cliente {
    pub id: Option<i64>,
    pub nombre: String,
    pub telefono: Option<String>,
    pub notas: Option<String>,
}
