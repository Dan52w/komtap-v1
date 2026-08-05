import type { Producto } from "../../types/productos";

type Props = {
  productos: Producto[];
  onEditar: (producto: Producto) => void;
  onAjustarStock: (producto: Producto) => void;
  onEliminar: (producto: Producto) => void;
};

const formatoMoneda = new Intl.NumberFormat("es-CO", {
  style: "currency",
  currency: "COP",
  maximumFractionDigits: 0,
});

function estadoStock(p: Producto): { texto: string; clase: string } {
  if (p.stock_actual === 0) return { texto: "Agotado", clase: "rojo" };
  if (p.stock_actual <= p.stock_minimo) return { texto: "Stock bajo", clase: "amarillo" };
  return { texto: "Disponible", clase: "verde" };
}

export function TablaProductos({ productos, onEditar, onAjustarStock, onEliminar }: Props) {
  if (productos.length === 0) {
    return <p className="empty-state">No hay prendas que coincidan con estos filtros.</p>;
  }

  return (
    <div className="tabla-card">
      <table className="tabla-base">
        <thead>
          <tr>
            <th>Prenda</th>
            <th>Categoría</th>
            <th>Talla</th>
            <th>Color</th>
            <th>Costo</th>
            <th>Precio</th>
            <th>Stock</th>
            <th>Estado</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {productos.map((p) => {
            const estado = estadoStock(p);
            return (
              <tr key={p.id}>
                <td>
                  <strong>{p.nombre}</strong>
                  {p.sku && <span className="tabla-base__sku">{p.sku}</span>}
                </td>
                <td>{p.categoria}</td>
                <td>{p.talla}</td>
                <td>{p.color ?? "—"}</td>
                <td>{formatoMoneda.format(p.costo_unitario)}</td>
                <td>{formatoMoneda.format(p.precio_venta)}</td>
                <td>{p.stock_actual}</td>
                <td>
                  <span className={`badge badge--${estado.clase}`}>{estado.texto}</span>
                </td>
                <td className="tabla-base__acciones">
                  <button className="btn-icono" onClick={() => onAjustarStock(p)} title="Ajustar stock">
                    ±
                  </button>
                  <button className="btn-icono" onClick={() => onEditar(p)} title="Editar">
                    ✎
                  </button>
                  <button
                    className="btn-icono btn-icono--peligro"
                    onClick={() => onEliminar(p)}
                    title="Eliminar"
                  >
                    ✕
                  </button>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}