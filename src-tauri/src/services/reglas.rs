//! Reglas de validación compartidas entre servicios.
//!
//! Centraliza las listas cerradas de métodos de pago y categorías para que
//! no estén duplicadas (y potencialmente divergentes) en cada módulo.

pub const METODOS_PAGO_VENTA: [&str; 5] =
    ["Efectivo", "Transferencia", "Tarjeta", "Mixto", "Credito"];

pub const METODOS_PAGO_COMPRA: [&str; 6] = [
    "Efectivo",
    "Transferencia",
    "Transferencia bancaria",
    "Consignación",
    "Cheque",
    "Otro",
];

pub const CATEGORIAS_GASTO: [&str; 6] = [
    "Arriendo",
    "Servicios",
    "Nómina",
    "Mantenimiento",
    "Varios",
    "General",
];

pub fn validar_metodo_pago_venta(metodo: &str) -> Result<(), String> {
    if !METODOS_PAGO_VENTA.contains(&metodo) {
        return Err("Método de pago no válido".to_string());
    }
    Ok(())
}

pub fn validar_metodo_pago_compra(metodo: &str) -> Result<(), String> {
    if !METODOS_PAGO_COMPRA.contains(&metodo) {
        return Err("Método de pago no válido".to_string());
    }
    Ok(())
}

pub fn validar_categoria_gasto(categoria: &str) -> Result<(), String> {
    if !CATEGORIAS_GASTO.contains(&categoria) {
        return Err("Categoría de gasto no válida".to_string());
    }
    Ok(())
}

/// Estado de pago a partir del saldo pendiente y el total.
/// Los montos que llegan aquí ya vienen redondeados a 2 decimales
/// (ver `crate::services::dinero`), así que la comparación es exacta.
pub fn estado_pago(saldo_pendiente: f64, total: f64) -> &'static str {
    if saldo_pendiente <= 0.0 {
        "Pagado"
    } else if saldo_pendiente < total {
        "Parcial"
    } else {
        "Pendiente"
    }
}

/// Valida que un texto con formato "YYYY-MM" sea un mes plausible.
pub fn validar_mes(mes: &str) -> Result<(), String> {
    let partes: Vec<&str> = mes.split('-').collect();
    let valido = partes.len() == 2
        && partes[0].len() == 4
        && partes[0].chars().all(|c| c.is_ascii_digit())
        && partes[1].len() == 2
        && matches!(partes[1].parse::<u32>(), Ok(1..=12));
    if valido {
        Ok(())
    } else {
        Err("Mes no válido (use el formato YYYY-MM)".to_string())
    }
}
