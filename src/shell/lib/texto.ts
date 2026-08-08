export function normalizarTexto(valor: string): string {
  const limpio = valor.replace(/\s+/g, " ").trim();
  if (!limpio) return "";
  return limpio.charAt(0).toUpperCase() + limpio.slice(1).toLowerCase();
}