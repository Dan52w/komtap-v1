use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Clave secreta de firma, inyectada en build time vía KOMTAP_LICENSE_KEY.
const CLAVE_SECRETA: &[u8] = env!("KOMTAP_LICENSE_KEY").as_bytes();

#[derive(Debug, Clone)]
pub struct PayloadLicencia {
    pub tipo: TipoLicencia,
    pub dias: u16, // 0 = permanente
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TipoLicencia {
    Permanente = 0,
    Temporal = 1,
}

/// Genera un código de activación firmado. El payload incluye un nonce
/// aleatorio de 4 bytes para que cada código sea único, incluso si se
/// generan varios con el mismo tipo y duración.
pub fn generar_codigo(tipo: TipoLicencia, dias: u16) -> String {
    let nonce: u32 = rand::thread_rng().gen();

    let mut payload = Vec::with_capacity(7);
    payload.push(tipo as u8);
    payload.extend_from_slice(&dias.to_be_bytes());
    payload.extend_from_slice(&nonce.to_be_bytes());

    let mut mac = HmacSha256::new_from_slice(CLAVE_SECRETA).expect("clave HMAC inválida");
    mac.update(&payload);
    let firma = mac.finalize().into_bytes();
    let firma_truncada = &firma[..8]; // 8 bytes = suficiente para evitar fuerza bruta práctica

    let payload_b32 = base32::encode(base32::Alphabet::Crockford, &payload);
    let firma_hex = hex::encode(firma_truncada);

    format!("KOMTAP-{payload_b32}-{firma_hex}").to_uppercase()
}

/// Verifica un código de activación. Devuelve el payload (tipo y días) si la firma es válida.
pub fn verificar_codigo(codigo: &str) -> Result<PayloadLicencia, String> {
    let codigo = codigo.trim().to_uppercase();
    let partes: Vec<&str> = codigo.split('-').collect();
    // Formato esperado: KOMTAP-<payload_b32>-<firma_hex>
    let [prefijo, payload_b32, firma_hex] = partes.as_slice() else {
        return Err("Formato de código inválido".to_string());
    };
    if *prefijo != "KOMTAP" {
        return Err("Formato de código inválido".to_string());
    }

    let payload = base32::decode(base32::Alphabet::Crockford, payload_b32)
        .ok_or("Código corrupto o ilegible")?;
    if payload.len() != 7 {
        return Err("Código corrupto o ilegible".to_string());
    }

    let firma_recibida = hex::decode(firma_hex).map_err(|_| "Código corrupto o ilegible")?;
    if firma_recibida.len() != 8 {
        return Err("Código corrupto o ilegible".to_string());
    }

    let mut mac = HmacSha256::new_from_slice(CLAVE_SECRETA).expect("clave HMAC inválida");
    mac.update(&payload);
    let firma_calculada = mac.finalize().into_bytes();

    if firma_recibida.as_slice() != &firma_calculada[..8] {
        return Err("Código de licencia no válido".to_string());
    }

    let tipo = match payload[0] {
        0 => TipoLicencia::Permanente,
        1 => TipoLicencia::Temporal,
        _ => return Err("Código corrupto o ilegible".to_string()),
    };
    let dias = u16::from_be_bytes([payload[1], payload[2]]);
    // payload[3..7] es el nonce; no se usa después de verificar la firma.

    Ok(PayloadLicencia { tipo, dias })
}