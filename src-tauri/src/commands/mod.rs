//! Comandos Tauri: adaptadores delgados entre el frontend y los servicios.
//!
//! Cada comando abre la conexión (y una transacción si la operación escribe
//! en varias tablas), delega la lógica a `crate::services` y confirma.
//! Ningún comando contiene SQL: eso vive en `crate::repos`.

pub mod auth;
pub mod clientes;
pub mod gastos;
pub mod productos;
pub mod proveedores;
pub mod reportes;
pub mod respaldo;
pub mod seed;
pub mod seguridad;
pub mod ventas;
