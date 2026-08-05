import { FormEvent, useState } from "react";
import { Modal } from "../common/Modal";
import type { Producto } from "../../types/productos";

type Props = {
  producto: Producto;
  onAjustar: (delta: number, motivo: string) => Promise<void>;
  onCerrar: () => void;
};

export function AjusteStockModal({ producto, onAjustar, onCerrar }: Props) {
  const [tipo, setTipo] = useState<"entrada" | "salida">("salida");
  const [cantidad, setCantidad] = useState(1);
  const [motivo, setMotivo] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    if (cantidad <= 0) {
      setError("La cantidad debe ser mayor que cero");
      return;
    }
    if (!motivo.trim()) {
      setError("Indica un motivo (pérdida, prenda defectuosa, corrección…)");
      return;
    }
    const delta = tipo === "entrada" ? cantidad : -cantidad;
    setGuardando(true);
    try {
      await onAjustar(delta, motivo.trim());
    } catch (err) {
      setError(String(err));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <Modal titulo={`Ajustar stock — ${producto.nombre}`} onCerrar={onCerrar}>
      <form className="form-grid" onSubmit={handleSubmit}>
        <p className="form-hint">
          Stock actual: <strong>{producto.stock_actual} u.</strong>
        </p>

        <div className="form-grid__cols-2">
          <div>
            <label>Tipo de ajuste</label>
            <select value={tipo} onChange={(e) => setTipo(e.target.value as "entrada" | "salida")}>
              <option value="salida">Restar (pérdida, defecto…)</option>
              <option value="entrada">Sumar (corrección)</option>
            </select>
          </div>
          <div>
            <label>Cantidad</label>
            <input
              type="number"
              min={1}
              value={cantidad}
              onChange={(e) => setCantidad(Number(e.target.value))}
              required
            />
          </div>
        </div>

        <label>Motivo</label>
        <input
          value={motivo}
          onChange={(e) => setMotivo(e.target.value)}
          placeholder="Ej. prenda dañada en bodega"
          required
        />

        {error && <p className="form-error">{error}</p>}

        <div className="modal__acciones">
          <button type="button" className="btn btn--ghost" onClick={onCerrar}>
            Cancelar
          </button>
          <button type="submit" className="btn btn--primary" disabled={guardando}>
            {guardando ? "Guardando…" : "Aplicar ajuste"}
          </button>
        </div>
      </form>
    </Modal>
  );
}