import { useMemo, useState } from "react";
import { productos as productosApi } from "../../lib/api";
import { useProductos } from "../../../hooks/useProductos";
import { FiltrosProductos } from "./FiltrosProductos";
import { TablaProductos } from "./TablaProductos";
import { ProductoFormModal } from "./ProductoFormModal";
import { AjusteStockModal } from "./AjusteStockModal";
import { TopPrendas } from "./TopPrendas";
import type { FiltroProductos, Producto } from "../../types/productos";

type Tab = "catalogo" | "top";

const formatoMoneda = new Intl.NumberFormat("es-CO", {
  style: "currency",
  currency: "COP",
  maximumFractionDigits: 0,
});

export function Inventario() {
  const [tab, setTab] = useState<Tab>("catalogo");
  const [filtro, setFiltro] = useState<FiltroProductos>({});
  const { lista, cargando, error, recargar } = useProductos(filtro);

  const [modalForm, setModalForm] = useState<{ abierto: boolean; producto: Producto | null }>({
    abierto: false,
    producto: null,
  });
  const [modalAjuste, setModalAjuste] = useState<Producto | null>(null);
  const [errorAccion, setErrorAccion] = useState<string | null>(null);

  const stats = useMemo(() => {
    const agotados = lista.filter((p) => p.stock_actual === 0).length;
    const bajo = lista.filter((p) => p.stock_actual > 0 && p.stock_actual <= p.stock_minimo).length;
    const valor = lista.reduce((acc, p) => acc + p.stock_actual * p.costo_unitario, 0);
    return { total: lista.length, agotados, bajo, valor };
  }, [lista]);

  const guardarProducto = async (producto: Producto) => {
    if (producto.id) {
      await productosApi.actualizar(producto);
    } else {
      await productosApi.crear(producto);
    }
    await recargar();
    setModalForm({ abierto: false, producto: null });
  };

  const ajustarStock = async (delta: number, motivo: string) => {
    if (!modalAjuste?.id) return;
    await productosApi.ajustarStock({ productoId: modalAjuste.id, delta, motivo });
    await recargar();
    setModalAjuste(null);
  };

  const eliminarProducto = async (producto: Producto) => {
    if (!producto.id) return;
    const confirmar = window.confirm(`¿Eliminar "${producto.nombre}"? Esta acción no se puede deshacer.`);
    if (!confirmar) return;
    setErrorAccion(null);
    try {
      await productosApi.eliminar(producto.id);
      await recargar();
    } catch (e) {
      setErrorAccion(String(e));
    }
  };

  return (
    <div className="inventario">
      <div className="inventario__header">
        <div>
          <h1>Inventario</h1>
          <p className="inventario__subtitulo">Catálogo, existencias y prendas más vendidas</p>
        </div>
        <div className="periodo-toggle">
          <button className={tab === "catalogo" ? "activo" : ""} onClick={() => setTab("catalogo")}>
            Catálogo
          </button>
          <button className={tab === "top" ? "activo" : ""} onClick={() => setTab("top")}>
            Más vendidas
          </button>
        </div>
      </div>

      {tab === "catalogo" ? (
        <>
          <section className="inventario__stats">
            <div className="mini-stat">
              <span className="mini-stat__valor">{stats.total}</span>
              <span className="mini-stat__label">Prendas en vista</span>
            </div>
            <div className="mini-stat mini-stat--negativo">
              <span className="mini-stat__valor">{stats.agotados}</span>
              <span className="mini-stat__label">Agotadas</span>
            </div>
            <div className="mini-stat mini-stat--warning">
              <span className="mini-stat__valor">{stats.bajo}</span>
              <span className="mini-stat__label">Stock bajo</span>
            </div>
            <div className="mini-stat mini-stat--positivo">
              <span className="mini-stat__valor">{formatoMoneda.format(stats.valor)}</span>
              <span className="mini-stat__label">Valor a costo</span>
            </div>
          </section>

          <div className="inventario__toolbar">
            <FiltrosProductos filtro={filtro} onCambiar={setFiltro} />
            <button className="btn btn--primary" onClick={() => setModalForm({ abierto: true, producto: null })}>
              + Nueva prenda
            </button>
          </div>

          {error && <p className="form-error">{error}</p>}
          {errorAccion && <p className="form-error">{errorAccion}</p>}

          {cargando ? (
            <p>Cargando inventario…</p>
          ) : (
            <TablaProductos
              productos={lista}
              onEditar={(p) => setModalForm({ abierto: true, producto: p })}
              onAjustarStock={setModalAjuste}
              onEliminar={eliminarProducto}
            />
          )}
        </>
      ) : (
        <TopPrendas />
      )}

      {modalForm.abierto && (
        <ProductoFormModal
          productoInicial={modalForm.producto}
          onGuardar={guardarProducto}
          onCerrar={() => setModalForm({ abierto: false, producto: null })}
        />
      )}

      {modalAjuste && (
        <AjusteStockModal producto={modalAjuste} onAjustar={ajustarStock} onCerrar={() => setModalAjuste(null)} />
      )}
    </div>
  );
}