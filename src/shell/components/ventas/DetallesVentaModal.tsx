import { useEffect, useState } from "react";
import { Modal } from "../common/Modal";
import { seguridad, ventas as ventasApi } from "../../lib/api";
import { escapar, imprimirDocumento } from "../../lib/imprimir";
import type { Venta, ItemVenta } from "../../types/ventas";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });
const METODOS_PAGO = ["Efectivo", "Transferencia", "Tarjeta", "Mixto", "Credito"];

type Props = {
  venta: Venta;
  onCerrar: () => void;
  onGuardado: () => Promise<void>;
};

type AccionProtegida = "editar" | "eliminar";

export function DetalleVentaModal({ venta, onCerrar, onGuardado }: Props) {
  const [filas, setFilas] = useState<ItemVenta[]>([]);
  const [editando, setEditando] = useState(false);
  const [modoCodigo, setModoCodigo] = useState<"crear" | "verificar" | null>(null);
  const [accionPendiente, setAccionPendiente] = useState<AccionProtegida | null>(null);
  const [codigo, setCodigo] = useState("");
  const [metodo, setMetodo] = useState(venta.metodo_pago);
  const [descuento, setDescuento] = useState(venta.descuento);
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);

  useEffect(() => {
    void ventasApi.listarItems(venta.id).then(setFilas);
    setMetodo(venta.metodo_pago);
    setDescuento(venta.descuento);
    setEditando(false);
    setModoCodigo(null);
    setAccionPendiente(null);
  }, [venta]);

  const subtotal = filas.reduce((acc, x) => acc + x.cantidad * x.precio_unitario, 0);
  const totalNeto = Math.max(0, subtotal - descuento);

  const solicitarCodigo = async (accion: AccionProtegida) => {
    if (accion === "eliminar" && !window.confirm(`¿Eliminar la venta #${venta.id}? Se restaurará el inventario de sus artículos.`)) return;
    setError(null);
    setAccionPendiente(accion);
    const existe = await seguridad.existeCodigoAdmin();
    setModoCodigo(existe ? "verificar" : "crear");
  };

  const eliminarVenta = async () => {
    setGuardando(true);
    setError(null);
    try {
      await ventasApi.eliminar(venta.id);
      await onGuardado();
      onCerrar();
    } catch (e) {
      setError(String(e));
    } finally {
      setGuardando(false);
    }
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
      const accion = accionPendiente;
      setModoCodigo(null);
      setCodigo("");
      setAccionPendiente(null);
      if (accion === "eliminar") {
        await eliminarVenta();
      } else {
        setEditando(true);
      }
    } catch (e) {
      setError(String(e));
    }
  };

  const guardarCambios = async () => {
    if (!filas.length) return setError("Elimina la venta completa si ya no tiene artículos.");
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

  const imprimir = () => {
    const filasHtml = filas.map((fila) => `
      <tr><td>${escapar(fila.producto_nombre)}</td><td class="right">${fila.cantidad}</td>
      <td class="right">${dinero.format(fila.precio_unitario)}</td><td class="right">${dinero.format(fila.cantidad * fila.precio_unitario)}</td></tr>`).join("");
    imprimirDocumento(`Comprobante venta #${venta.id}`, `
      <p><strong>Komtap</strong><br>Comprobante de venta #${venta.id}<br>${escapar(venta.fecha)}</p>
      <table><thead><tr><th>Artículo</th><th class="right">Cant.</th><th class="right">Precio</th><th class="right">Subtotal</th></tr></thead><tbody>${filasHtml}</tbody></table>
      <div class="summary"><div><span>Subtotal</span><strong>${dinero.format(subtotal)}</strong></div><div><span>Descuento</span><strong>${dinero.format(descuento)}</strong></div><div><span>Total</span><strong>${dinero.format(editando ? totalNeto : venta.total_neto)}</strong></div></div>
      <p>Método de pago: ${escapar(editando ? metodo : venta.metodo_pago)} · Estado: ${escapar(venta.estado_pago)}</p>`);
  };

  return (
    <Modal titulo="Comprobante de venta" onCerrar={onCerrar}>
      <section className="sale-receipt">
        <header className="sale-receipt__header">
          <div>
            <p className="eyebrow">KOMTAP · PUNTO DE VENTA</p>
            <h2>Comprobante de venta</h2>
            <p>{venta.fecha}</p>
          </div>
          <div className="sale-receipt__number">
            <span>Venta</span>
            <strong>#{venta.id}</strong>
            <em>{venta.estado_pago}</em>
          </div>
        </header>

        {venta.referencia_cliente && <p className="sale-receipt__customer">Cliente: <strong>{venta.referencia_cliente}</strong></p>}

        <div className="tabla-card sale-receipt__table">
          <table className="tabla-base">
            <thead><tr><th>Artículo</th><th>Cantidad</th><th>Precio unitario</th><th>Subtotal</th>{editando && <th />}</tr></thead>
            <tbody>
              {filas.map((f, i) => (
                <tr key={f.id}>
                  <td><strong>{f.producto_nombre}</strong></td>
                  <td>{editando ? <input aria-label={`Cantidad de ${f.producto_nombre}`} type="number" min="1" value={f.cantidad} onChange={(e) => setFilas(filas.map((x, n) => n === i ? { ...x, cantidad: Math.max(1, Number(e.target.value)) } : x))} /> : f.cantidad}</td>
                  <td>{editando ? <input aria-label={`Precio de ${f.producto_nombre}`} type="number" min="0" value={f.precio_unitario} onChange={(e) => setFilas(filas.map((x, n) => n === i ? { ...x, precio_unitario: Math.max(0, Number(e.target.value)) } : x))} /> : dinero.format(f.precio_unitario)}</td>
                  <td className="amount"><strong>{dinero.format(f.cantidad * f.precio_unitario)}</strong></td>
                  {editando && <td className="table-actions"><button className="danger-link" type="button" onClick={() => setFilas(filas.filter((_, n) => n !== i))}>Eliminar</button></td>}
                </tr>
              ))}
              {!filas.length && <tr><td colSpan={editando ? 5 : 4} className="empty-state">No quedan artículos. Puedes eliminar la venta completa.</td></tr>}
            </tbody>
          </table>
        </div>

        {editando && (
          <div className="form-grid__cols-2 sale-receipt__edit-fields">
            <div><label>Método de pago</label><select value={metodo} onChange={(e) => setMetodo(e.target.value)}>{METODOS_PAGO.map((m) => <option key={m}>{m}</option>)}</select></div>
            <div><label>Descuento</label><input type="number" min="0" value={descuento} onChange={(e) => setDescuento(Math.max(0, Number(e.target.value)))} /></div>
          </div>
        )}

        <div className="sale-receipt__totals">
          <div><span>Subtotal</span><strong>{dinero.format(subtotal)}</strong></div>
          <div><span>Descuento</span><strong>{dinero.format(descuento)}</strong></div>
          <div className="sale-receipt__grand-total"><span>Total</span><strong>{dinero.format(editando ? totalNeto : venta.total_neto)}</strong></div>
        </div>
      </section>

      {error && <p className="form-error">{error}</p>}

      {modoCodigo && (
        <div className="security-code-panel">
          <strong>{accionPendiente === "eliminar" ? "Confirma la eliminación" : "Autoriza la edición"}</strong>
          <label>{modoCodigo === "crear" ? "Crea el código de seguridad" : "Código de seguridad"}</label>
          <input type="password" inputMode="numeric" autoFocus value={codigo} onChange={(e) => setCodigo(e.target.value)} />
          <div className="modal__acciones"><button type="button" className="btn btn--ghost" onClick={() => { setModoCodigo(null); setAccionPendiente(null); }}>Cancelar</button><button type="button" className="btn btn--primary" onClick={() => void confirmarCodigo()}>Confirmar</button></div>
        </div>
      )}

      <div className="modal__acciones sale-receipt__actions">
        <button type="button" className="btn btn--ghost" onClick={onCerrar}>Cerrar</button>
        {!editando && !modoCodigo && <button type="button" className="btn btn--ghost" onClick={imprimir}>Imprimir / Generar PDF</button>}
        {!editando && !modoCodigo && <button type="button" className="btn btn--ghost" onClick={() => void solicitarCodigo("editar")}>Editar venta</button>}
        {!modoCodigo && <button type="button" className="btn btn--danger" disabled={guardando} onClick={() => void solicitarCodigo("eliminar")}>Eliminar venta</button>}
        {editando && <button type="button" className="btn btn--primary" disabled={guardando || !filas.length} onClick={() => void guardarCambios()}>{guardando ? "Guardando…" : "Guardar cambios"}</button>}
      </div>
    </Modal>
  );
}
