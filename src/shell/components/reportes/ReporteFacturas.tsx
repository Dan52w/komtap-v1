import type { FacturaConProveedor } from "../../../hooks/useReportes";
import { dinero } from "../../lib/format";

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
              Total {dinero(x.monto_total)} · Abonado {dinero(x.monto_total - x.saldo_pendiente)} · {x.fecha_emision} ({x.proveedor})
            </small>
          </span>
          <b>{dinero(x.saldo_pendiente)}</b>
        </div>
      ))}
    </section>
  );
}