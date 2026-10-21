import { dinero } from "../../lib/format";
type Props = {
  titulo: string;
  valor: number;
  tono?: "neutral" | "positivo" | "negativo";
};

export function KpiCard({ titulo, valor, tono = "neutral" }: Props) {
  return (
    <div className={`kpi-card kpi-card--${tono}`}>
      <span className="kpi-card__titulo">{titulo}</span>
      <span className="kpi-card__valor">{dinero(valor)}</span>
    </div>
  );
}