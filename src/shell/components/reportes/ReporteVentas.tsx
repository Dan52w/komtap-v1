import type { Venta } from "../../types/ventas";
import { dinero } from "../../lib/format";

export function ReporteVentas({ ventas }: { ventas: Venta[] }) {
  return (
    <section className="report-card">
      <h3>Ventas del mes</h3>
      {ventas.length === 0 && <p className="empty-state">Sin ventas este mes.</p>}
      {ventas.map((x) => (
        <div className="report-row" key={x.id}>
          <span>Venta #{x.id}<small>{x.fecha.slice(0, 10)}</small></span>
          <b>{dinero(x.total_neto)}</b>
        </div>
      ))}
    </section>
  );
}