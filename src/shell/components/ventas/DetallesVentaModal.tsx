import { useEffect, useState } from "react";
import { Modal } from "../common/Modal";
import { seguridad, ventas as ventasApi } from "../../lib/api";
import type { Venta, ItemVenta } from "../../types/ventas";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });
const METODOS_PAGO = ["Efectivo", "Transferencia", "Tarjeta", "Mixto", "Credito"];

type Props = {
  venta: Venta;
  onCerrar: () => void;
  onGuardado: () => Promise<void>;
};

export function DetalleVentaModal({ venta, onCerrar, onGuardado }: Props) {
  const [filas, setFilas] = useState<ItemVenta[]>([]);
  const [editando, setEditando] = useState(false);
  const [modoCodigo, setModoCodigo] = useState<"crear" | "verificar" | null>(null);
  const [codigo, setCodigo] = useState("");
  const [metodo, setMetodo] = useState(venta.metodo_pago);
  const [descuento, setDescuento] = useState(venta.descuento);
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);

  useEffect(() => {
    ventasApi.listarItems(venta.id).then(setFilas);
    setMetodo(venta.metodo_pago);
    setDescuento(venta.descuento);
  }, [venta]);

  const total = filas.reduce((acc, x) => acc + x.cantidad * x.precio_unitario, 0);
  const totalNeto = Math.max(0, total - descuento);

  const pedirEdicion = async () => {
    setError(null);
    const existe = await seguridad.existeCodigoAdmin();
    setModoCodigo(existe ? "verificar" : "crear");
  };

  const confirmarCodigo = async () => {
    setError(null);
    try {
      if (modoCodigo === "crear") {
        if (codigo.trim().length < 4) return setError("Mínimo 4 caracteres");
        await seguridad.establecerCodigoAdmin(codigo.trim());
      } else {
        const ok = await seguridad.verificarCodigoAdmin(codigo);
        if (!ok) return setError("Código incorrecto");
      }
      setModoCodigo(null);
      setEditando(true);
      setCodigo("");
    } catch (e) {
      setError(String(e));
    }
  };

  const guardarCambios = async () => {
    setGuardando(true);
    setError(null);
    try {
      await ventasApi.actualizar(venta.id, {
        items: filas.map((f) => ({ producto_id: f.producto_id, cantidad: f.cantidad, precio_unitario: f.precio_unitario })),
        descuento,
        metodo_pago: metodo,
        referencia_cliente: venta.referencia_cliente,
      });
      await onGuardado();
      onCerrar();
    } catch (e) {
      setError(String(e));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <Modal titulo={`Venta #${venta.id}`} onCerrar={onCerrar}>
      <p className="form-hint">{venta.fecha} · {venta.estado_pago}</p>

      <div className="tabla-card">
        <table className="tabla-base">
          <thead>
            <tr>
              <th>Prenda</th>
              <th>Cantidad</th>
              <th>Precio</th>
              <th>Subtotal</th>
            </tr>
          </thead>
          <tbody>
            {filas.map((f, i) => (
              <tr key={f.id}>
                <td>{f.producto_nombre}</td>
                <td>
                  {editando ? (
                    <input
                      type="number"
                      min="1"
                      value={f.cantidad}
                      onChange={(e) =>
                        setFilas(filas.map((x, n) => (n === i ? { ...x, cantidad: Number(e.target.value) } : x)))
                      }
                    />
                  ) : (
                    f.cantidad
                  )}
                </td>
                <td>
                  {editando ? (
                    <input
                      type="number"
                      min="0"
                      value={f.precio_unitario}
                      onChange={(e) =>
                        setFilas(filas.map((x, n) => (n === i ? { ...x, precio_unitario: Number(e.target.value) } : x)))
                      }
                    />
                  ) : (
                    dinero.format(f.precio_unitario)
                  )}
                </td>
                <td>{dinero.format(f.cantidad * f.precio_unitario)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {editando && (
        <div className="form-grid__cols-2" style={{ marginTop: "0.75rem" }}>
          <div>
            <label>Método de pago</label>
            <select value={metodo} onChange={(e) => setMetodo(e.target.value)}>
              {METODOS_PAGO.map((m) => (
                <option key={m}>{m}</option>
              ))}
            </select>
          </div>
          <div>
            <label>Descuento</label>
            <input type="number" min="0" value={descuento} onChange={(e) => setDescuento(Number(e.target.value))} />
          </div>
        </div>
      )}

      <div className="sale-total">
        <span>Total</span>
        <strong>{dinero.format(editando ? totalNeto : venta.total_neto)}</strong>
      </div>

      {error && <p className="form-error">{error}</p>}

      {modoCodigo && (
        <div className="abono-form">
          <label>{modoCodigo === "crear" ? "Crea el código de administrador (solo tú lo sabrás)" : "Código de administrador"}</label>
          <input type="password" autoFocus value={codigo} onChange={(e) => setCodigo(e.target.value)} />
          <div className="modal__acciones">
            <button type="button" className="btn btn--ghost" onClick={() => setModoCodigo(null)}>
              Cancelar
            </button>
            <button type="button" className="btn btn--primary" onClick={confirmarCodigo}>
              Confirmar
            </button>
          </div>
        </div>
      )}

      <div className="modal__acciones">
        <button type="button" className="btn btn--ghost" onClick={onCerrar}>
          Cerrar
        </button>
        {!editando && !modoCodigo && (
          <button type="button" className="btn btn--ghost" onClick={pedirEdicion}>
            Editar venta
          </button>
        )}
        {editando && (
          <button type="button" className="btn btn--primary" disabled={guardando} onClick={guardarCambios}>
            {guardando ? "Guardando…" : "Guardar cambios"}
          </button>
        )}
      </div>
    </Modal>
  );
}