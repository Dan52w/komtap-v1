import { useState } from "react";
import { KpiCard } from "./KpiCard";
import { StockCriticoList } from "./StockCriticoList";
import type { Periodo } from "../../types/reportes";
import { useDashboard } from "../../../hooks/useDashboard";

function mesActualISO(): string {
  const hoy = new Date();
  return `${hoy.getFullYear()}-${String(hoy.getMonth() + 1).padStart(2, "0")}`;
}

export function Dashboard() {
  const [periodo, setPeriodo] = useState<Periodo>("dia");
  const mesActual = mesActualISO();
  const { financiero, ventasPorMetodo, stockCritico, deudaProveedores, cargando, error, recargar } =
    useDashboard(periodo, mesActual);

  return (
    <div className="dashboard">
      <div className="dashboard__header">
        <h1>Dashboard</h1>
        <div className="periodo-toggle">
          {(["dia", "mes", "anio"] as Periodo[]).map((p) => (
            <button
              key={p}
              className={p === periodo ? "activo" : ""}
              onClick={() => setPeriodo(p)}
            >
              {p === "dia" ? "Hoy" : p === "mes" ? "Este mes" : "Este año"}
            </button>
          ))}
          <button onClick={recargar} title="Actualizar">↻</button>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}

      {cargando && !financiero ? (
        <p>Cargando indicadores…</p>
      ) : financiero ? (
        <>
          <section className="kpi-grid">
            <KpiCard titulo="Dinero ingresado" valor={financiero.dinero_ingresado} tono="positivo" />
            <KpiCard titulo="Dinero gastado" valor={financiero.dinero_gastado} tono="negativo" />
            <KpiCard titulo="Ganancia bruta" valor={financiero.ganancia_bruta} />
            <KpiCard titulo="Ganancia neta real" valor={financiero.ganancia_neta_real} tono="positivo" />
            <KpiCard titulo="Valor del inventario" valor={financiero.valor_inventario} />
            <KpiCard titulo="Deuda con proveedores" valor={deudaProveedores} tono="negativo" />
          </section>

          <section className="dashboard__row">
            <div className="dashboard__panel">
              <h3>Ventas por método de pago (este mes)</h3>
              {ventasPorMetodo.length === 0 ? (
                <p className="empty-state">Sin ventas registradas este mes.</p>
              ) : (
                <ul className="metodo-pago-list">
                  {ventasPorMetodo.map((v) => (
                    <li key={v.metodo_pago}>
                      <span>{v.metodo_pago}</span>
                      <strong>
                        {new Intl.NumberFormat("es-CO", {
                          style: "currency",
                          currency: "COP",
                          maximumFractionDigits: 0,
                        }).format(v.total)}
                      </strong>
                    </li>
                  ))}
                </ul>
              )}
            </div>

            <div className="dashboard__panel">
              <h3>Alertas de stock</h3>
              <StockCriticoList productos={stockCritico} />
            </div>
          </section>
        </>
      ) : null}
    </div>
  );
}