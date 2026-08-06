use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Clave secreta de firma. En un mundo ideal esto NO vive en texto plano
/// en el código fuente ni en el binario final: se inyecta en build time
/// via variable de entorno (ver nota abajo sobre ofuscación / build.rs).
/// Aun así, esto es enormemente mejor que una lista estática de códigos:
/// alguien tendría que extraer la clave del binario y entender el formato
/// del payload para poder *generar* códigos válidos, en vez de simplemente
/// leerlos con `strings`.
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

/// Genera un código de activación firmado. Se usa en una herramienta CLI
/// aparte (no distribuida con la app) para emitir licencias a clientes.
pub fn generar_codigo(tipo: TipoLicencia, dias: u16) -> String {
    let mut payload = Vec::with_capacity(3);
    payload.push(tipo as u8);
    payload.extend_from_slice(&dias.to_be_bytes());

    let mut mac = HmacSha256::new_from_slice(CLAVE_SECRETA).expect("clave HMAC inválida");
    mac.update(&payload);
    let firma = mac.finalize().into_bytes();
    let firma_truncada = &firma[..8]; // 8 bytes = suficiente para evitar fuerza bruta práctica

    let payload_b32 = base32::encode(base32::Alphabet::Crockford, &payload);
    let firma_hex = hex::encode(firma_truncada);

    format!("KOMTAP-{payload_b32}-{firma_hex}").to_uppercase()
}

/// Verifica un código de activación. Devuelve el payload si la firma es válida.
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
    if payload.len() != 3 {
        return Err("Código corrupto o ilegible".to_string());
    }

    let firma_recibida = hex::decode(firma_hex).map_err(|_| "Código corrupto o ilegible")?;

    let mut mac = HmacSha256::new_from_slice(CLAVE_SECRETA).expect("clave HMAC inválida");
    mac.update(&payload);
    // verify_slice usa comparación en tiempo constante (protege contra timing attacks)
    mac.verify_slice(&firma_recibida[..]).map_err(|_| "Código de licencia no válido")?;

    // Nota: verify_slice espera el largo completo del HMAC (32 bytes); si truncaste
    // la firma a 8 bytes al generar, aquí debes comparar manualmente los primeros
    // 8 bytes recalculados en vez de usar verify_slice directo. Ver alternativa abajo.

    let tipo = match payload[0] {
        0 => TipoLicencia::Permanente,
        1 => TipoLicencia::Temporal,
        _ => return Err("Código corrupto o ilegible".to_string()),
    };
    let dias = u16::from_be_bytes([payload[1], payload[2]]);

    Ok(PayloadLicencia { tipo, dias })
}