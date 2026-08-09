import { invoke } from "@tauri-apps/api/core";
import type { EstadoLicencia, Usuario } from "../types/auth";
import type {
  DashboardFinanciero,
  VentasPorMetodoPago,
  ProductoCritico,
  Periodo,
} from "../types/reportes";
import type { Producto, FiltroProductos } from "../types/productos";
import type { TopPrenda } from "../types/ventas";
import type { Proveedor, ResumenProveedor, FacturaProveedor, AbonoProveedor, DetalleFactura, NuevaFacturaInput, AbonoInput } from "../types/proveedores";
import type { Gasto, NuevoGasto } from "../types/gastos";
import type { Venta, NuevaVenta, ItemVenta } from "../types/ventas";

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
    codigoAdmin: string;
    nombreCompleto?: string;
  }) =>
    invoke<Usuario>("registrar_usuario", {
      codigoMaestro: params.codigoMaestro,
      username: params.username,
      password: params.password,
      codigoAdmin: params.codigoAdmin,
      nombreCompleto: params.nombreCompleto ?? null,
    }),

  iniciarSesion: (username: string, password: string) =>
    invoke<Usuario>("iniciar_sesion", { username, password }),
};

// ===== Reportes / dashboard (HU-08) =====

export const reportes = {
  dashboardFinanciero: (periodo: Periodo) => invoke<DashboardFinanciero>("dashboard_financiero", { periodo }),
  dashboardFinancieroMes: (mes: string) => invoke<DashboardFinanciero>("dashboard_financiero_mes", { mes }),
  ventasPorMetodoPago: (mes: string) => invoke<VentasPorMetodoPago[]>("ventas_por_metodo_pago", { mes }),
  productosStockCritico: () => invoke<ProductoCritico[]>("productos_stock_critico"),
};

// ===== Proveedores (deuda global, usada en el dashboard) =====

export const proveedores = {
  deudaTotalGlobal: () => invoke<number>("deuda_total_global"),
  listar: () => invoke<Proveedor[]>("listar_proveedores"),
  crear: (proveedor: Proveedor) => invoke<number>("crear_proveedor", { proveedor }),
  actualizar: (proveedor: Proveedor) => invoke<void>("actualizar_proveedor", { proveedor }),
  eliminar: (proveedorId: number) => invoke<void>("eliminar_proveedor", { proveedorId }),
  resumen: () => invoke<ResumenProveedor[]>("resumen_por_proveedor"),
  listarFacturas: (proveedorId: number) => invoke<FacturaProveedor[]>("listar_facturas_proveedor", { proveedorId }),
  // antes mandaba abono_inicial: null siempre — ahora sí se envía lo que capture el modal
  crearFactura: (factura: NuevaFacturaInput) =>
    invoke<number>("crear_factura_proveedor", {
      factura: { ...factura, abono_inicial: factura.abono_inicial ?? null },
    }),
  listarAbonos: (facturaId: number) => invoke<AbonoProveedor[]>("listar_abonos_factura_proveedor", { facturaId }),
  listarItemsFactura: (facturaId: number) => invoke<DetalleFactura[]>("listar_items_factura_proveedor", { facturaId }),
  // antes mandaba banco: null siempre — ahora viene del formulario
  registrarAbono: (facturaId: number, abono: AbonoInput) =>
    invoke<number>("registrar_abono_proveedor", {
      facturaId,
      abono: { ...abono, banco: abono.banco ?? null },
    }),
};

// ===== Inventario / productos (HU-04) =====

export const productos = {
  listar: (filtro?: FiltroProductos) =>
    invoke<Producto[]>("listar_productos", { filtro: filtro ?? null }),

  crear: (producto: Producto) => invoke<number>("crear_producto", { producto }),

  actualizar: (producto: Producto) =>
    invoke<void>("actualizar_producto", { producto }),

  ajustarStock: (params: { productoId: number; delta: number; motivo: string }) =>
    invoke<number>("ajustar_stock_manual", params),

  eliminar: (productoId: number) =>
    invoke<void>("eliminar_producto", { productoId }),
};

// ===== Ventas (solo el ranking por ahora, HU-05) =====

export const ventas = {
  topPrendas: (params: { dias: number | null; limite?: number }) =>
    invoke<TopPrenda[]>("top_prendas_vendidas", {
      dias: params.dias,
      limite: params.limite ?? null,
    }),
  listar: (params?: { mes?: string; limite?: number; offset?: number }) =>
    invoke<Venta[]>("listar_ventas", {
      limite: params?.limite ?? 50,
      offset: params?.offset ?? 0,
      mes: params?.mes ?? null,
    }),
  crear: (venta: NuevaVenta) => invoke<number>("crear_venta", { venta }),
  listarItems: (ventaId: number) => invoke<ItemVenta[]>("listar_items_venta", { ventaId }),
  actualizar: (ventaId: number, venta: NuevaVenta) => invoke<void>("actualizar_venta", { ventaId, venta }),
  eliminar: (ventaId: number) => invoke<void>("eliminar_venta", { ventaId }),
};

export const gastos = {
  listar: (mes: string) => invoke<Gasto[]>("listar_gastos", { mes, limite: 500, offset: 0 }),
  crear: (gasto: NuevoGasto) => invoke<number>("crear_gasto", { gasto }),
  eliminar: (gastoId: number) => invoke<void>("eliminar_gasto", { gastoId }),
};

export const seguridad = {
  existeCodigoAdmin: () => invoke<boolean>("existe_codigo_admin"),
  establecerCodigoAdmin: (codigo: string) => invoke<void>("establecer_codigo_admin", { codigo }),
  verificarCodigoAdmin: (codigo: string) => invoke<boolean>("verificar_codigo_admin", { codigo }),
};
