import { useState } from "react";
import { useReportes } from "../../../hooks/useReportes";
import { escapar, imprimirDocumento } from "../../lib/imprimir";
import { ReporteKpis } from "./ReporteKpis";
import { ReporteVentas } from "./ReporteVentas";
import { ReporteGastos } from "./ReporteGastos";
import { ReporteFacturas } from "./ReporteFacturas";
import { dinero } from "../../lib/format";

const mesActual = new Date().toISOString().slice(0, 7);

export function Reportes() {
  const [mes, setMes] = useState(mesActual);
  const { financiero, ventas, gastos, facturasPendientes, deudaProveedores, cargando, error } = useReportes(mes);

  const imprimir = () => {
    if (!financiero) return;
    const totalGastos = gastos.reduce((total, gasto) => total + gasto.monto, 0);
    const filasGastos = gastos.map((gasto) => `
      <tr><td>${escapar(gasto.fecha)}</td><td>${escapar(gasto.categoria)}</td><td>${escapar(gasto.descripcion)}</td><td class="right">${dinero(gasto.monto)}</td></tr>`).join("");
    const filasFacturas = facturasPendientes.map((x) => `
      <tr><td>${escapar(x.numero_factura || x.id)}</td><td>${escapar(x.proveedor)}</td><td class="right">${dinero(x.monto_total)}</td><td class="right">${dinero(x.monto_total - x.saldo_pendiente)}</td><td>${escapar(x.fecha_emision)}</td></tr>`).join("");
    imprimirDocumento(
      `Reporte mensual · ${mes}`,
      `<div class="summary">
        <div><span>Dinero ingresado</span><strong>${dinero(financiero.dinero_ingresado)}</strong></div>
        <div><span>Ganancia neta real</span><strong>${dinero(financiero.ganancia_neta_real)}</strong></div>
        <div><span>Deuda con proveedores</span><strong>${dinero(deudaProveedores)}</strong></div>
      </div>
      <h2>Gastos del mes</h2>
      <table><thead><tr><th>Fecha</th><th>Categoría</th><th>Descripción</th><th class="right">Monto</th></tr></thead>
      <tbody>${filasGastos || '<tr><td colspan="4">Sin gastos registrados este mes.</td></tr>'}</tbody>
      <tfoot><tr><td colspan="3" class="right"><strong>Total gastos operativos</strong></td><td class="right"><strong>${dinero(totalGastos)}</strong></td></tr></tfoot></table>
      <h2>Facturas de proveedores</h2>
      <table><thead><tr><th>Factura</th><th>Proveedor</th><th class="right">Total</th><th class="right">Abonado</th><th>Emisión</th></tr></thead><tbody>${filasFacturas || '<tr><td colspan="5">Sin facturas pendientes.</td></tr>'}</tbody></table>`
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
