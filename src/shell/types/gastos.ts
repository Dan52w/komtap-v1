export type Gasto = { id: number; descripcion: string; monto: number; fecha: string; categoria: string; metodo_pago: string };
export type NuevoGasto = {
  descripcion: string;
  monto: number;
  fecha?: string | null;
  categoria?: string | null;
  metodo_pago?: string | null;
};
