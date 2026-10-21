//! Registro e inicio de sesión del usuario administrador único.
//!
//! El registro se desactiva permanentemente tras el primer usuario; no hay
//! ningún código maestro ni secreto hardcodeado: la única puerta es que aún
//! no exista ningún usuario en la base de datos local.

use crate::models::Usuario;
use crate::repos::{auth, seguridad};
use bcrypt::{hash, verify, DEFAULT_COST};
use rusqlite::Connection;

pub fn existe_registrado(conn: &rusqlite::Connection) -> Result<bool, String> {
    Ok(auth::contar_usuarios(conn).map_err(crate::errors::bd)? > 0)
}

pub fn registrar(
    conn: &Connection,
    username: &str,
    password: &str,
    codigo_admin: &str,
    nombre_completo: Option<String>,
) -> Result<Usuario, String> {
    if auth::contar_usuarios(conn).map_err(crate::errors::bd)? > 0 {
        return Err("Ya existe un usuario registrado; el registro está desactivado".to_string());
    }

    let username = username.trim();
    if username.is_empty() || password.is_empty() {
        return Err("El usuario y la contraseña son obligatorios".to_string());
    }
    if username.len() > 64 {
        return Err("El nombre de usuario es demasiado largo".to_string());
    }
    if password.len() < 4 {
        return Err("La contraseña debe tener al menos 4 caracteres".to_string());
    }
    if codigo_admin.trim().len() < 4 {
        return Err("El código de seguridad debe tener al menos 4 caracteres".to_string());
    }

    let password_hash = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    let codigo_admin_hash =
        hash(codigo_admin.trim(), DEFAULT_COST).map_err(|e| e.to_string())?;

    let id = auth::insertar_usuario(conn, username, &password_hash, &nombre_completo)
        .map_err(|e| crate::errors::base_datos(e, None))?;
    seguridad::guardar_hash(conn, &codigo_admin_hash).map_err(crate::errors::bd)?;

    Ok(Usuario {
        id,
        username: username.to_string(),
    })
}

pub fn iniciar_sesion(
    conn: &rusqlite::Connection,
    username: &str,
    password: &str,
) -> Result<Usuario, String> {
    let (id, password_hash) = auth::obtener_credencial(conn, username.trim())
        .map_err(crate::errors::bd)?
        .ok_or_else(|| "Usuario o contraseña incorrectos".to_string())?;

    let valido = verify(password, &password_hash).map_err(|e| e.to_string())?;
    if !valido {
        return Err("Usuario o contraseña incorrectos".to_string());
    }

    Ok(Usuario {
        id,
        username: username.trim().to_string(),
    })
}
