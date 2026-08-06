mod db;
mod models;
mod commands;

use db::{init_db, DbConnection};
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let conn = init_db(&app.handle());
            app.manage(DbConnection(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Licencia y autenticación
            commands::auth::estado_licencia,
            commands::auth::activar_licencia,
            commands::auth::existe_usuario_registrado,
            commands::auth::registrar_usuario,
            commands::auth::iniciar_sesion,
            // Inventario / productos
            commands::productos::crear_producto,
            commands::productos::listar_productos,
            commands::productos::actualizar_producto,
            commands::productos::ajustar_stock_manual,
            commands::productos::eliminar_producto,
            // Proveedores y compras
            commands::proveedores::crear_proveedor,
            commands::proveedores::listar_proveedores,
            commands::proveedores::actualizar_proveedor,
            commands::proveedores::eliminar_proveedor,
            commands::proveedores::crear_factura_proveedor,
            commands::proveedores::listar_facturas_proveedor,
            commands::proveedores::listar_abonos_factura_proveedor,
            commands::proveedores::listar_items_factura_proveedor,
            commands::proveedores::registrar_abono_proveedor,
            commands::proveedores::deuda_total_global,
            commands::proveedores::resumen_por_proveedor,
            // Ventas / POS
            commands::ventas::crear_venta,
            commands::ventas::listar_ventas,
            commands::ventas::listar_items_venta,
            commands::ventas::registrar_abono_venta,
            commands::ventas::top_prendas_vendidas,
            // Gastos operativos
            commands::gastos::crear_gasto,
            commands::gastos::listar_gastos,
            commands::gastos::eliminar_gasto,
            // Contabilidad y reportes
            commands::reportes::dashboard_financiero,
            commands::reportes::ventas_por_metodo_pago,
            commands::reportes::productos_stock_critico,
            commands::reportes::desglose_gastos_mes,
        ])
        .run(tauri::generate_context!())
        .expect("error corriendo la app");
}
