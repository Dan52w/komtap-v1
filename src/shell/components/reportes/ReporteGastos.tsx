import type { Gasto } from "../../types/gastos";
import { dinero } from "../../lib/format";

export function ReporteGastos({ gastos }: { gastos: Gasto[] }) {
  return (
    <section className="report-card">
      <h3>Gastos del mes</h3>
      {gastos.length === 0 && <p className="empty-state">Sin gastos este mes.</p>}
      {gastos.map((x) => (
        <div className="report-row" key={x.id}>
          <span>{x.descripcion}<small>{x.categoria}</small></span>
          <b>{dinero(x.monto)}</b>
        </div>
      ))}
    </section>
  );
}