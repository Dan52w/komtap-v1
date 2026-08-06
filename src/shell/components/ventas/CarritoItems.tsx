import type { ItemCarrito } from "../../../hooks/useCarritoVenta";

type Props = {
  items: ItemCarrito[];
  onActualizar: (index: number, cambios: Partial<Pick<ItemCarrito, "cantidad" | "precio">>) => void;
  onQuitar: (index: number) => void;
};

export function CarritoItems({ items, onActualizar, onQuitar }: Props) {
  if (items.length === 0) {
    return <p className="empty-state">Busca un artículo para añadirlo a la venta.</p>;
  }

  return (
    <div className="sale-items">
      {items.map((item, i) => (
        <div className="sale-item" key={item.producto.id}>
          <div>
            <strong>{item.producto.nombre}</strong>
            <small>
              {item.producto.talla} · {item.producto.color || "Sin color"} · Stock: {item.producto.stock_actual}
            </small>
          </div>
          <input
            aria-label="Cantidad"
            type="number"
            min="1"
            max={item.producto.stock_actual}
            value={item.cantidad}
            onChange={(e) => onActualizar(i, { cantidad: Number(e.target.value) })}
          />
          <input
            aria-label="Precio"
            type="number"
            min="0"
            value={item.precio}
            onChange={(e) => onActualizar(i, { precio: Number(e.target.value) })}
          />
          <button className="icon-button" onClick={() => onQuitar(i)}>
            ×
          </button>
        </div>
      ))}
    </div>
  );
}