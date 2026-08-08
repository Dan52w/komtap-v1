import { FormEvent, useEffect, useState } from "react";
import { Modal } from "../common/Modal";
import type { Producto } from "../../types/productos";

type Props = {
  productoInicial: Producto | null; // null = modo creación
  onGuardar: (producto: Producto) => Promise<void>;
  onCerrar: () => void;
};

const VACIO: Producto = {
  id: null,
  sku: null,
  nombre: "",
  categoria: "General",
  talla: "",
  color: null,
  costo_unitario: 0,
  precio_venta: 0,
  stock_actual: 0,
  stock_minimo: 3,
  estado: "activo",
};

export function ProductoFormModal({ productoInicial, onGuardar, onCerrar }: Props) {
  const [form, setForm] = useState<Producto>(productoInicial ?? VACIO);
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);
  const esEdicion = productoInicial !== null;

  useEffect(() => {
    setForm(productoInicial ?? VACIO);
  }, [productoInicial]);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    if (!form.nombre.trim() || !form.talla.trim()) {
      setError("Nombre y talla son obligatorios");
      return;
    }
    setGuardando(true);
    try {
      await onGuardar(form);
    } catch (err) {
      setError(String(err));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <Modal titulo={esEdicion ? "Editar prenda" : "Nueva prenda"} onCerrar={onCerrar}>
      <form className="form-grid" onSubmit={handleSubmit}>
        <label>Nombre</label>
        <input
          value={form.nombre}
          onChange={(e) => setForm({ ...form, nombre: e.target.value })}
          placeholder="Jeans Skinny Dama"
          autoFocus
          required
        />

        <label>Categoría</label>
        <input
          value={form.categoria}
          onChange={(e) => setForm({ ...form, categoria: e.target.value })}
          placeholder="Pantalones"
        />

        <div className="form-grid__cols-2">
          <div>
            <label>Talla</label>
            <input
              value={form.talla}
              onChange={(e) => setForm({ ...form, talla: e.target.value })}
              placeholder="M"
              required
            />
          </div>
          <div>
            <label>Color</label>
            <input
              value={form.color ?? ""}
              onChange={(e) => setForm({ ...form, color: e.target.value || null })}
              placeholder="Negro"
            />
          </div>
        </div>

        <label>SKU / código (opcional)</label>
        <input
          value={form.sku ?? ""}
          onChange={(e) => setForm({ ...form, sku: e.target.value || null })}
        />

        <div className="form-grid__cols-2">
          <div>
            <label>Costo unitario</label>
            <input
              type="number"
              min={0}
              step="1"
              placeholder="Ej. 25000"
              value={form.costo_unitario || ""}
              onChange={(e) => setForm({ ...form, costo_unitario: Number(e.target.value) })}
              required
            />
          </div>
          <div>
            <label>Precio de venta</label>
            <input
              type="number"
              min={0}
              step="1"
              placeholder="Ej. 45000"
              value={form.precio_venta || ""}
              onChange={(e) => setForm({ ...form, precio_venta: Number(e.target.value) })}
              required
            />
          </div>
        </div>

        {!esEdicion && (
          <>
            <label>Stock inicial</label>
            <input
              type="number"
              min={0}
              step="1"
              placeholder="Ej. 10"
              value={form.stock_actual || ""}
              onChange={(e) => setForm({ ...form, stock_actual: Number(e.target.value) })}
            />
          </>
        )}

        <div className="form-grid__cols-2">
          <div>
            <label>Stock mínimo (alerta)</label>
            <input
              type="number"
              min={0}
              step="1"
              placeholder="Ej. 3"
              value={form.stock_minimo || ""}
              onChange={(e) => setForm({ ...form, stock_minimo: Number(e.target.value) })}
            />
          </div>
          <div>
            <label>Estado</label>
            <select value={form.estado} onChange={(e) => setForm({ ...form, estado: e.target.value })}>
              <option value="activo">Activo</option>
              <option value="inactivo">Inactivo</option>
            </select>
          </div>
        </div>

        {esEdicion && (
          <p className="form-hint">
            El stock actual ({form.stock_actual} u.) solo se modifica desde "Ajustar stock" o
            registrando compras/ventas.
          </p>
        )}

        {error && <p className="form-error">{error}</p>}

        <div className="modal__acciones">
          <button type="button" className="btn btn--ghost" onClick={onCerrar}>
            Cancelar
          </button>
          <button type="submit" className="btn btn--primary" disabled={guardando}>
            {guardando ? "Guardando…" : esEdicion ? "Guardar cambios" : "Crear prenda"}
          </button>
        </div>
      </form>
    </Modal>
  );
}