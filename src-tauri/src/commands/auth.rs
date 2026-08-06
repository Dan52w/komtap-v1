use crate::db::DbConnection;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::licencia::{verificar_codigo, TipoLicencia};

// TODO: reemplazar por la lógica real de generación/validación de códigos
// (firma criptográfica, servidor de licencias, etc.) antes de distribuir.
// Al ser una app 100% offline (HU-10), por ahora la validación es local
// contra un catálogo estático de códigos válidos.
//
// Formato sugerido de código: KOMTAP-<TIPO>-<DURACION_O_PERM>-<HASH>
const CODIGO_MAESTRO_REGISTRO: &str = "KOMTAP-MASTER-0000-Z9Y8";

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

// ===== LICENCIA (primer inicio y verificación en cada apertura) =====

#[tauri::command]
pub fn estado_licencia(state: State<DbConnection>) -> Result<EstadoLicencia, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let fila: Option<(String, String, Option<String>)> = conn
        .query_row(
            "SELECT estado, tipo_licencia, fecha_expiracion FROM licencia_sistema WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .ok();

    let Some((estado, tipo_licencia, fecha_expiracion)) = fila else {
        return Ok(EstadoLicencia { activa: false, tipo_licencia: None, fecha_expiracion: None });
    };

    if estado != "activa" {
        return Ok(EstadoLicencia { activa: false, tipo_licencia: Some(tipo_licencia), fecha_expiracion });
    }

    if tipo_licencia == "permanente" {
        return Ok(EstadoLicencia { activa: true, tipo_licencia: Some(tipo_licencia), fecha_expiracion });
    }

    // Licencia temporal: verificar expiración contra la fecha/hora local del sistema.
    let vigente: bool = conn
        .query_row(
            "SELECT fecha_expiracion > datetime('now','localtime') FROM licencia_sistema WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if !vigente {
        conn.execute("UPDATE licencia_sistema SET estado = 'expirada' WHERE id = 1", [])
            .map_err(|e| e.to_string())?;
        return Ok(EstadoLicencia { activa: false, tipo_licencia: Some(tipo_licencia), fecha_expiracion });
    }

    Ok(EstadoLicencia { activa: true, tipo_licencia: Some(tipo_licencia), fecha_expiracion })
}

#[tauri::command]
pub fn activar_licencia(state: State<DbConnection>, codigo: String) -> Result<EstadoLicencia, String> {
    let payload = verificar_codigo(&codigo)?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    match payload.tipo {
        TipoLicencia::Permanente => {
            conn.execute(
                "INSERT INTO licencia_sistema (id, codigo_activacion, tipo_licencia, fecha_expiracion, estado)
                 VALUES (1, ?1, 'permanente', NULL, 'activa')
                 ON CONFLICT(id) DO UPDATE SET
                    codigo_activacion = excluded.codigo_activacion,
                    tipo_licencia = 'permanente',
                    fecha_expiracion = NULL,
                    estado = 'activa'",
                [&codigo],
            ).map_err(|e| e.to_string())?;
            Ok(EstadoLicencia { activa: true, tipo_licencia: Some("permanente".into()), fecha_expiracion: None })
        }
        TipoLicencia::Temporal => {
            let fecha_expiracion: String = conn.query_row(
                "SELECT datetime('now','localtime', ?1)",
                [format!("+{} days", payload.dias)],
                |row| row.get(0),
            ).map_err(|e| e.to_string())?;

            conn.execute(
                "INSERT INTO licencia_sistema (id, codigo_activacion, tipo_licencia, fecha_expiracion, estado)
                 VALUES (1, ?1, 'temporal', ?2, 'activa')
                 ON CONFLICT(id) DO UPDATE SET
                    codigo_activacion = excluded.codigo_activacion,
                    tipo_licencia = 'temporal',
                    fecha_expiracion = excluded.fecha_expiracion,
                    estado = 'activa'",
                (&codigo, &fecha_expiracion),
            ).map_err(|e| e.to_string())?;
            Ok(EstadoLicencia { activa: true, tipo_licencia: Some("temporal".into()), fecha_expiracion: Some(fecha_expiracion) })
        }
    }
}

// ===== REGISTRO DE USUARIO ÚNICO (se desactiva permanentemente tras el primero) =====

#[tauri::command]
pub fn existe_usuario_registrado(state: State<DbConnection>) -> Result<bool, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(total > 0)
}

#[tauri::command]
pub fn registrar_usuario(
    state: State<DbConnection>,
    codigo_maestro: String,
    username: String,
    password: String,
    nombre_completo: Option<String>,
) -> Result<Usuario, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM usuarios", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    if total > 0 {
        return Err("Ya existe un usuario registrado; el registro está desactivado".to_string());
    }

    if codigo_maestro.trim() != CODIGO_MAESTRO_REGISTRO {
        return Err("Código maestro de registro no válido".to_string());
    }

    if username.trim().is_empty() || password.is_empty() {
        return Err("El usuario y la contraseña son obligatorios".to_string());
    }

    let password_hash = hash(&password, DEFAULT_COST).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO usuarios (username, password_hash, nombre_completo) VALUES (?1, ?2, ?3)",
        (&username, &password_hash, &nombre_completo),
    ).map_err(|e| e.to_string())?;

    Ok(Usuario {
        id: conn.last_insert_rowid(),
        username,
    })
}

// ===== LOGIN (cada vez que se abre la app) =====

#[tauri::command]
pub fn iniciar_sesion(state: State<DbConnection>, username: String, password: String) -> Result<Usuario, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let (id, password_hash): (i64, String) = conn.query_row(
        "SELECT id, password_hash FROM usuarios WHERE username = ?1",
        [&username],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).map_err(|_| "Usuario o contraseña incorrectos".to_string())?;

    let valido = verify(&password, &password_hash).map_err(|e| e.to_string())?;
    if !valido {
        return Err("Usuario o contraseña incorrectos".to_string());
    }

    Ok(Usuario { id, username })
}
