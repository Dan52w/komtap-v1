import { invoke } from "@tauri-apps/api/core";
import type { EstadoLicencia, Usuario } from "../types/auth";
import type {
  DashboardFinanciero,
  VentasPorMetodoPago,
  ProductoCritico,
  Periodo,
} from "../types/reportes";

/**
 * Wrapper delgado sobre `invoke`. Cada comando Rust falla con un `String`
 * (ver los `.map_err(|e| e.to_string())` en el backend), así que los
 * errores que llegan aquí ya son mensajes listos para mostrar al usuario.
 */

// ===== Licencia y autenticación (HU-11) =====

export const auth = {
  estadoLicencia: () => invoke<EstadoLicencia>("estado_licencia"),

  activarLicencia: (codigo: string) =>
    invoke<EstadoLicencia>("activar_licencia", { codigo }),

  existeUsuarioRegistrado: () => invoke<boolean>("existe_usuario_registrado"),

  registrarUsuario: (params: {
    codigoMaestro: string;
    username: string;
    password: string;
    nombreCompleto?: string;
  }) =>
    invoke<Usuario>("registrar_usuario", {
      codigoMaestro: params.codigoMaestro,
      username: params.username,
      password: params.password,
      nombreCompleto: params.nombreCompleto ?? null,
    }),

  iniciarSesion: (username: string, password: string) =>
    invoke<Usuario>("iniciar_sesion", { username, password }),
};

// ===== Reportes / dashboard (HU-08) =====

export const reportes = {
  dashboardFinanciero: (periodo: Periodo) =>
    invoke<DashboardFinanciero>("dashboard_financiero", { periodo }),

  ventasPorMetodoPago: (mes: string) =>
    invoke<VentasPorMetodoPago[]>("ventas_por_metodo_pago", { mes }),

  productosStockCritico: () =>
    invoke<ProductoCritico[]>("productos_stock_critico"),
};

// ===== Proveedores (deuda global, usada en el dashboard) =====

export const proveedores = {
  deudaTotalGlobal: () => invoke<number>("deuda_total_global"),
};