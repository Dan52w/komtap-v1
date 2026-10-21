//! Manejo de dinero: todo valor monetario se redondea a 2 decimales
//! (centavos) antes de guardarse, para evitar "polvo" de punto flotante
//! en saldos y comparaciones (ej. un saldo de 0.0000001 que nunca llega a cero).

/// Redondea a 2 decimales.
pub fn redondear2(valor: f64) -> f64 {
    (valor * 100.0).round() / 100.0
}

/// Subtotal de una línea: cantidad × precio unitario, redondeado.
pub fn subtotal_linea(cantidad: i64, precio_unitario: f64) -> f64 {
    redondear2(cantidad as f64 * precio_unitario)
}
