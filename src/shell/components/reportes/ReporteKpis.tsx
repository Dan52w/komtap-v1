import type { DashboardFinanciero } from "../../types/reportes";
import { dinero } from "../../lib/format";

type Props = { financiero: DashboardFinanciero | null; deudaProveedores: number };

export function ReporteKpis({ financiero, deudaProveedores }: Props) {
  if (!financiero) return null;

  return (
    <section className="report-kpis">
      <Kpi label="Dinero ingresado" value={dinero(financiero.dinero_ingresado)} tone="positive" />
      <Kpi
        label="Ganancia neta real"
        value={dinero(financiero.ganancia_neta_real)}
        tone={financiero.ganancia_neta_real >= 0 ? "positive" : "negative"}
      />
      <Kpi label="Dinero gastado" value={dinero(financiero.dinero_gastado)} tone="negative" />
      <Kpi label="Deuda con proveedores" value={dinero(deudaProveedores)} tone="warning" />
    </section>
  );
}

function Kpi({ label, value, tone }: { label: string; value: string; tone: string }) {
  return (
    <div className={`report-kpi report-kpi--${tone}`}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}