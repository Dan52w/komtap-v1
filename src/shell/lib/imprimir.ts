const escapar = (valor: string | number | null | undefined) => String(valor ?? "").replace(/[&<>"']/g, (caracter) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[caracter]!);

export function imprimirDocumento(titulo: string, contenido: string) {
  // En WebView/Tauri, `noopener` hace que window.open devuelva null y rompe la impresión.
  const ventana = window.open("", "_blank", "width=900,height=700");
  if (!ventana) return window.alert("No se pudo abrir la vista de impresión. Permite ventanas emergentes e inténtalo de nuevo.");
  ventana.document.write(`<!doctype html><html lang="es"><head><meta charset="utf-8"><title>${escapar(titulo)}</title><style>body{font-family:Arial,sans-serif;color:#1e2e34;margin:42px}h1{font-size:23px;margin:0 0 5px}h2{font-size:14px;margin-top:26px;text-transform:uppercase;color:#245b69}p{color:#59676b}table{width:100%;border-collapse:collapse;margin:12px 0 24px}th{background:#e7efec;text-align:left;font-size:11px;text-transform:uppercase;padding:9px}td{padding:9px;border-bottom:1px solid #d8d7d1;font-size:13px}.right{text-align:right}.summary{display:grid;grid-template-columns:repeat(3,1fr);gap:10px;margin:22px 0}.summary div{background:#f1f0eb;padding:12px;border-radius:6px}.summary span{display:block;font-size:11px;color:#59676b}.summary strong{font-size:16px}@media print{body{margin:22px}}</style></head><body><h1>${escapar(titulo)}</h1><p>Generado el ${new Date().toLocaleString("es-CO")}</p>${contenido}</body></html>`);
  ventana.document.close(); ventana.focus(); setTimeout(() => ventana.print(), 250);
}
export { escapar };
