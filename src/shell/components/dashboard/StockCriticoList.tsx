import type { ProductoCritico } from "../../types/reportes";

export function StockCriticoList({ productos }: { productos: ProductoCritico[] }) {
  if (productos.length === 0) {
    return <p className="empty-state">No hay productos con stock crítico. 🎉</p>;
  }

  return (
    <table className="stock-critico-table">
      <thead>
        <tr>
          <th>Prenda</th>
          <th>Talla</th>
          <th>Stock</th>
          <th>Mínimo</th>
          <th>Estado</th>
        </tr>
      </thead>
      <tbody>
        {productos.map((p) => (
          <tr key={p.id}>
            <td>{p.nombre}</td>
            <td>{p.talla}</td>
            <td>{p.stock_actual}</td>
            <td>{p.stock_minimo}</td>
            <td>
              <span className={`badge badge--${p.estado_stock === "Agotado" ? "rojo" : "amarillo"}`}>
                {p.estado_stock}
              </span>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}