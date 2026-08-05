export type DashboardFinanciero = {
  dinero_ingresado: number;
  dinero_gastado: number;
  ganancia_bruta: number;
  ganancia_neta_real: number;
  valor_inventario: number;
};

export type VentasPorMetodoPago = {
  metodo_pago: string;
  total: number;
};

export type ProductoCritico = {
  id: number;
  nombre: string;
  talla: string;
  stock_actual: number;
  stock_minimo: number;
  estado_stock: "Agotado" | "Stock Bajo";
};

export type GastoOperativoReporte = {
  descripcion: string;
  monto: number;
  fecha: string;
  categoria: string;
};

export type Periodo = "dia" | "mes" | "anio";