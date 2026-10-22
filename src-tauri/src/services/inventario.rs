//! Reglas de negocio de inventario: validación de productos y ajustes
//! de stock auditados.

use crate::models::Producto;
use crate::repos::productos;
use rusqlite::Connection;

fn validar_producto(p: &Producto) -> Result<(), String> {
    if p.nombre.trim().is_empty() {
        return Err("El nombre de la prenda es obligatorio".to_string());
    }
    if p.categoria.trim().is_empty() {
        return Err("La categoría es obligatoria".to_string());
    }
    if p.talla.trim().is_empty() {
        return Err("La talla es obligatoria".to_string());
    }
    if p.costo_unitario < 0.0 {
        return Err("El costo unitario no puede ser negativo".to_string());
    }
    if p.precio_venta < 0.0 {
        return Err("El precio de venta no puede ser negativo".to_string());
    }
    if p.stock_actual < 0 {
        return Err("El stock actual no puede ser negativo".to_string());
    }
    if p.stock_minimo < 0 {
        return Err("El stock mínimo no puede ser negativo".to_string());
    }
    if !["activo", "inactivo"].contains(&p.estado.as_str()) {
        return Err("Estado no válido (use 'activo' o 'inactivo')".to_string());
    }
    Ok(())
}

pub fn crear(conn: &Connection, producto: &Producto) -> Result<i64, String> {
    validar_producto(producto)?;
    productos::insertar(conn, producto).map_err(|e| crate::errors::base_datos(e, None))
}

pub fn actualizar(conn: &Connection, producto: &Producto) -> Result<(), String> {
    let id = producto
        .id
        .ok_or_else(|| "El producto no tiene identificador".to_string())?;
    validar_producto(producto)?;
    let filas = productos::actualizar(conn, producto, id).map_err(crate::errors::bd)?;
    if filas == 0 {
        return Err("Producto no encontrado".to_string());
    }
    Ok(())
}

/// Ajuste manual de stock (pérdidas, defectos, correcciones internas).
/// `delta` puede ser positivo o negativo; se valida que el resultado no sea
/// negativo y el movimiento queda auditado en `ajustes_stock` con su motivo.
pub fn ajustar_stock(
    conn: &Connection,
    producto_id: i64,
    delta: i64,
    motivo: &str,
) -> Result<i64, String> {
    let motivo = motivo.trim();
    if motivo.is_empty() {
        return Err("Debe indicar un motivo para el ajuste de stock".to_string());
    }
    let stock_actual = productos::obtener_stock(conn, producto_id)
        .map_err(crate::errors::bd)?
        .ok_or_else(|| "Producto no encontrado".to_string())?;

    let nuevo_stock = stock_actual + delta;
    if nuevo_stock < 0 {
        return Err("El ajuste dejaría el stock en un valor negativo".to_string());
    }

    productos::fijar_stock(conn, producto_id, nuevo_stock).map_err(crate::errors::bd)?;
    productos::auditar_ajuste(conn, producto_id, delta, stock_actual, nuevo_stock, motivo)
        .map_err(crate::errors::bd)?;

    Ok(nuevo_stock)
}

pub fn eliminar(conn: &Connection, producto_id: i64) -> Result<(), String> {
    let (ventas, compras) = productos::contar_usos(conn, producto_id).map_err(crate::errors::bd)?;
    if ventas > 0 {
        return Err(
            "No se puede eliminar: el producto tiene ventas asociadas. Márquelo como inactivo."
                .to_string(),
        );
    }
    if compras > 0 {
        return Err(
            "No se puede eliminar: el producto tiene compras registradas a proveedores. Márquelo como inactivo."
                .to_string(),
        );
    }
    let filas = productos::eliminar(conn, producto_id).map_err(|e| {
        crate::errors::base_datos(
            e,
            Some("No se puede eliminar: el producto todavía tiene registros relacionados. Márquelo como inactivo."),
        )
    })?;
    if filas == 0 {
        return Err("Producto no encontrado".to_string());
    }
    Ok(())
}
