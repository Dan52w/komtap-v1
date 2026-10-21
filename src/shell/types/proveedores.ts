export type Proveedor = {
  id: number | null;
  nombre: string;
  telefono: string | null;
  direccion: string | null;
  correo: string | null;
  notas: string | null;
};

export type ResumenProveedor = { 
  proveedor_id: number;
  nombre: string; 
  deuda_total: number 
};

export type FacturaProveedor = { 
  id:number; 
  proveedor_id:number; 
  numero_factura:string|null; 
  fecha_emision:string; 
  fecha_limite_pago:string|null; 
  monto_total:number; 
  saldo_pendiente:number; 
  estado_pago:string 
};

export type AbonoProveedor = { 
  id:number; 
  factura_id:number; 
  monto:number; 
  fecha:string; 
  metodo_pago:string; 
  banco:string|null; 
  numero_referencia:string|null; 
  nota:string|null 
};

export type ItemFactura = { 
  producto_id:number|null; 
  nombre_nuevo?:string|null; 
  sku_nuevo?:string|null; 
  categoria_nueva?:string|null; 
  talla_nueva?:string|null; 
  color_nuevo?:string|null; 
  cantidad:number; 
  costo_unitario:number; 
  precio_venta_sugerido?:number|null; 
  stock_minimo_nuevo?:number|null 
};

export type DetalleFactura = { 
  id:number; producto_id:number; 
  producto_nombre:string; 
  cantidad:number; 
  costo_unitario:number; 
  precio_venta_sugerido:number|null; 
  subtotal:number 
};

export type AbonoInput = {
  monto: number;
  metodo_pago: string;
  banco?: string | null;
  numero_referencia?: string | null;
  nota?: string | null;
  fecha?: string | null;
};

export type NuevaFacturaInput = {
  proveedor_id: number;
  numero_factura?: string | null;
  fecha_emision: string;
  fecha_limite_pago?: string | null;
  items: ItemFactura[];
  abono_inicial?: AbonoInput | null;
};

export type FacturaPendiente = {
  id: number;
  proveedor_id: number;
  proveedor_nombre: string;
  numero_factura: string | null;
  fecha_emision: string;
  fecha_limite_pago: string | null;
  monto_total: number;
  saldo_pendiente: number;
  estado_pago: string;
};