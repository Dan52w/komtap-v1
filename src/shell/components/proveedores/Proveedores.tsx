// src/shell/components/proveedores/Proveedores.tsx (orquestador)
import { useEffect, useState } from "react";
import { proveedores as api } from "../../lib/api";
import type { FacturaProveedor, Proveedor, ResumenProveedor } from "../../types/proveedores";
import { FacturaModal } from "./FacturaModal";
import { DetalleFacturaModal } from "./DetalleFacturaModal";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });

export function Proveedores() {
  const [p, setP] = useState<Proveedor[]>([]);
  const [r, setR] = useState<ResumenProveedor[]>([]);
  const [sel, setSel] = useState<number | null>(null);
  const [f, setF] = useState<FacturaProveedor[]>([]);
  const [crear, setCrear] = useState(false);
  const [detalle, setDetalle] = useState<FacturaProveedor | null>(null);
  const [error, setError] = useState<string | null>(null);

  const cargar = async () => {
    const [a, b] = await Promise.all([api.listar(), api.resumen()]);
    setP(a);
    setR(b);
    setSel((x) => x ?? a[0]?.id ?? null);
  };

  useEffect(() => {
    void cargar().catch((e) => setError(String(e)));
  }, []);

  useEffect(() => {
    if (sel) api.listarFacturas(sel).then(setF).catch((e) => setError(String(e)));
  }, [sel]);

  const deuda = (id: number | null) => r.find((x) => x.proveedor_id === id)?.deuda_total ?? 0;
  const proveedor = p.find((x) => x.id === sel);

  return (
    <div className="module-page providers-page">
      <div className="module-header">
        <div>
          <p className="eyebrow">Compras & cuentas por pagar</p>
          <h1>Proveedores</h1>
          <p>Productos, facturas y pagos organizados por proveedor.</p>
        </div>
        <button className="btn btn--primary" disabled={!sel} onClick={() => setCrear(true)}>
          + Agregar factura
        </button>
      </div>

      {error && <p className="form-error">{error}</p>}

      <section className="provider-workspace">
        <aside className="provider-list">
          <div className="provider-list__title">
            <span>Proveedores</span>
            <strong>{p.length}</strong>
          </div>
          {p.map((x) => (
            <button className={`provider-item ${x.id === sel ? "active" : ""}`} key={x.id} onClick={() => setSel(x.id)}>
              <span>{x.nombre}</span>
              <small>{deuda(x.id) ? `Debe ${dinero.format(deuda(x.id))}` : "Sin saldo pendiente"}</small>
            </button>
          ))}
        </aside>

        <section className="invoices-pane">
          <div className="invoices-pane__header">
            <div>
              <p className="eyebrow">{proveedor?.nombre || "Selecciona un proveedor"}</p>
              <h2>Facturas</h2>
            </div>
            <strong className="provider-debt">
              {dinero.format(deuda(sel))}
              <small>pendiente total</small>
            </strong>
          </div>

          <div className="invoice-grid">
            {f.map((x) => (
              <article className="invoice-card" key={x.id}>
                <div>
                  <span className={`status status--${x.estado_pago.toLowerCase()}`}>{x.estado_pago}</span>
                  <strong>{x.numero_factura || `Factura #${x.id}`}</strong>
                  <small>Emitida: {x.fecha_emision}</small>
                </div>
                <div className="invoice-card__footer">
                  <span>Saldo <b>{dinero.format(x.saldo_pendiente)}</b></span>
                  <button className="btn btn--ghost" onClick={() => setDetalle(x)}>Detalles</button>
                </div>
              </article>
            ))}
          </div>

          {!f.length && <div className="empty-card">No hay facturas para este proveedor.</div>}
        </section>
      </section>

      {crear && sel && (
        <FacturaModal
          proveedorId={sel}
          onCerrar={() => setCrear(false)}
          onGuardar={async (x) => {
            await api.crearFactura(x);
            setCrear(false);
            await cargar();
            setF(await api.listarFacturas(sel));
          }}
        />
      )}

      {detalle && (
        <DetalleFacturaModal
          factura={detalle}
          onCerrar={() => setDetalle(null)}
          onCambio={async () => {
            await cargar();
            setF(await api.listarFacturas(detalle.proveedor_id));
          }}
        />
      )}
    </div>
  );
}