export type Producto = {
  id: number | null;
  sku: string | null;
  nombre: string;
  categoria: string;
  talla: string;
  color: string | null;
  costo_unitario: number;
  precio_venta: number;
  stock_actual: number;
  stock_minimo: number;
  estado: string; // "activo" | "inactivo"
};

export type EstadoStockFiltro = "agotado" | "bajo" | "disponible";

export type FiltroProductos = {
  categoria?: string | null;
  talla?: string | null;
  nombre?: string | null;
  estado_stock?: EstadoStockFiltro | null;
};