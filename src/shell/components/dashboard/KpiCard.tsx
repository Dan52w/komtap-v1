type Props = {
  titulo: string;
  valor: number;
  tono?: "neutral" | "positivo" | "negativo";
};

const formatoMoneda = new Intl.NumberFormat("es-CO", {
  style: "currency",
  currency: "COP",
  maximumFractionDigits: 0,
});

export function KpiCard({ titulo, valor, tono = "neutral" }: Props) {
  return (
    <div className={`kpi-card kpi-card--${tono}`}>
      <span className="kpi-card__titulo">{titulo}</span>
      <span className="kpi-card__valor">{formatoMoneda.format(valor)}</span>
    </div>
  );
}