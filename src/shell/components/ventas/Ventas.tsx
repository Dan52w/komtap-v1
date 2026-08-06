import { useEffect, useState } from "react";
import { productos as productosApi, ventas as ventasApi } from "../../lib/api";
import type { Producto } from "../../types/productos";
import type { Venta } from "../../types/ventas";
import { useCarritoVenta } from "../../../hooks/useCarritoVenta";
import { BuscadorArticulo } from "./BuscadorArticulo";
import { CarritoItems } from "./CarritoItems";
import { DatosVenta } from "./DatosVenta";
import { HistorialVentas } from "./HistorialVentas";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });

export function Ventas() {
  const [productos, setProductos] = useState<Producto[]>([]);
  const [historial, setHistorial] = useState<Venta[]>([]);
  const [metodo, setMetodo] = useState("Efectivo");
  const [descuento, setDescuento] = useState(0);
  const [cliente, setCliente] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);
  const carrito = useCarritoVenta();

  const cargar = async () => {
    try {
      const [p, v] = await Promise.all([productosApi.listar(), ventasApi.listar()]);
      setProductos(p.filter((x) => x.estado === "activo" && x.stock_actual > 0));
      setHistorial(v);
    } catch (e) {
      setError(String(e));
    }
  };

  useEffect(() => {
    void cargar();
  }, []);

  const agregarProducto = (p: Producto) => setError(carrito.agregar(p));

  const confirmar = async () => {
    if (!carrito.items.length) {
      setError("Agrega al menos un artículo.");
      return;
    }
    setGuardando(true);
    setError(null);
    try {
      await ventasApi.crear({
        items: carrito.items.map((x) => ({
          producto_id: x.producto.id!,
          cantidad: x.cantidad,
          precio_unitario: x.precio,
        })),
        descuento,
        metodo_pago: metodo,
        referencia_cliente: cliente || null,
      });
      carrito.limpiar();
      setDescuento(0);
      setCliente("");
      await cargar();
    } catch (e) {
      setError(String(e));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <div className="module-page">
      <div className="module-header">
        <div>
          <p className="eyebrow">Punto de venta</p>
          <h1>Ventas</h1>
          <p>Busca por nombre o código de barras y registra el pago.</p>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}

      <div className="sales-layout">
        <section className="sale-card">
          <h3>Nueva venta</h3>
          <BuscadorArticulo productos={productos} onSeleccionar={agregarProducto} />
          <CarritoItems items={carrito.items} onActualizar={carrito.actualizar} onQuitar={carrito.quitar} />
          <DatosVenta
            descuento={descuento}
            metodo={metodo}
            cliente={cliente}
            onCambiarDescuento={setDescuento}
            onCambiarMetodo={setMetodo}
            onCambiarCliente={setCliente}
          />
          <div className="sale-total">
            <span>Total</span>
            <strong>{dinero.format(carrito.total(descuento))}</strong>
          </div>
          <button className="btn btn--primary sale-submit" disabled={guardando} onClick={() => void confirmar()}>
            {guardando ? "Registrando…" : "Confirmar venta"}
          </button>
        </section>

        <HistorialVentas ventas={historial} />
      </div>
    </div>
  );
}