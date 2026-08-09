const escapar = (valor: string | number | null | undefined) =>
  String(valor ?? "").replace(
    /[&<>"']/g,
    (caracter) =>
      ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[caracter]!
  );

export function imprimirDocumento(titulo: string, contenido: string) {
  const html = `<!doctype html><html lang="es"><head><meta charset="utf-8"><title>${escapar(
    titulo
  )}</title><style>body{font-family:"Segoe UI",Aptos,Arial,sans-serif;color:#183f4d;margin:42px;font-variant-numeric:tabular-nums}h1{font-family:Georgia,serif;font-size:25px;margin:0 0 5px;color:#1c4e5c}h2{font-size:14px;margin-top:26px;text-transform:uppercase;letter-spacing:.06em;color:#245b69}p{color:#59676b}table{width:100%;border-collapse:collapse;margin:12px 0 24px}th{background:#e7efec;text-align:left;font-size:11px;text-transform:uppercase;padding:9px}td{padding:9px;border-bottom:1px solid #d8d7d1;font-size:13px}tfoot td{background:#f5f3ed}.right{text-align:right}.summary{display:grid;grid-template-columns:repeat(3,1fr);gap:10px;margin:22px 0}.summary div{background:#f1f0eb;padding:12px;border-radius:6px}.summary span{display:block;font-size:11px;color:#59676b}.summary strong{font-family:Georgia,serif;font-size:17px}@media print{body{margin:22px}}</style></head><body><h1>${escapar(
    titulo
  )}</h1><p>Generado el ${new Date().toLocaleString(
    "es-CO"
  )}</p>${contenido}</body></html>`;

  // Se usa un iframe oculto en vez de window.open: dentro del WebView de Tauri
  // las ventanas emergentes suelen quedar bloqueadas, un iframe no cuenta como pop-up.
  const iframe = document.createElement("iframe");
  iframe.style.position = "fixed";
  iframe.style.right = "0";
  iframe.style.bottom = "0";
  iframe.style.width = "0";
  iframe.style.height = "0";
  iframe.style.border = "0";
  document.body.appendChild(iframe);

  const doc = iframe.contentWindow?.document;
  if (!doc) {
    document.body.removeChild(iframe);
    window.alert("No se pudo preparar el documento para imprimir.");
    return;
  }

  doc.open();
  doc.write(html);
  doc.close();

  const limpiar = () => {
    document.body.removeChild(iframe);
  };

  iframe.onload = () => {
    try {
      iframe.contentWindow?.focus();
      iframe.contentWindow?.print();
    } finally {
      // Se espera un poco antes de quitar el iframe para dar tiempo al diálogo de impresión.
      setTimeout(limpiar, 1000);
    }
  };
}

export { escapar };
