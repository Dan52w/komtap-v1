import type { Cliente } from "../../types/clientes";

const METODOS_PAGO = ["Efectivo", "Transferencia", "Tarjeta", "Mixto", "Credito"];

type Props = {
  descuento: number;
  metodo: string;
  cliente: string;
  clienteId: number | null;
  clientes: Cliente[];
  onCambiarDescuento: (v: number) => void;
  onCambiarMetodo: (v: string) => void;
  onCambiarCliente: (v: string) => void;
  onCambiarClienteId: (v: number | null) => void;
};

export function DatosVenta({
  descuento,
  metodo,
  cliente,
  clienteId,
  clientes,
  onCambiarDescuento,
  onCambiarMetodo,
  onCambiarCliente,
  onCambiarClienteId,
}: Props) {
  return (
    <div className="sale-details">
      <label>
        Descuento
        <input type="number" min="0" step="1" placeholder="Ej. 5000" value={descuento || ""}
          onChange={(e) => onCambiarDescuento(Number(e.target.value))}/>
      </label>
      <label>
        Método
        <select value={metodo} onChange={(e) => onCambiarMetodo(e.target.value)}>
          {METODOS_PAGO.map((m) => (
            <option key={m}>{m}</option>
          ))}
        </select>
      </label>
      <label>
        Cliente registrado (opcional)
        <select
          value={clienteId ?? ""}
          onChange={(e) => onCambiarClienteId(e.target.value ? Number(e.target.value) : null)}
        >
          <option value="">Sin cliente registrado</option>
          {clientes.map((c) => (
            <option key={c.id} value={c.id!}>{c.nombre}</option>
          ))}
        </select>
      </label>
      {!clienteId && (
        <label>
          Referencia (opcional)
          <input value={cliente} onChange={(e) => onCambiarCliente(e.target.value)} placeholder="Nombre o teléfono" />
        </label>
      )}
    </div>
  );
}
