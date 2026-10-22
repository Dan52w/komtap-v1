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
  const hayFiltros = Boolean(
    filtro.sku || filtro.nombre || filtro.categoria || filtro.talla || filtro.estado_stock,
  );

  return (
    <div className="filtros-panel">
      <div className="filtros-panel__campo filtros-panel__campo--scan">
        <label htmlFor="filtro-sku">Código de barras</label>
        <div className="filtros-panel__scan-box">
          <span className="filtros-panel__scan-icon" aria-hidden="true">▤</span>
          <input
            id="filtro-sku"
            placeholder="Escanea o escribe el código…"
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
      </div>

      <div className="filtros-panel__campo filtros-panel__campo--nombre">
        <label htmlFor="filtro-nombre">Nombre</label>
        <input
          id="filtro-nombre"
          placeholder="Buscar por nombre…"
          value={filtro.nombre ?? ""}
          onChange={(e) => onCambiar({ ...filtro, nombre: e.target.value || null })}
        />
      </div>

      <div className="filtros-panel__campo filtros-panel__campo--categoria">
        <label htmlFor="filtro-categoria">Categoría</label>
        <select
          id="filtro-categoria"
          value={filtro.categoria ?? ""}
          onChange={(e) => onCambiar({ ...filtro, categoria: e.target.value || null })}
        >
          <option value="">Todas las categorías</option>
          {CATEGORIAS_PRODUCTO.map((categoria) => (
            <option key={categoria} value={categoria}>{categoria}</option>
          ))}
        </select>
      </div>

      <div className="filtros-panel__campo filtros-panel__campo--talla">
        <label htmlFor="filtro-talla">Talla</label>
        <input
          id="filtro-talla"
          placeholder="Ej. M, 32…"
          value={filtro.talla ?? ""}
          onChange={(e) => onCambiar({ ...filtro, talla: e.target.value || null })}
        />
      </div>

      <div className="filtros-panel__campo filtros-panel__campo--estado">
        <label htmlFor="filtro-estado">Estado</label>
        <select
          id="filtro-estado"
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

      <div className="filtros-panel__campo filtros-panel__campo--accion">
        {hayFiltros && (
          <button type="button" className="btn btn--ghost" onClick={() => onCambiar({})}>
            Limpiar filtros
          </button>
        )}
      </div>
    </div>
  );
}
