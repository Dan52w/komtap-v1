export type EstadoLicencia = {
  activa: boolean;
  tipo_licencia: string | null;
  fecha_expiracion: string | null;
};

export type Usuario = {
  id: number;
  username: string;
};