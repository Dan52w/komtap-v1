//! Semilla de datos de ejemplo: clientes, proveedores y productos de una
//! tienda de ropa ficticia. Solo se puede ejecutar sobre tablas vacías, para
//! no duplicar datos si se corre más de una vez.

use crate::models::{Cliente, Producto, Proveedor, ResumenSeed};
use crate::repos::{clientes, productos, proveedores};
use rusqlite::Connection;

fn cliente(nombre: &str, telefono: &str, notas: Option<&str>) -> Cliente {
    Cliente {
        id: None,
        nombre: nombre.to_string(),
        telefono: Some(telefono.to_string()),
        notas: notas.map(str::to_string),
    }
}

fn proveedor(
    nombre: &str,
    telefono: Option<&str>,
    direccion: Option<&str>,
    correo: Option<&str>,
    notas: Option<&str>,
) -> Proveedor {
    Proveedor {
        id: None,
        nombre: nombre.to_string(),
        telefono: telefono.map(str::to_string),
        direccion: direccion.map(str::to_string),
        correo: correo.map(str::to_string),
        notas: notas.map(str::to_string),
    }
}

#[allow(clippy::too_many_arguments)]
fn producto(
    sku: &str,
    nombre: &str,
    categoria: &str,
    talla: &str,
    color: &str,
    costo: f64,
    precio: f64,
    stock: i64,
) -> Producto {
    Producto {
        id: None,
        sku: Some(sku.to_string()),
        nombre: nombre.to_string(),
        categoria: categoria.to_string(),
        talla: talla.to_string(),
        color: Some(color.to_string()),
        costo_unitario: costo,
        precio_venta: precio,
        stock_actual: stock,
        stock_minimo: 5,
        estado: "activo".to_string(),
    }
}

/// Carga los datos de ejemplo. Falla si alguna de las tres tablas ya tiene
/// registros, para no mezclar datos reales con datos de prueba.
pub fn cargar_datos_ejemplo(conn: &Connection) -> Result<ResumenSeed, String> {
    let hay_clientes = clientes::contar(conn).map_err(crate::errors::bd)?;
    let hay_proveedores = proveedores::contar(conn).map_err(crate::errors::bd)?;
    let hay_productos = productos::contar(conn).map_err(crate::errors::bd)?;
    if hay_clientes + hay_proveedores + hay_productos > 0 {
        return Err(
            "Ya hay datos registrados: la semilla de ejemplo solo funciona sobre una base vacía."
                .to_string(),
        );
    }

    let lista_clientes = vec![
        cliente("María Pérez", "300 123 4567", Some("Paga quincenal")),
        cliente("Carlos Mendoza", "301 987 6543", Some("Cliente frecuente")),
        cliente("Ana Lucía Torres", "302 456 7890", None),
        cliente("Jorge Ramírez", "315 234 5678", Some("Compra al por mayor")),
        cliente("Laura Castillo", "320 876 5432", None),
        cliente("Diego Fernández", "311 345 6789", Some("Prefiere transferencia")),
        cliente("Sofía Vargas", "304 567 8901", None),
        cliente("Andrés Gutiérrez", "318 901 2345", Some("Talla M")),
    ];
    for c in &lista_clientes {
        clientes::insertar(conn, c).map_err(crate::errors::bd)?;
    }

    let lista_proveedores = vec![
        proveedor(
            "Distribuidora Andina",
            Some("601 234 5678"),
            Some("Calle 45 #12-30, Bogotá"),
            Some("contacto@distribuidoraandina.co"),
            Some("Jeans y ropa casual"),
        ),
        proveedor(
            "Textiles del Caribe",
            Some("605 432 1098"),
            Some("Av. Libertador #25-40, Santa Marta"),
            Some("ventas@textilescaribe.com"),
            Some("Ropa fresca para clima cálido"),
        ),
        proveedor(
            "Confecciones Medellín",
            Some("604 567 1234"),
            Some("Carrera 50 #30-15, Medellín"),
            Some("pedidos@confeccionesmed.com"),
            None,
        ),
        proveedor(
            "Importadora Pacífico",
            Some("602 345 6789"),
            Some("Calle 15 #8-22, Cali"),
            Some("info@importpacifico.com"),
            Some("Ropa deportiva importada"),
        ),
        proveedor(
            "Moda al Mayor S.A.S.",
            Some("601 876 5432"),
            None,
            Some("mayormoda@modaalmayor.co"),
            Some("Precios por volumen"),
        ),
    ];
    for p in &lista_proveedores {
        proveedores::insertar_proveedor(conn, p).map_err(crate::errors::bd)?;
    }

    let lista_productos = vec![
        producto("CAM-BLA-M", "Camiseta básica blanca", "Camisetas", "M", "Blanco", 18000.0, 35000.0, 45),
        producto("CAM-NEG-L", "Camiseta básica negra", "Camisetas", "L", "Negro", 18000.0, 35000.0, 38),
        producto("CAM-EST-S", "Camiseta estampada", "Camisetas", "S", "Azul", 22000.0, 42000.0, 12),
        producto("JEA-SKN-28", "Jean skinny dama", "Pantalones", "28", "Azul oscuro", 45000.0, 89900.0, 25),
        producto("JEA-CLA-32", "Jean clásico caballero", "Pantalones", "32", "Azul", 48000.0, 95000.0, 30),
        producto("PAN-DRI-30", "Pantalón dril beige", "Pantalones", "30", "Beige", 40000.0, 79900.0, 8),
        producto("VES-FLO-M", "Vestido floral verano", "Vestidos", "M", "Multicolor", 55000.0, 110000.0, 15),
        producto("VES-NEG-S", "Vestido negro cóctel", "Vestidos", "S", "Negro", 60000.0, 120000.0, 6),
        producto("FAL-JEA-M", "Falda jean", "Faldas", "M", "Azul", 35000.0, 69900.0, 10),
        producto("BLU-BLA-M", "Blusa blanca elegante", "Blusas", "M", "Blanco", 38000.0, 75000.0, 20),
        producto("BLU-EST-L", "Blusa estampada", "Blusas", "L", "Rojo", 36000.0, 72000.0, 0),
        producto("CHA-JEA-M", "Chaqueta jean", "Chaquetas", "M", "Azul", 70000.0, 139900.0, 9),
        producto("CHA-IMP-L", "Chaqueta impermeable", "Chaquetas", "L", "Negro", 85000.0, 169900.0, 4),
        producto("ACC-GOR-U", "Gorra plana", "Accesorios", "Única", "Negro", 15000.0, 30000.0, 50),
        producto("ACC-CIN-M", "Cinturón cuero", "Accesorios", "M", "Marrón", 25000.0, 49900.0, 22),
        producto("CAL-TEN-40", "Tenis deportivos", "Calzado", "40", "Blanco", 90000.0, 179900.0, 14),
        producto("CAL-SAN-37", "Sandalias playeras", "Calzado", "37", "Dorado", 30000.0, 59900.0, 18),
        producto("TDB-ENT-M", "Vestido de baño enterizo", "Trajes de baño", "M", "Turquesa", 50000.0, 99900.0, 11),
        producto("DEP-SHO-L", "Short deportivo", "Ropa deportiva", "L", "Gris", 28000.0, 54900.0, 26),
        producto("DEP-LEG-M", "Leggins deportivos", "Ropa deportiva", "M", "Negro", 32000.0, 64900.0, 33),
        producto("DOR-PIJ-M", "Pijama algodón", "Ropa de dormir", "M", "Rosado", 40000.0, 79900.0, 7),
        producto("INT-CAM-L", "Camiseta interior pack x3", "Ropa interior", "L", "Blanco", 30000.0, 59900.0, 40),
        producto("JEA-BOT-26", "Jean bota campana", "Pantalones", "26", "Azul claro", 47000.0, 92900.0, 3),
        producto("CHA-BLA-M", "Blazer elegante", "Chaquetas", "M", "Gris", 95000.0, 189900.0, 5),
    ];
    for p in &lista_productos {
        productos::insertar(conn, p).map_err(crate::errors::bd)?;
    }

    Ok(ResumenSeed {
        clientes: lista_clientes.len() as i64,
        proveedores: lista_proveedores.len() as i64,
        productos: lista_productos.len() as i64,
    })
}
