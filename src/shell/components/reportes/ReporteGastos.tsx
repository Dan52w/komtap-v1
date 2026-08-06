import type { Gasto } from "../../types/gastos";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });

export function ReporteGastos({ gastos }: { gastos: Gasto[] }) {
  return (
    <section className="report-card">
      <h3>Gastos del mes</h3>
      {gastos.length === 0 && <p className="empty-state">Sin gastos este mes.</p>}
      {gastos.map((x) => (
        <div className="report-row" key={x.id}>
          <span>{x.descripcion}<small>{x.categoria}</small></span>
          <b>{dinero.format(x.monto)}</b>
        </div>
      ))}
    </section>
  );
}