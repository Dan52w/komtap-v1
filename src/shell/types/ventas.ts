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
  cliente_id: number | null; cliente_nombre: string | null;
  referencia_cliente: string | null;
};

export type NuevaVenta = {
  items: { producto_id: number; cantidad: number; precio_unitario: number }[];
  descuento?: number; metodo_pago: string; monto_pagado?: number;
  cliente_id?: number | null;
  referencia_cliente?: string | null; notas?: string | null;
};
export type ItemVenta = { id:number; producto_id:number; producto_nombre:string; cantidad:number; precio_unitario:number; subtotal:number };

export type AbonoVentaInput = {
  monto: number;
  metodo_pago: string;
  numero_referencia?: string | null;
  nota?: string | null;
};

export type AbonoVenta = {
  id: number; venta_id: number; monto: number; fecha: string;
  metodo_pago: string; numero_referencia: string | null; nota: string | null;
};
