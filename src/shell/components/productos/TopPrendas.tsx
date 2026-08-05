import { useState } from "react";
import { useTopPrendas } from "../../../hooks/useTopPrendas";

type RangoTop = "hoy" | "mes" | "historico";

// "Mes anterior" no es viable hoy: el backend solo filtra por
// "últimos N días desde ahora", no por rango de fechas fijo.
function diasParaRango(rango: RangoTop): number | null {
  if (rango === "hoy") return 1;
  if (rango === "mes") return new Date().getDate();
  return null;
}

const formatoMoneda = new Intl.NumberFormat("es-CO", {
  style: "currency",
  currency: "COP",
  maximumFractionDigits: 0,
});

export function TopPrendas() {
  const [rango, setRango] = useState<RangoTop>("mes");
  const { lista, cargando, error } = useTopPrendas(diasParaRango(rango));

  return (
    <div>
      <div className="periodo-toggle" style={{ marginBottom: "1rem" }}>
        <button className={rango === "hoy" ? "activo" : ""} onClick={() => setRango("hoy")}>
          Hoy
        </button>
        <button className={rango === "mes" ? "activo" : ""} onClick={() => setRango("mes")}>
          Este mes
        </button>
        <button className={rango === "historico" ? "activo" : ""} onClick={() => setRango("historico")}>
          Histórico
        </button>
      </div>

      {error && <p className="form-error">{error}</p>}

      {cargando ? (
        <p>Calculando ranking…</p>
      ) : lista.length === 0 ? (
        <p className="empty-state">Aún no hay ventas registradas en este período.</p>
      ) : (
        <table className="tabla-base">
          <thead>
            <tr>
              <th>#</th>
              <th>Prenda</th>
              <th>Unidades vendidas</th>
              <th>Ingreso total</th>
              <th>Margen estimado</th>
            </tr>
          </thead>
          <tbody>
            {lista.map((p, i) => (
              <tr key={p.producto_id}>
                <td>{i + 1}</td>
                <td>{p.nombre}</td>
                <td>{p.unidades_vendidas}</td>
                <td>{formatoMoneda.format(p.ingreso_total)}</td>
                <td>{formatoMoneda.format(p.margen_estimado)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}