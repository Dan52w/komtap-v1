import { useMemo, useState } from "react";
import type { Producto } from "../../types/productos";

type Props = { productos: Producto[]; onSeleccionar: (producto: Producto) => void };

export function BuscadorArticulo({ productos, onSeleccionar }: Props) {
  const [busqueda, setBusqueda] = useState("");

  const resultados = useMemo(() => {
    const q = busqueda.trim().toLowerCase();
    if (!q) return [];
    return productos
      .filter((p) => p.nombre.toLowerCase().includes(q) || p.sku?.toLowerCase().includes(q))
      .slice(0, 8);
  }, [busqueda, productos]);

  const seleccionar = (p: Producto) => {
    onSeleccionar(p);
    setBusqueda("");
  };

  return (
    <div className="barcode-search">
      <label htmlFor="busqueda-articulo">Buscar artículo</label>
      <input
        id="busqueda-articulo"
        autoFocus
        value={busqueda}
        onChange={(e) => setBusqueda(e.target.value)}
        placeholder="Nombre o código de barras"
      />
      {resultados.length > 0 && (
        <div className="search-results">
          {resultados.map((p) => (
            <button type="button" key={p.id} onClick={() => seleccionar(p)}>
              <strong>{p.nombre}</strong>
              <small>
                Código: {p.sku || "Sin código"} · Talla: {p.talla} · Color: {p.color || "Sin color"} · Stock:{" "}
                {p.stock_actual}
              </small>
            </button>
          ))}
        </div>
      )}
      <small>Escribe o escanea el código; los resultados aparecen al instante.</small>
    </div>
  );
}