import { FormEvent, useState } from "react";
import { Modal } from "../common/Modal";
import { CATEGORIAS_GASTO, type Gasto, type GastoActualizado, type NuevoGasto } from "../../types/gastos";

const METODOS_PAGO = ["Efectivo", "Transferencia", "Tarjeta", "Otro"];

type Props = {
  inicial?: Gasto | null;
  onCerrar: () => void;
  onGuardar: (gasto: NuevoGasto | GastoActualizado) => Promise<void>;
};

export function GastoFormModal({ inicial, onCerrar, onGuardar }: Props) {
  const [descripcion, setDescripcion] = useState(inicial?.descripcion ?? "");
  const [monto, setMonto] = useState(inicial?.monto ?? 0);
  const [fecha, setFecha] = useState((inicial?.fecha ?? new Date().toISOString()).slice(0, 10));
  const [categoria, setCategoria] = useState(inicial?.categoria ?? "General");
  const [metodoPago, setMetodoPago] = useState(inicial?.metodo_pago ?? "Efectivo");
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    if (!descripcion.trim() || monto <= 0) {
      setError("Completa una descripción y un monto válido");
      return;
    }
    setGuardando(true);
    try {
      await onGuardar({ descripcion: descripcion.trim(), monto, fecha, categoria, metodo_pago: metodoPago });
    } catch (err) {
      setError(String(err));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <Modal titulo={inicial ? "Editar gasto" : "Registrar gasto"} onCerrar={onCerrar}>
      <form className="form-grid" onSubmit={handleSubmit}>
        <label>Descripción</label>
        <input autoFocus value={descripcion} onChange={(e) => setDescripcion(e.target.value)} />

        <div className="form-grid__cols-2">
          <div>
            <label>Monto</label>
            <input type="number" min="1" step="1" placeholder="Ej. 50000" value={monto || ""} onChange={(e) => setMonto(Number(e.target.value))}            />
          </div>
          <div>
            <label>Fecha</label>
            <input type="date" value={fecha} onChange={(e) => setFecha(e.target.value)} />
          </div>
        </div>

        <div className="form-grid__cols-2">
          <div>
            <label>Categoría</label>
            <select value={categoria} onChange={(e) => setCategoria(e.target.value)}>
              {CATEGORIAS_GASTO.map((c) => (
                <option key={c}>{c}</option>
              ))}
            </select>
          </div>
          <div>
            <label>Método de pago</label>
            <select value={metodoPago} onChange={(e) => setMetodoPago(e.target.value)}>
              {METODOS_PAGO.map((m) => (
                <option key={m}>{m}</option>
              ))}
            </select>
          </div>
        </div>

        {error && <p className="form-error">{error}</p>}

        <div className="modal__acciones">
          <button type="button" className="btn btn--ghost" onClick={onCerrar}>
            Cancelar
          </button>
          <button className="btn btn--primary" disabled={guardando}>
            {guardando ? "Guardando…" : inicial ? "Guardar cambios" : "Registrar"}
          </button>
        </div>
      </form>
    </Modal>
  );
}
