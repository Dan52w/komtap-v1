import type { FacturaConProveedor } from "../../../hooks/useReportes";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });

export function ReporteFacturas({ facturas }: { facturas: FacturaConProveedor[] }) {
  return (
    <section className="report-card">
      <h3>Facturas de proveedores</h3>
      {facturas.length === 0 && <p className="empty-state">Sin saldos pendientes.</p>}
      {facturas.map((x) => (
        <div className="report-row" key={x.id}>
          <span>
            <strong>{x.numero_factura || x.id}</strong>
            <small>
              Total {dinero.format(x.monto_total)} · Abonado {dinero.format(x.monto_total - x.saldo_pendiente)} · {x.fecha_emision} ({x.proveedor})
            </small>
          </span>
          <b>{dinero.format(x.saldo_pendiente)}</b>
        </div>
      ))}
    </section>
  );
}