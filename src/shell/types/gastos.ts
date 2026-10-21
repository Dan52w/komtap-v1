export type Gasto = { id: number; descripcion: string; monto: number; fecha: string; categoria: string; metodo_pago: string };
export type NuevoGasto = {
  descripcion: string;
  monto: number;
  fecha?: string | null;
  categoria?: string | null;
  metodo_pago?: string | null;
};
export type GastoActualizado = {
  descripcion: string;
  monto: number;
  fecha: string;
  categoria: string;
  metodo_pago: string;
};

export const CATEGORIAS_GASTO = [
  "Arriendo",
  "Servicios",
  "Nómina",
  "Mantenimiento",
  "Varios",
  "General",
] as const;
