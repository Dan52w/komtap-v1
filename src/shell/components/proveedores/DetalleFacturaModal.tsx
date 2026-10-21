import { useEffect, useState } from "react";
import { proveedores as api } from "../../lib/api";
import { escapar, imprimirDocumento } from "../../lib/imprimir";
import { Modal } from "../common/Modal";
import { AbonoForm } from "./AbonoForm";
import type { AbonoProveedor, DetalleFactura, FacturaProveedor } from "../../types/proveedores";
import { dinero } from "../../lib/format";

const hoy = new Date().toISOString().slice(0, 10);

type Props = { factura: FacturaProveedor; onCerrar: () => void; onCambio: () => Promise<void> };

export function DetalleFacturaModal({ factura, onCerrar, onCambio }: Props) {
  const [items, setItems] = useState<DetalleFactura[]>([]);
  const [abonos, setAbonos] = useState<AbonoProveedor[]>([]);
  const [abonar, setAbonar] = useState(false);

  const cargar = async () => {
    const [i, a] = await Promise.all([api.listarItemsFactura(factura.id), api.listarAbonos(factura.id)]);
    setItems(i);
    setAbonos(a);
  };

  useEffect(() => {
    void cargar();
  }, []);

  const abonado = factura.monto_total - factura.saldo_pendiente;
  const vencida = !!factura.fecha_limite_pago && factura.saldo_pendiente > 0 && factura.fecha_limite_pago < hoy;

  const imprimir = () =>
    imprimirDocumento(
      "Comprobante de egreso",
      `<div class="summary">
        <div><span>Total de la compra</span><strong>${dinero(factura.monto_total)}</strong></div>
        <div><span>Total abonado</span><strong>${dinero(abonado)}</strong></div>
        <div><span>Saldo por deber</span><strong>${dinero(factura.saldo_pendiente)}</strong></div>
      </div>
      <p><strong>Factura:</strong> ${escapar(factura.numero_factura || `#${factura.id}`)} &nbsp;·&nbsp;
         <strong>Estado:</strong> ${escapar(factura.estado_pago)} &nbsp;·&nbsp;
         <strong>Emitida:</strong> ${escapar(factura.fecha_emision)}
         ${factura.fecha_limite_pago ? ` &nbsp;·&nbsp; <strong>Vence:</strong> ${escapar(factura.fecha_limite_pago)}` : ""}</p>
      <h2>Detalles de compra</h2>
      <table>
        <tr><th>Artículo</th><th>Cantidad</th><th>Costo unit.</th><th class="right">Subtotal</th></tr>
        ${items.map((x) => `<tr><td>${escapar(x.producto_nombre)}</td><td>${x.cantidad}</td><td>${dinero(x.costo_unitario)}</td><td class="right">${dinero(x.subtotal)}</td></tr>`).join("")}
      </table>
      <h2>Abonos</h2>
      <table>
        <tr><th>Fecha</th><th>Método</th><th>Referencia</th><th>Nota</th><th class="right">Monto</th></tr>
        ${abonos.map((x) => `<tr><td>${escapar(x.fecha)}</td><td>${escapar(x.metodo_pago)}${x.banco ? " · " + escapar(x.banco) : ""}</td><td>${escapar(x.numero_referencia || "-")}</td><td>${escapar(x.nota || "-")}</td><td class="right">${dinero(x.monto)}</td></tr>`).join("")}
      </table>`
    );

  return (
    <Modal titulo={factura.numero_factura ? `Factura ${factura.numero_factura}` : `Factura #${factura.id}`} onCerrar={onCerrar}>
      <div className="detalle-factura">
        <div className="detalle-factura__meta">
          <span className={`status status--${factura.estado_pago.toLowerCase()}`}>{factura.estado_pago}</span>
          <span>Emitida: {factura.fecha_emision}</span>
          {factura.fecha_limite_pago && (
            <span className={vencida ? "detalle-factura__vencida" : ""}>
              Vence: {factura.fecha_limite_pago}{vencida ? " (vencida)" : ""}
            </span>
          )}
        </div>

        <div className="detail-summary">
          <span>Total<strong>{dinero(factura.monto_total)}</strong></span>
          <span>Abonado<strong>{dinero(abonado)}</strong></span>
          <span>Saldo<strong>{dinero(factura.saldo_pendiente)}</strong></span>
        </div>

        <button className="btn btn--ghost" onClick={imprimir}>Imprimir / Generar PDF</button>

        <h4>Artículos ({items.length})</h4>
        {items.length === 0 ? (
          <p className="empty-state">Sin artículos.</p>
        ) : (
          <div className="tabla-card">
            <table className="tabla-base">
              <thead>
                <tr><th>Artículo</th><th>Cantidad</th><th>Costo unit.</th><th>Precio sugerido</th><th className="amount">Subtotal</th></tr>
              </thead>
              <tbody>
                {items.map((x) => (
                  <tr key={x.id}>
                    <td>{x.producto_nombre}</td>
                    <td>{x.cantidad}</td>
                    <td>{dinero(x.costo_unitario)}</td>
                    <td>{x.precio_venta_sugerido != null ? dinero(x.precio_venta_sugerido) : "—"}</td>
                    <td className="amount">{dinero(x.subtotal)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        <div className="modal-section-head">
          <h4>Abonos ({abonos.length})</h4>
          {factura.saldo_pendiente > 0 && (
            <button className="btn btn--primary" onClick={() => setAbonar(true)}>Registrar abono</button>
          )}
        </div>
        {abonos.length === 0 ? (
          <p className="empty-state">Aún no se han registrado abonos.</p>
        ) : (
          <div className="tabla-card">
            <table className="tabla-base">
              <thead>
                <tr><th>Fecha</th><th>Método</th><th>Referencia</th><th>Nota</th><th className="amount">Monto</th></tr>
              </thead>
              <tbody>
                {abonos.map((x) => (
                  <tr key={x.id}>
                    <td>{x.fecha.slice(0, 16).replace("T", " ")}</td>
                    <td>{x.metodo_pago}{x.banco ? ` · ${x.banco}` : ""}</td>
                    <td>{x.numero_referencia || "—"}</td>
                    <td>{x.nota || "—"}</td>
                    <td className="amount">{dinero(x.monto)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {abonar && (
          <AbonoForm
            saldoMaximo={factura.saldo_pendiente}
            onGuardar={async (abono) => {
              await api.registrarAbono(factura.id, abono);
              setAbonar(false);
              await cargar();
              await onCambio();
              onCerrar();
            }}
            onCancelar={() => setAbonar(false)}
          />
        )}
      </div>
    </Modal>
  );
}