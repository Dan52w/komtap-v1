//! Reglas de negocio del licenciamiento.
//!
//! El mecanismo criptográfico (generación y verificación de códigos) vive en
//! `crate::licencia` y no se toca: aquí solo está el flujo de activación y
//! la lectura del estado.

use crate::licencia::{verificar_codigo, TipoLicencia};
use crate::models::EstadoLicencia;
use crate::repos::auth;
use rusqlite::Connection;

/// Estado actual de la licencia (para verificar en cada apertura).
pub fn estado(conn: &Connection) -> Result<EstadoLicencia, String> {
    let fila = auth::obtener_licencia(conn).map_err(crate::errors::bd)?;
    let temporal_vigente = match &fila {
        Some((estado, tipo, _)) if estado == "activa" && tipo == "temporal" => {
            let vigente = auth::licencia_temporal_vigente(conn).map_err(crate::errors::bd)?;
            if !vigente {
                auth::marcar_licencia_expirada(conn).map_err(crate::errors::bd)?;
            }
            vigente
        }
        _ => false,
    };
    Ok(auth::estado_desde_fila(fila, temporal_vigente))
}

/// Activa una licencia validando la firma del código y bloqueando su reuso:
/// cada código, sin importar su tipo, solo puede canjearse una vez en la
/// vida (queda registrado en `codigos_activacion_usados`), evitando que un
/// código temporal expirado se reactive infinitamente.
pub fn activar(conn: &Connection, codigo: &str) -> Result<EstadoLicencia, String> {
    let payload = verificar_codigo(codigo)?;
    let codigo_normalizado = codigo.trim().to_uppercase();

    if auth::codigo_ya_usado(conn, &codigo_normalizado).map_err(crate::errors::bd)? {
        return Err(
            "Este código de activación ya fue utilizado y no puede reutilizarse.".to_string(),
        );
    }

    let (tipo, fecha_expiracion) = match payload.tipo {
        TipoLicencia::Permanente => ("permanente", None),
        TipoLicencia::Temporal => {
            let fecha = auth::calcular_expiracion(conn, payload.dias).map_err(crate::errors::bd)?;
            ("temporal", Some(fecha))
        }
    };

    auth::guardar_licencia(conn, &codigo_normalizado, tipo, &fecha_expiracion)
        .map_err(crate::errors::bd)?;
    auth::marcar_codigo_usado(conn, &codigo_normalizado).map_err(crate::errors::bd)?;

    Ok(EstadoLicencia {
        activa: true,
        tipo_licencia: Some(tipo.to_string()),
        fecha_expiracion,
    })
}
