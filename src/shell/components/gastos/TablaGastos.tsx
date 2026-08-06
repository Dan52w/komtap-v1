import type { Gasto } from "../../types/gastos";

const moneda = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });

type Props = {
  gastos: Gasto[];
  onEliminar: (id: number) => void;
};

export function TablaGastos({ gastos, onEliminar }: Props) {
  return (
    <div className="tabla-card">
      <table className="tabla-base">
        <thead>
          <tr>
            <th>Fecha</th>
            <th>Descripción</th>
            <th>Categoría</th>
            <th>Pago</th>
            <th>Monto</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {gastos.map((g) => (
            <tr key={g.id}>
              <td>{g.fecha.slice(0, 10)}</td>
              <td>
                <strong>{g.descripcion}</strong>
              </td>
              <td>
                <span className="badge badge--amarillo">{g.categoria}</span>
              </td>
              <td>{g.metodo_pago}</td>
              <td className="amount amount--negative">{moneda.format(g.monto)}</td>
              <td>
                <button
                  className="danger-link"
                  onClick={() => {
                    if (window.confirm("¿Eliminar este gasto?")) onEliminar(g.id);
                  }}
                >
                  Eliminar
                </button>
              </td>
            </tr>
          ))}
          {gastos.length === 0 && (
            <tr>
              <td colSpan={6} className="empty-state">
                No hay gastos para este mes.
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}