//! Capa de acceso a datos (repositorios).
//!
//! Todo el SQL de la aplicación vive en estos módulos. Los comandos y los
//! servicios nunca escriben SQL directamente: piden datos a través de estas
//! funciones, que reciben `&rusqlite::Connection` (gracias a `Deref`, un
//! `&Transaction` sirve igual) y devuelven tipos de `crate::models`.

pub mod auth;
pub mod clientes;
pub mod gastos;
pub mod productos;
pub mod proveedores;
pub mod reportes;
pub mod seguridad;
pub mod ventas;
