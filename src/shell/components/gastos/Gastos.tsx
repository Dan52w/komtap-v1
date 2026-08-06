import { useState } from "react";
import { useGastos } from "../../../hooks/useGastos";
import { GastoFormModal } from "./GastoFormModal";
import { TablaGastos } from "./TablaGastos";

const moneda = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });
const mesActual = new Date().toISOString().slice(0, 7);

export function Gastos() {
  const [mes, setMes] = useState(mesActual);
  const [nuevo, setNuevo] = useState(false);
  const { lista, error, crear, eliminar } = useGastos(mes);
  const total = lista.reduce((acc, g) => acc + g.monto, 0);

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
          <button className="btn btn--primary" onClick={() => setNuevo(true)}>
            + Registrar gasto
          </button>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}

      <section className="summary-strip">
        <div>
          <span>Total del mes</span>
          <strong className="amount--negative">{moneda.format(total)}</strong>
        </div>
        <div>
          <span>Movimientos</span>
          <strong>{lista.length}</strong>
        </div>
      </section>

      <TablaGastos gastos={lista} onEliminar={eliminar} />

      {nuevo && (
        <GastoFormModal
          onCerrar={() => setNuevo(false)}
          onGuardar={async (g) => {
            await crear(g);
            setNuevo(false);
          }}
        />
      )}
    </div>
  );
}