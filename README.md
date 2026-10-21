# Komtap

Sistema de gestión para tienda de ropa: proveedores, compras, inventario, ventas (POS),
clientes, gastos operativos, contabilidad, reportes y respaldos. Construido con
Tauri + React + TypeScript, 100% offline con SQLite local.

## Módulos
1. Proveedores y Compras de Mercancía (facturas, abonos, cuentas por pagar)
2. Catálogo e Inventario de Prendas (stock, ajustes auditados, alertas)
3. Ventas y Caja (POS, ventas a crédito con abonos, comprobantes PDF)
4. Clientes (para identificar las cuentas por cobrar)
5. Gastos operativos
6. Contabilidad, Flujo de Caja y Reportes PDF
7. Ajustes: licencia, código de seguridad y respaldo/restauración de la BD
8. Licenciamiento y Autenticación

## Arquitectura del backend (`src-tauri/src`)

```
commands/   → adaptadores Tauri: validan la entrada, abren la transacción
              y delegan. No contienen SQL.
services/   → reglas de negocio y orquestación (totales, estados de pago,
              validaciones, activación de licencia, respaldo).
repos/      → acceso a datos. Todo el SQL de la app vive aquí.
models.rs   → DTOs compartidos entre comandos, servicios y repositorios.
errors.rs   → mensajes de error centralizados (listos para el usuario).
licencia.rs → generación/verificación criptográfica de códigos (HMAC-SHA256).
db.rs       → conexión + migraciones idempotentes (001, 002, …).
```

Principios aplicados: cada capa tiene una sola responsabilidad (SRP), los
comandos dependen de abstracciones de servicio y no de SQL (DIP), las reglas
de negocio están aisladas y son extensibles sin modificar lo existente (OCP).

## Desarrollo

```bash
pnpm install
pnpm tauri dev
```

## Licencias

Los códigos de activación se generan con el binario `generar_licencia`:

```powershell
# Compila el generador (requiere KOMTAP_LICENSE_KEY en el entorno)
.\generar-licencias.ps1
.\generar-licencias.ps1 -CantidadPermanentes 50 -CantidadTemporales 200 -DiasTemporal 90
```

La clave HMAC se inyecta en build time vía la variable `KOMTAP_LICENSE_KEY`.

## Recommended IDE Setup
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
