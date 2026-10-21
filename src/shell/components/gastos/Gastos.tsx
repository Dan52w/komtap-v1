import { useState } from "react";
import { useGastos } from "../../../hooks/useGastos";
import { GastoFormModal } from "./GastoFormModal";
import { TablaGastos } from "./TablaGastos";
import { escapar, imprimirDocumento } from "../../lib/imprimir";
import { dinero as moneda, mesActualISO } from "../../lib/format";
import type { Gasto } from "../../types/gastos";

const mesActual = mesActualISO();

export function Gastos() {
  const [mes, setMes] = useState(mesActual);
  const [nuevo, setNuevo] = useState(false);
  const [editando, setEditando] = useState<Gasto | null>(null);
  const { lista, error, crear, actualizar, eliminar } = useGastos(mes);
  const total = lista.reduce((acc, g) => acc + g.monto, 0);

  const generarPdf = () => {
    imprimirDocumento(
      `Gastos — ${mes}`,
      `<div class="summary">
         <div><span>Total del mes</span><strong>${moneda(total)}</strong></div>
         <div><span>Movimientos</span><strong>${lista.length}</strong></div>
       </div>
       <table>
         <tr><th>Fecha</th><th>Descripción</th><th>Categoría</th><th>Método</th><th class="right">Monto</th></tr>
         ${lista.map((g) => `<tr><td>${g.fecha.slice(0,10)}</td><td>${escapar(g.descripcion)}</td><td>${escapar(g.categoria)}</td><td>${escapar(g.metodo_pago)}</td><td class="right">${moneda(g.monto)}</td></tr>`).join("")}
       </table>`
    );
  };

  return (
    <div className="module-page">
      <div className="module-header">
        <div>
          <p className="eyebrow">Control operativo</p>
          <h1>Gastos</h1>
          <p>Registra los egresos que no provienen de compras a proveedores.</p>
        </div>
        <div className="header-actions">
          <input aria-label="Mes" type="month" value={mes} onChange={(e) => setMes(e.target.value)} />
          <button className="btn btn--ghost" onClick={generarPdf}>
            Generar PDF
          </button>
          <button className="btn btn--primary" onClick={() => setNuevo(true)}>
            + Registrar gasto
          </button>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}

      <section className="summary-strip">
        <div>
          <span>Total del mes</span>
          <strong className="amount--negative">{moneda(total)}</strong>
        </div>
        <div>
          <span>Movimientos</span>
          <strong>{lista.length}</strong>
        </div>
      </section>

      <TablaGastos gastos={lista} onEditar={setEditando} onEliminar={eliminar} />

      {nuevo && (
        <GastoFormModal
          onCerrar={() => setNuevo(false)}
          onGuardar={async (g) => {
            await crear(g);
            setNuevo(false);
          }}
        />
      )}

      {editando && (
        <GastoFormModal
          inicial={editando}
          onCerrar={() => setEditando(null)}
          onGuardar={async (g) => {
            await actualizar(editando.id, {
              descripcion: g.descripcion,
              monto: g.monto,
              fecha: g.fecha ?? editando.fecha,
              categoria: g.categoria ?? editando.categoria,
              metodo_pago: g.metodo_pago ?? editando.metodo_pago,
            });
            setEditando(null);
          }}
        />
      )}
    </div>
  );
}