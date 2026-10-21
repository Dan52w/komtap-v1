/** Formato de moneda y fechas centralizado (es-CO / COP). */

const formatoCOP = new Intl.NumberFormat("es-CO", {
  style: "currency",
  currency: "COP",
  maximumFractionDigits: 0,
});

export function dinero(valor: number | null | undefined): string {
  return formatoCOP.format(valor ?? 0);
}

/** "2026-10-21 14:30:00" → "21/10/2026 2:30 p. m." (corto y legible). */
export function fechaHora(valor: string | null | undefined): string {
  if (!valor) return "—";
  const d = new Date(valor.replace(" ", "T"));
  if (Number.isNaN(d.getTime())) return valor;
  return d.toLocaleString("es-CO", {
    day: "2-digit",
    month: "2-digit",
    year: "numeric",
    hour: "numeric",
    minute: "2-digit",
  });
}

/** "2026-10-21" → "21/10/2026". */
export function fechaCorta(valor: string | null | undefined): string {
  if (!valor) return "—";
  const d = new Date(`${valor}T00:00:00`);
  if (Number.isNaN(d.getTime())) return valor;
  return d.toLocaleDateString("es-CO", { day: "2-digit", month: "2-digit", year: "numeric" });
}

/** Mes actual en formato "YYYY-MM". */
export function mesActualISO(): string {
  const hoy = new Date();
  return `${hoy.getFullYear()}-${String(hoy.getMonth() + 1).padStart(2, "0")}`;
}
