import type { Venta } from "../../types/ventas";
import { dinero } from "../../lib/format";

type Props = {
  ventas: Venta[];
  hayMas?: boolean;
  cargando?: boolean;
  onCargarMas?: () => void;
  onVerDetalle?: (venta: Venta) => void;
};

export function HistorialVentas({ ventas, hayMas, cargando, onCargarMas, onVerDetalle }: Props) {
  return (
    <section className="history-card">
      <div className="panel-title">
        <h3>Últimas ventas</h3>
        <p>{ventas.length} movimientos cargados</p>
      </div>
      {ventas.map((v) => (
        <article
          className="history-item"
          key={v.id}
          onClick={() => onVerDetalle?.(v)}
          style={{ cursor: "pointer" }}
        >
          <div>
            <strong>Venta #{v.id}</strong>
            <span>{v.fecha.slice(0, 16)}</span>
            <small>
              {v.metodo_pago} · {v.estado_pago}
              {(v.cliente_nombre || v.referencia_cliente) && ` · ${v.cliente_nombre ?? v.referencia_cliente}`}
            </small>
          </div>
          <strong>{dinero(v.total_neto)}</strong>
        </article>
      ))}

      {hayMas && (
        <button
          className="btn btn--ghost"
          style={{ width: "100%", marginTop: "0.75rem" }}
          disabled={cargando}
          onClick={onCargarMas}
        >
          {cargando ? "Cargando…" : "Cargar más"}
        </button>
      )}
    </section>
  );
}