//! Capa de servicios: reglas de negocio y orquestación.
//!
//! Los servicios no contienen SQL (eso vive en `crate::repos`) y no conocen
//! a Tauri (eso vive en `crate::commands`). Reciben una `&Connection` (una `&Transaction` sirve igual gracias a `Deref`),
//! `&Connection`, aplican validaciones y reglas, orquestan varios
//! repositorios y devuelven `Result<T, String>` con mensajes para el usuario.

pub mod compras;
pub mod dinero;
pub mod gastos;
pub mod inventario;
pub mod licencia;
pub mod reglas;
pub mod respaldo;
pub mod seed;
pub mod usuarios;
pub mod ventas;
