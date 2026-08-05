import type { EstadoStockFiltro, FiltroProductos } from "../../types/productos";

type Props = {
  filtro: FiltroProductos;
  onCambiar: (filtro: FiltroProductos) => void;
};

const OPCIONES_ESTADO: { valor: EstadoStockFiltro | ""; label: string }[] = [
  { valor: "", label: "Todos los estados" },
  { valor: "agotado", label: "Agotado" },
  { valor: "bajo", label: "Stock bajo" },
  { valor: "disponible", label: "Disponible" },
];

export function FiltrosProductos({ filtro, onCambiar }: Props) {
  return (
    <div className="filtros-bar">
      <input
        className="filtros-bar__input"
        placeholder="Buscar por nombre…"
        value={filtro.nombre ?? ""}
        onChange={(e) => onCambiar({ ...filtro, nombre: e.target.value || null })}
      />
      <input
        className="filtros-bar__input"
        placeholder="Categoría"
        value={filtro.categoria ?? ""}
        onChange={(e) => onCambiar({ ...filtro, categoria: e.target.value || null })}
      />
      <input
        className="filtros-bar__input"
        placeholder="Talla"
        value={filtro.talla ?? ""}
        onChange={(e) => onCambiar({ ...filtro, talla: e.target.value || null })}
      />
      <select
        className="filtros-bar__input"
        value={filtro.estado_stock ?? ""}
        onChange={(e) =>
          onCambiar({
            ...filtro,
            estado_stock: (e.target.value || null) as EstadoStockFiltro | null,
          })
        }
      >
        {OPCIONES_ESTADO.map((o) => (
          <option key={o.valor} value={o.valor}>
            {o.label}
          </option>
        ))}
      </select>
    </div>
  );
}