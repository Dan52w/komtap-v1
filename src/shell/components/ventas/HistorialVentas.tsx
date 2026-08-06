import type { Venta } from "../../types/ventas";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });

export function HistorialVentas({ ventas }: { ventas: Venta[] }) {
  return (
    <section className="history-card">
      <div className="panel-title">
        <h3>Últimas ventas</h3>
        <p>{ventas.length} movimientos registrados</p>
      </div>
      {ventas.map((v) => (
        <article className="history-item" key={v.id}>
          <div>
            <strong>Venta #{v.id}</strong>
            <span>{v.fecha.slice(0, 16)}</span>
            <small>
              {v.metodo_pago} · {v.estado_pago}
            </small>
          </div>
          <strong>{dinero.format(v.total_neto)}</strong>
        </article>
      ))}
    </section>
  );
}