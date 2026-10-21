import { useEffect, useState } from "react";
import { clientes as clientesApi, productos as productosApi, ventas as ventasApi } from "../../lib/api";
import { dinero } from "../../lib/format";
import type { Producto } from "../../types/productos";
import type { Venta } from "../../types/ventas";
import type { Cliente } from "../../types/clientes";
import { useCarritoVenta } from "../../../hooks/useCarritoVenta";
import { BuscadorArticulo } from "./BuscadorArticulo";
import { CarritoItems } from "./CarritoItems";
import { DatosVenta } from "./DatosVenta";
import { HistorialVentas } from "./HistorialVentas";
import { DetalleVentaModal } from "./DetallesVentaModal";

const PAGE_SIZE = 30;

export function Ventas() {
  const [productos, setProductos] = useState<Producto[]>([]);
  const [clientes, setClientes] = useState<Cliente[]>([]);
  const [historial, setHistorial] = useState<Venta[]>([]);
  const [offset, setOffset] = useState(0);
  const [hayMas, setHayMas] = useState(true);
  const [cargandoMas, setCargandoMas] = useState(false);
  const [metodo, setMetodo] = useState("Efectivo");
  const [descuento, setDescuento] = useState(0);
  const [cliente, setCliente] = useState("");
  const [clienteId, setClienteId] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);
  const [ventaSeleccionada, setVentaSeleccionada] = useState<Venta | null>(null);
  const carrito = useCarritoVenta();

  // Recarga completa: productos + primera página del historial (offset 0).
  const cargar = async () => {
    try {
      const [p, v, c] = await Promise.all([
        productosApi.listar(),
        ventasApi.listar({ limite: PAGE_SIZE, offset: 0 }),
        clientesApi.listar(),
      ]);
      setProductos(p.filter((x) => x.estado === "activo" && x.stock_actual > 0));
      setHistorial(v);
      setClientes(c);
      setOffset(v.length);
      setHayMas(v.length === PAGE_SIZE);
    } catch (e) {
      setError(String(e));
    }
  };

  const cargarMas = async () => {
    if (cargandoMas || !hayMas) return;
    setCargandoMas(true);
    try {
      const v = await ventasApi.listar({ limite: PAGE_SIZE, offset });
      setHistorial((prev) => [...prev, ...v]);
      setOffset((prev) => prev + v.length);
      setHayMas(v.length === PAGE_SIZE);
    } catch (e) {
      setError(String(e));
    } finally {
      setCargandoMas(false);
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
        cliente_id: clienteId,
        referencia_cliente: clienteId ? null : cliente || null,
      });
      carrito.limpiar();
      setDescuento(0);
      setCliente("");
      setClienteId(null);
      await cargar(); // vuelve a la primera página con la venta nueva arriba
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
            clienteId={clienteId}
            clientes={clientes}
            onCambiarDescuento={setDescuento}
            onCambiarMetodo={setMetodo}
            onCambiarCliente={setCliente}
            onCambiarClienteId={setClienteId}
          />
          <div className="sale-total">
            <span>Total</span>
            <strong>{dinero(carrito.total(descuento))}</strong>
          </div>
          <button className="btn btn--primary sale-submit" disabled={guardando} onClick={() => void confirmar()}>
            {guardando ? "Registrando…" : "Confirmar venta"}
          </button>
        </section>

        <HistorialVentas
          ventas={historial}
          hayMas={hayMas}
          cargando={cargandoMas}
          onCargarMas={cargarMas}
          onVerDetalle={setVentaSeleccionada}
        />
      </div>

      {ventaSeleccionada && (
        <DetalleVentaModal
          venta={ventaSeleccionada}
          onCerrar={() => setVentaSeleccionada(null)}
          onGuardado={cargar}
        />
      )}
    </div>
  );
}