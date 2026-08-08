import { useState } from "react";
import type { AbonoInput } from "../../types/proveedores";

const METODOS_PAGO_PROVEEDOR = ["Efectivo", "Transferencia bancaria", "Consignación", "Cheque", "Otro"];
const hoy = new Date().toISOString().slice(0, 10);

type Props = {
  saldoMaximo?: number;
  valorInicial?: Partial<AbonoInput>;
  onGuardar: (abono: AbonoInput) => void | Promise<void>;
  onCancelar?: () => void;
  textoBoton?: string;
};

export function AbonoForm({ saldoMaximo, valorInicial, onGuardar, onCancelar, textoBoton = "Guardar abono" }: Props) {
  const [monto, setMonto] = useState(valorInicial?.monto ?? 0);
  const [metodo, setMetodo] = useState(valorInicial?.metodo_pago ?? "Efectivo");
  const [banco, setBanco] = useState(valorInicial?.banco ?? "");
  const [referencia, setReferencia] = useState(valorInicial?.numero_referencia ?? "");
  const [fecha, setFecha] = useState(valorInicial?.fecha ?? hoy);
  const [nota, setNota] = useState(valorInicial?.nota ?? "");
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);

  const requiereReferencia = metodo !== "Efectivo";

  const confirmar = async () => {
    setError(null);
    if (monto <= 0) return setError("El monto debe ser mayor que cero");
    if (saldoMaximo !== undefined && monto > saldoMaximo) {
      return setError(`El monto no puede superar ${saldoMaximo}`);
    }
    if (requiereReferencia && !referencia.trim()) {
      return setError("Indica un número de referencia para este método de pago");
    }

    setGuardando(true);
    try {
      await onGuardar({
        monto,
        metodo_pago: metodo,
        banco: banco.trim() || null,
        numero_referencia: referencia.trim() || null,
        fecha,
        nota: nota.trim() || null,
      });
    } catch (e) {
      setError(String(e));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <div className="abono-form">
      <h4>Abono</h4>
      <input
        type="number"
        max={saldoMaximo}
        min="1"
        step="1"
        placeholder="Ej. 50000"
        value={monto || ""}
        onChange={(e) => setMonto(Number(e.target.value))}
      />
      <select value={metodo} onChange={(e) => setMetodo(e.target.value)}>
        {METODOS_PAGO_PROVEEDOR.map((m) => (
          <option key={m}>{m}</option>
        ))}
      </select>
      {requiereReferencia && (
        <>
          <input placeholder="Banco (opcional)" value={banco} onChange={(e) => setBanco(e.target.value)} />
          <input placeholder="Referencia" value={referencia} onChange={(e) => setReferencia(e.target.value)} />
        </>
      )}
      <input type="date" value={fecha} onChange={(e) => setFecha(e.target.value)} />
      <textarea placeholder="Nota" value={nota} onChange={(e) => setNota(e.target.value)} />

      {error && <p className="form-error">{error}</p>}

      <div className="modal__acciones">
        {onCancelar && (
          <button type="button" className="btn btn--ghost" onClick={onCancelar}>
            Cancelar
          </button>
        )}
        <button type="button" className="btn btn--primary" disabled={guardando} onClick={confirmar}>
          {guardando ? "Guardando…" : textoBoton}
        </button>
      </div>
    </div>
  );
}