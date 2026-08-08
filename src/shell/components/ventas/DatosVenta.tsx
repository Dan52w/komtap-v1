const METODOS_PAGO = ["Efectivo", "Transferencia", "Tarjeta", "Mixto", "Credito"];

type Props = {
  descuento: number;
  metodo: string;
  cliente: string;
  onCambiarDescuento: (v: number) => void;
  onCambiarMetodo: (v: string) => void;
  onCambiarCliente: (v: string) => void;
};

export function DatosVenta({
  descuento,
  metodo,
  cliente,
  onCambiarDescuento,
  onCambiarMetodo,
  onCambiarCliente,
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
        Cliente (opcional)
        <input value={cliente} onChange={(e) => onCambiarCliente(e.target.value)} placeholder="Nombre o teléfono" />
      </label>
    </div>
  );
}