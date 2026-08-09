import type { EstadoStockFiltro, FiltroProductos } from "../../types/productos";
import { CATEGORIAS_PRODUCTO } from "../../lib/categorias";

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
    <div className="filtros-panel">
      <div className="filtros-panel__scan">
        <span className="filtros-panel__scan-icon" aria-hidden="true">▤</span>
        <input
          className="filtros-panel__scan-input"
          placeholder="Escanea o escribe el código de barras…"
          value={filtro.sku ?? ""}
          onChange={(e) => onCambiar({ ...filtro, sku: e.target.value || null })}
          autoFocus
          autoComplete="off"
          spellCheck={false}
        />
        {filtro.sku && (
          <button
            type="button"
            className="filtros-panel__scan-clear"
            onClick={() => onCambiar({ ...filtro, sku: null })}
            aria-label="Limpiar código"
          >
            ×
          </button>
        )}
      </div>

      <div className="filtros-panel__row">
        <input
          className="filtros-panel__input"
          placeholder="Buscar por nombre…"
          value={filtro.nombre ?? ""}
          onChange={(e) => onCambiar({ ...filtro, nombre: e.target.value || null })}
        />
        <select
          className="filtros-panel__input"
          value={filtro.categoria ?? ""}
          onChange={(e) => onCambiar({ ...filtro, categoria: e.target.value || null })}
        >
          <option value="">Todas las categorías</option>
          {CATEGORIAS_PRODUCTO.map((categoria) => (
            <option key={categoria} value={categoria}>{categoria}</option>
          ))}
        </select>
        <input
          className="filtros-panel__input"
          placeholder="Talla"
          value={filtro.talla ?? ""}
          onChange={(e) => onCambiar({ ...filtro, talla: e.target.value || null })}
        />
        <select
          className="filtros-panel__input"
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
    </div>
  );
}
