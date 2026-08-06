fn main() {
    let args: Vec<String> = std::env::args().collect();
    let codigo = match args.get(1).map(String::as_str) {
        Some("permanente") => komtap_lib::licencia::generar_codigo(komtap_lib::licencia::TipoLicencia::Permanente, 0),
        Some("temporal") => {
            let dias: u16 = args.get(2).expect("falta el número de días").parse().expect("días inválido");
            komtap_lib::licencia::generar_codigo(komtap_lib::licencia::TipoLicencia::Temporal, dias)
        }
        _ => {
            eprintln!("Uso: generar_licencia <permanente|temporal> [dias]");
            std::process::exit(1);
        }
    };
    println!("{codigo}");
}