import { useEffect, useMemo, useState } from "react";
import { productos as productosApi } from "../../lib/api";
import { Modal } from "../common/Modal";
import { AbonoForm } from "./AbonoForm";
import type { AbonoInput, ItemFactura, NuevaFacturaInput } from "../../types/proveedores";
import type { Producto } from "../../types/productos";

const dinero = new Intl.NumberFormat("es-CO", { style: "currency", currency: "COP", maximumFractionDigits: 0 });
const hoy = new Date().toISOString().slice(0, 10);

type Linea = ItemFactura & { etiqueta: string };

const lineaNueva = (): Linea => ({
  producto_id: null,
  nombre_nuevo: "",
  sku_nuevo: "",
  categoria_nueva: "General",
  talla_nueva: "",
  color_nuevo: "",
  cantidad: 1,
  costo_unitario: 0,
  precio_venta_sugerido: 0,
  stock_minimo_nuevo: 3,
  etiqueta: "",
});

type Props = {
  proveedorId: number;
  onCerrar: () => void;
  onGuardar: (x: NuevaFacturaInput) => Promise<void>;
};

export function FacturaModal({ proveedorId, onCerrar, onGuardar }: Props) {
  const [productos, setProductos] = useState<Producto[]>([]);
  const [q, setQ] = useState("");
  const [actual, setActual] = useState<Linea>(lineaNueva());
  const [lineas, setLineas] = useState<Linea[]>([]);
  const [numero, setNumero] = useState("");
  const [fecha, setFecha] = useState(hoy);
  const [vence, setVence] = useState("");
  const [nuevo, setNuevo] = useState(false);
  const [conAbonoInicial, setConAbonoInicial] = useState(false);
  const [abonoInicial, setAbonoInicial] = useState<AbonoInput | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    productosApi.listar().then(setProductos);
  }, []);

  const coincidencias = useMemo(
    () => productos.filter((x) => `${x.nombre} ${x.sku ?? ""}`.toLowerCase().includes(q.toLowerCase())).slice(0, 8),
    [productos, q]
  );

  const totalFactura = useMemo(
    () => lineas.reduce((acc, l) => acc + l.cantidad * l.costo_unitario, 0),
    [lineas]
  );

  const agregar = () => {
    if (actual.cantidad < 1 || actual.costo_unitario < 0 || (!actual.producto_id && !actual.nombre_nuevo?.trim())) {
      return setError("Completa los datos del artículo.");
    }
    setLineas([
      ...lineas,
      {
        ...actual,
        etiqueta: actual.producto_id
          ? productos.find((x) => x.id === actual.producto_id)?.nombre ?? "Producto"
          : actual.nombre_nuevo!,
      },
    ]);
    setActual(lineaNueva());
    setQ("");
    setNuevo(false);
    setError(null);
  };

  return (
    <Modal titulo="Agregar factura de compra" onCerrar={onCerrar}>
      <form
        className="form-grid"
        onSubmit={async (e) => {
          e.preventDefault();
          if (!lineas.length) return setError("Agrega al menos un artículo.");
          if (conAbonoInicial && (!abonoInicial || abonoInicial.monto <= 0)) {
            return setError("Completa el abono inicial o desactívalo.");
          }
          if (abonoInicial && abonoInicial.monto > totalFactura) {
            return setError("El abono inicial no puede superar el total de la compra.");
          }
          try {
            await onGuardar({
              proveedor_id: proveedorId,
              numero_factura: numero || null,
              fecha_emision: fecha,
              fecha_limite_pago: vence || null,
              items: lineas,
              abono_inicial: conAbonoInicial ? abonoInicial : null,
            });
          } catch (x) {
            setError(String(x));
          }
        }}
      >
        <div className="form-grid__cols-2">
          <div>
            <label>Número</label>
            <input value={numero} onChange={(e) => setNumero(e.target.value)} />
          </div>
          <div>
            <label>Fecha</label>
            <input type="date" value={fecha} onChange={(e) => setFecha(e.target.value)} />
          </div>
        </div>

        <label>Vencimiento</label>
        <input type="date" value={vence} onChange={(e) => setVence(e.target.value)} />

        <div className="invoice-lines">
          {lineas.map((x, i) => (
            <div className="report-row" key={i}>
              <span>{x.etiqueta} · {x.cantidad} u.</span>
              <b>{dinero.format(x.cantidad * x.costo_unitario)}</b>
              <button type="button" className="danger-link" onClick={() => setLineas(lineas.filter((_, n) => n !== i))}>
                Quitar
              </button>
            </div>
          ))}
        </div>

        <label>Buscar producto por nombre o código de barras</label>
        <input
          value={q}
          onChange={(e) => {
            setQ(e.target.value);
            setNuevo(false);
          }}
          placeholder="Escribe nombre o código"
        />

        {q && (
          <div className="search-results">
            {coincidencias.map((x) => (
              <button
                type="button"
                key={x.id}
                onClick={() => {
                  setActual({
                    ...lineaNueva(),
                    producto_id: x.id,
                    etiqueta: x.nombre,
                    costo_unitario: x.costo_unitario,
                    precio_venta_sugerido: x.precio_venta,
                  });
                  setQ(x.nombre);
                }}
              >
                {x.nombre} <small>{x.sku || "Sin código"}</small>
              </button>
            ))}
            <button type="button" onClick={() => { setNuevo(true); setActual(lineaNueva()); }}>
              + Crear nuevo artículo
            </button>
          </div>
        )}

        {(actual.producto_id || nuevo) && (
          <>
            <p className="form-hint">
              {nuevo ? "Nuevo artículo" : "Producto seleccionado: " + actual.etiqueta}
            </p>
            {nuevo && (
              <>
                <div className="form-grid__cols-2">
                  <div>
                    <label>Nombre</label>
                    <input value={actual.nombre_nuevo ?? ""} onChange={(e) => setActual({ ...actual, nombre_nuevo: e.target.value })} />
                  </div>
                  <div>
                    <label>Código de barras / SKU</label>
                    <input value={actual.sku_nuevo ?? ""} onChange={(e) => setActual({ ...actual, sku_nuevo: e.target.value })} />
                  </div>
                </div>
                <div className="form-grid__cols-2">
                  <div>
                    <label>Categoría</label>
                    <input value={actual.categoria_nueva ?? ""} onChange={(e) => setActual({ ...actual, categoria_nueva: e.target.value })} />
                  </div>
                  <div>
                    <label>Talla</label>
                    <input value={actual.talla_nueva ?? ""} onChange={(e) => setActual({ ...actual, talla_nueva: e.target.value })} />
                  </div>
                </div>
                <label>Color</label>
                <input value={actual.color_nuevo ?? ""} onChange={(e) => setActual({ ...actual, color_nuevo: e.target.value })} />
              </>
            )}

            <div className="form-grid__cols-2">
              <div>
                <label>Cantidad</label>
                <input type="number" min="1" step="1" placeholder="Ej. 10" value={actual.cantidad} 
                  onChange={(e) => setActual({ ...actual, cantidad: Number(e.target.value) })}/>
              </div>
              <div>
                <label>Costo unitario</label>
                <input type="number" min="0" step="1" placeholder="Ej. 25000" value={actual.costo_unitario || ""}
                  onChange={(e) => setActual({...actual, costo_unitario: Number(e.target.value), })}/>
              </div>
            </div>

            <div className="form-grid__cols-2">
              <div>
                <label>Precio de venta</label>
                <input type="number" min="0" step="1" placeholder="Ej. 45000" value={actual.precio_venta_sugerido || ""}
                  onChange={(e) => setActual({...actual, precio_venta_sugerido: Number(e.target.value), })}/>
              </div>
              {nuevo && (
                <div>
                  <label>Stock mínimo</label>
                  <input type="number" min="0" value={actual.stock_minimo_nuevo ?? 3} onChange={(e) => setActual({ ...actual, stock_minimo_nuevo: Number(e.target.value) })} />
                </div>
              )}
            </div>

            <button type="button" className="btn btn--ghost" onClick={agregar}>
              + Añadir a la factura
            </button>
          </>
        )}

        {lineas.length > 0 && (
          <label style={{ display: "flex", alignItems: "center", gap: "0.5rem", marginTop: "0.75rem" }}>
            <input
              type="checkbox"
              checked={conAbonoInicial}
              onChange={(e) => {
                setConAbonoInicial(e.target.checked);
                if (!e.target.checked) setAbonoInicial(null);
              }}
            />
            Registrar un abono inicial junto con la factura
          </label>
        )}

        {conAbonoInicial && (
          <AbonoForm
            saldoMaximo={totalFactura}
            valorInicial={abonoInicial ?? undefined}
            textoBoton="Confirmar abono inicial"
            onGuardar={(abono) => setAbonoInicial(abono)}
          />
        )}
        {conAbonoInicial && abonoInicial && (
          <p className="form-hint">
            Abono inicial listo: {dinero.format(abonoInicial.monto)} ({abonoInicial.metodo_pago}). Se registrará al guardar la factura.
          </p>
        )}

        {error && <p className="form-error">{error}</p>}

        <div className="modal__acciones">
          <button type="button" className="btn btn--ghost" onClick={onCerrar}>
            Cancelar
          </button>
          <button className="btn btn--primary">Registrar factura ({lineas.length})</button>
        </div>
      </form>
    </Modal>
  );
}