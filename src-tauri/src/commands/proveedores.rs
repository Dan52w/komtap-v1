use crate::db::DbConnection;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Proveedor {
    pub id: Option<i64>,
    pub nombre: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub correo: Option<String>,
    pub notas: Option<String>,
}

const METODOS_PAGO_VALIDOS: [&str; 6] = ["Efectivo", "Transferencia", "Transferencia bancaria", "Consignación", "Cheque", "Otro"];

fn validar_metodo_pago(metodo: &str) -> Result<(), String> {
    if !METODOS_PAGO_VALIDOS.contains(&metodo) {
        return Err("Método de pago no válido".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn crear_proveedor(state: State<DbConnection>, proveedor: Proveedor) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO proveedores (nombre, telefono, direccion, correo, notas) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&proveedor.nombre, &proveedor.telefono, &proveedor.direccion, &proveedor.correo, &proveedor.notas),
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn listar_proveedores(state: State<DbConnection>) -> Result<Vec<Proveedor>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id, nombre, telefono, direccion, correo, notas FROM proveedores ORDER BY nombre")
        .map_err(|e| e.to_string())?;

    let proveedores = stmt.query_map([], |row| {
        Ok(Proveedor {
            id: Some(row.get(0)?),
            nombre: row.get(1)?,
            telefono: row.get(2)?,
            direccion: row.get(3)?,
            correo: row.get(4)?,
            notas: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(proveedores)
}

#[tauri::command]
pub fn actualizar_proveedor(state: State<DbConnection>, proveedor: Proveedor) -> Result<(), String> {
    let id = proveedor.id.ok_or("El proveedor no tiene identificador")?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE proveedores SET nombre=?1, telefono=?2, direccion=?3, correo=?4, notas=?5 WHERE id=?6",
        (&proveedor.nombre, &proveedor.telefono, &proveedor.direccion, &proveedor.correo, &proveedor.notas, id))
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn eliminar_proveedor(state: State<DbConnection>, proveedor_id: i64) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let facturas_asociadas: i64 = conn.query_row(
        "SELECT COUNT(*) FROM facturas_proveedor WHERE proveedor_id = ?1",
        [proveedor_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    if facturas_asociadas > 0 {
        return Err("No se puede eliminar el proveedor porque tiene facturas/compras asociadas".to_string());
    }

    conn.execute("DELETE FROM proveedores WHERE id=?1", [proveedor_id]).map_err(|e| e.to_string())?;
    Ok(())
}

// ===== COMPRAS / FACTURAS DE PROVEEDOR (HU-02) =====

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

#[derive(Serialize, Deserialize)]
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

fn calcular_subtotal(cantidad: i64, costo_unitario: f64) -> f64 {
    (cantidad as f64 * costo_unitario * 100.0).round() / 100.0
}

fn estado_pago_para(saldo_pendiente: f64, monto_total: f64) -> &'static str {
    if saldo_pendiente <= 0.0 {
        "Pagado"
    } else if saldo_pendiente < monto_total {
        "Parcial"
    } else {
        "Pendiente"
    }
}

/// Registra una compra (factura de proveedor). Los ítems alimentan el
/// inventario automáticamente vía trigger `trg_aumentar_stock_compra`.
#[tauri::command]
pub fn crear_factura_proveedor(state: State<DbConnection>, factura: NuevaFactura) -> Result<i64, String> {
    if factura.items.is_empty() {
        return Err("La compra debe tener al menos un ítem".to_string());
    }
    if let Some(ref abono) = factura.abono_inicial {
        validar_metodo_pago(&abono.metodo_pago)?;
        if abono.monto <= 0.0 {
            return Err("El abono inicial debe ser mayor que cero".to_string());
        }
    }

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let monto_total: f64 = factura.items.iter()
        .map(|i| calcular_subtotal(i.cantidad, i.costo_unitario))
        .sum();

    if let Some(ref abono) = factura.abono_inicial {
        if abono.monto > monto_total {
            return Err("El abono inicial no puede superar el total de la compra".to_string());
        }
    }

    let saldo_inicial = monto_total - factura.abono_inicial.as_ref().map(|a| a.monto).unwrap_or(0.0);

    tx.execute(
        "INSERT INTO facturas_proveedor (proveedor_id, numero_factura, fecha_emision, fecha_limite_pago, monto_total, saldo_pendiente, estado_pago)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            factura.proveedor_id,
            &factura.numero_factura,
            &factura.fecha_emision,
            &factura.fecha_limite_pago,
            monto_total,
            saldo_inicial,
            estado_pago_para(saldo_inicial, monto_total),
        ),
    ).map_err(|e| e.to_string())?;

    let factura_id = tx.last_insert_rowid();

    for item in &factura.items {
        let producto_id = match item.producto_id {
            Some(id) => id,
            None => {
                let nombre = item.nombre_nuevo.clone().ok_or("Falta el nombre de la prenda nueva")?;
                tx.execute(
                    "INSERT INTO productos (sku, nombre, categoria, talla, color, costo_unitario, precio_venta, stock_actual, stock_minimo)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8)",
                    (
                        &item.sku_nuevo,
                        &nombre,
                        item.categoria_nueva.clone().unwrap_or_else(|| "General".to_string()),
                        item.talla_nueva.clone().unwrap_or_else(|| "Única".to_string()),
                        &item.color_nuevo,
                        item.costo_unitario,
                        item.precio_venta_sugerido.unwrap_or(0.0),
                        item.stock_minimo_nuevo.unwrap_or(3).max(0),
                    ),
                ).map_err(|e| e.to_string())?;
                tx.last_insert_rowid()
            }
        };

        let subtotal = calcular_subtotal(item.cantidad, item.costo_unitario);
        tx.execute(
            "INSERT INTO items_factura_proveedor (factura_id, producto_id, cantidad, costo_unitario, precio_venta_sugerido, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (factura_id, producto_id, item.cantidad, item.costo_unitario, item.precio_venta_sugerido, subtotal),
        ).map_err(|e| e.to_string())?;
    }

    if let Some(abono) = &factura.abono_inicial {
        tx.execute(
            "INSERT INTO abonos_proveedor (factura_id, monto, metodo_pago, banco, numero_referencia, nota, fecha)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, COALESCE(?7, datetime('now','localtime')))",
            (factura_id, abono.monto, &abono.metodo_pago, &abono.banco, &abono.numero_referencia, &abono.nota, &abono.fecha),
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(factura_id)
}

#[tauri::command]
pub fn listar_facturas_proveedor(state: State<DbConnection>, proveedor_id: i64) -> Result<Vec<Factura>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, proveedor_id, numero_factura, fecha_emision, fecha_limite_pago, monto_total, saldo_pendiente, estado_pago
         FROM facturas_proveedor WHERE proveedor_id = ?1 ORDER BY fecha_emision DESC"
    ).map_err(|e| e.to_string())?;

    let facturas = stmt.query_map([proveedor_id], |row| {
        Ok(Factura {
            id: row.get(0)?,
            proveedor_id: row.get(1)?,
            numero_factura: row.get(2)?,
            fecha_emision: row.get(3)?,
            fecha_limite_pago: row.get(4)?,
            monto_total: row.get(5)?,
            saldo_pendiente: row.get(6)?,
            estado_pago: row.get(7)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(facturas)
}

#[tauri::command]
pub fn listar_abonos_factura_proveedor(state: State<DbConnection>, factura_id: i64) -> Result<Vec<AbonoProveedor>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, factura_id, monto, fecha, metodo_pago, banco, numero_referencia, nota
         FROM abonos_proveedor WHERE factura_id = ?1 ORDER BY fecha DESC, id DESC"
    ).map_err(|e| e.to_string())?;
    let abonos = stmt.query_map([factura_id], |row| {
        Ok(AbonoProveedor {
            id: row.get(0)?, factura_id: row.get(1)?, monto: row.get(2)?, fecha: row.get(3)?,
            metodo_pago: row.get(4)?, banco: row.get(5)?, numero_referencia: row.get(6)?, nota: row.get(7)?,
        })
    }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();
    Ok(abonos)
}

#[tauri::command]
pub fn listar_items_factura_proveedor(state: State<DbConnection>, factura_id: i64) -> Result<Vec<ItemFacturaDetalle>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT i.id, i.producto_id, p.nombre, i.cantidad, i.costo_unitario, i.precio_venta_sugerido, i.subtotal
         FROM items_factura_proveedor i JOIN productos p ON p.id = i.producto_id
         WHERE i.factura_id = ?1 ORDER BY i.id"
    ).map_err(|e| e.to_string())?;
    let items = stmt.query_map([factura_id], |row| Ok(ItemFacturaDetalle {
        id: row.get(0)?, producto_id: row.get(1)?, producto_nombre: row.get(2)?, cantidad: row.get(3)?,
        costo_unitario: row.get(4)?, precio_venta_sugerido: row.get(5)?, subtotal: row.get(6)?,
    })).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect();
    Ok(items)
}

#[tauri::command]
pub fn registrar_abono_proveedor(state: State<DbConnection>, factura_id: i64, abono: AbonoInput) -> Result<i64, String> {
    validar_metodo_pago(&abono.metodo_pago)?;

    let mut conn = state.0.lock().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let (saldo_pendiente, monto_total): (f64, f64) = tx.query_row(
        "SELECT saldo_pendiente, monto_total FROM facturas_proveedor WHERE id = ?1",
        [factura_id], |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| "Factura no encontrada".to_string())?;

    if abono.monto <= 0.0 || abono.monto > saldo_pendiente {
        return Err("El abono debe ser mayor que cero y no superar el saldo pendiente".to_string());
    }

    tx.execute(
        "INSERT INTO abonos_proveedor (factura_id, monto, metodo_pago, banco, numero_referencia, nota, fecha)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, COALESCE(?7, datetime('now','localtime')))",
        (factura_id, abono.monto, &abono.metodo_pago, &abono.banco, &abono.numero_referencia, &abono.nota, &abono.fecha),
    ).map_err(|e| e.to_string())?;

    let nuevo_saldo = saldo_pendiente - abono.monto;
    tx.execute(
        "UPDATE facturas_proveedor SET saldo_pendiente = ?1, estado_pago = ?2 WHERE id = ?3",
        (nuevo_saldo, estado_pago_para(nuevo_saldo, monto_total), factura_id),
    ).map_err(|e| e.to_string())?;

    let abono_id = tx.last_insert_rowid();
    tx.commit().map_err(|e| e.to_string())?;
    Ok(abono_id)
}

#[derive(Serialize, Deserialize)]
pub struct ResumenProveedor {
    pub proveedor_id: i64,
    pub nombre: String,
    pub deuda_total: f64,
}

#[tauri::command]
pub fn deuda_total_global(state: State<DbConnection>) -> Result<f64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT COALESCE(SUM(saldo_pendiente), 0) FROM facturas_proveedor",
        [], |row| row.get(0),
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn resumen_por_proveedor(state: State<DbConnection>) -> Result<Vec<ResumenProveedor>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT p.id, p.nombre, COALESCE(SUM(f.saldo_pendiente), 0) AS deuda_total
         FROM proveedores p
         LEFT JOIN facturas_proveedor f ON f.proveedor_id = p.id
         GROUP BY p.id
         ORDER BY deuda_total DESC"
    ).map_err(|e| e.to_string())?;

    let resumen = stmt.query_map([], |row| {
        Ok(ResumenProveedor {
            proveedor_id: row.get(0)?,
            nombre: row.get(1)?,
            deuda_total: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
      .filter_map(|r| r.ok())
      .collect();

    Ok(resumen)
}
