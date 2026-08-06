export type TopPrenda = {
  producto_id: number;
  nombre: string;
  unidades_vendidas: number;
  ingreso_total: number;
  margen_estimado: number;
};

export type Venta = {
  id: number; fecha: string; subtotal: number; descuento: number; total_neto: number;
  monto_pagado: number; saldo_pendiente: number; metodo_pago: string; estado_pago: string;
  referencia_cliente: string | null;
};

export type NuevaVenta = {
  items: { producto_id: number; cantidad: number; precio_unitario: number }[];
  descuento?: number; metodo_pago: string; monto_pagado?: number;
  referencia_cliente?: string | null; notas?: string | null;
};
export type ItemVenta = { id:number; producto_id:number; producto_nombre:string; cantidad:number; precio_unitario:number; subtotal:number };
