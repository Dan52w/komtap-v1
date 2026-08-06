import { useState } from "react";
import { useReportes } from "../../../hooks/useReportes";
import { escapar, imprimirDocumento } from "../../lib/imprimir";
import { ReporteKpis } from "./ReporteKpis";
import { ReporteVentas } from "./ReporteVentas";
import { ReporteGastos } from "./ReporteGastos";
import { ReporteFacturas } from "./ReporteFacturas";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });
const mesActual = new Date().toISOString().slice(0, 7);

export function Reportes() {
  const [mes, setMes] = useState(mesActual);
  const { financiero, ventas, gastos, facturasPendientes, deudaProveedores, cargando, error } = useReportes(mes);

  const imprimir = () => {
    if (!financiero) return;
    imprimirDocumento(
      "Reporte mensual",
      `<div class="summary">
        <div><span>Dinero ingresado</span><strong>${dinero.format(financiero.dinero_ingresado)}</strong></div>
        <div><span>Ganancia neta real</span><strong>${dinero.format(financiero.ganancia_neta_real)}</strong></div>
        <div><span>Deuda con proveedores</span><strong>${dinero.format(deudaProveedores)}</strong></div>
      </div>
      <h2>Facturas de proveedores</h2>
      <table>${facturasPendientes
        .map((x) => `<tr><td>${escapar(x.numero_factura || x.id)}</td><td>${escapar(x.proveedor)}</td><td>${dinero.format(x.monto_total)}</td><td>${dinero.format(x.monto_total - x.saldo_pendiente)}</td><td>${escapar(x.fecha_emision)}</td></tr>`)
        .join("")}</table>`
    );
  };

  return (
    <div className="module-page">
      <div className="module-header">
        <div>
          <p className="eyebrow">Resumen financiero</p>
          <h1>Reportes</h1>
        </div>
        <div className="header-actions">
          <input type="month" value={mes} onChange={(e) => setMes(e.target.value)} />
          <button className="btn btn--ghost" onClick={imprimir} disabled={!financiero}>
            Imprimir / Generar PDF
          </button>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}
      {cargando && !financiero ? (
        <p>Cargando reporte…</p>
      ) : (
        <>
          <ReporteKpis financiero={financiero} deudaProveedores={deudaProveedores} />
          <div className="report-grid">
            <ReporteVentas ventas={ventas} />
            <ReporteGastos gastos={gastos} />
            <ReporteFacturas facturas={facturasPendientes} />
          </div>
        </>
      )}
    </div>
  );
}