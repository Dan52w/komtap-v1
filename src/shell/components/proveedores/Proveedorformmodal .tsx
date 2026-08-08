import { FormEvent, useEffect, useState } from "react";
import { Modal } from "../common/Modal";
import type { Proveedor } from "../../types/proveedores";

type Props = {
  proveedorInicial: Proveedor | null; // null = modo creación
  onGuardar: (proveedor: Proveedor) => Promise<void>;
  onCerrar: () => void;
};

const VACIO: Proveedor = {
  id: null,
  nombre: "",
  telefono: null,
  direccion: null,
  correo: null,
  notas: null,
};

export function ProveedorFormModal({ proveedorInicial, onGuardar, onCerrar }: Props) {
  const [form, setForm] = useState<Proveedor>(proveedorInicial ?? VACIO);
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);
  const esEdicion = proveedorInicial !== null;

  useEffect(() => {
    setForm(proveedorInicial ?? VACIO);
  }, [proveedorInicial]);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    if (!form.nombre.trim()) {
      setError("El nombre del proveedor es obligatorio");
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
    <Modal titulo={esEdicion ? "Editar proveedor" : "Nuevo proveedor"} onCerrar={onCerrar}>
      <form className="form-grid" onSubmit={handleSubmit}>
        <label>Nombre</label>
        <input
          value={form.nombre}
          onChange={(e) => setForm({ ...form, nombre: e.target.value })}
          placeholder="Textiles del Norte S.A.S."
          autoFocus
          required
        />

        <label>Teléfono</label>
        <input
          value={form.telefono ?? ""}
          onChange={(e) => setForm({ ...form, telefono: e.target.value || null })}
          placeholder="300 123 4567"
        />

        <label>Dirección</label>
        <input
          value={form.direccion ?? ""}
          onChange={(e) => setForm({ ...form, direccion: e.target.value || null })}
          placeholder="Calle 10 #5-20, Santa Marta"
        />

        <label>Correo</label>
        <input
          type="email"
          value={form.correo ?? ""}
          onChange={(e) => setForm({ ...form, correo: e.target.value || null })}
          placeholder="contacto@proveedor.com"
        />

        <label>Notas (opcional)</label>
        <textarea
          value={form.notas ?? ""}
          onChange={(e) => setForm({ ...form, notas: e.target.value || null })}
          placeholder="Condiciones de pago, contacto habitual, etc."
          rows={3}
        />

        {error && <p className="form-error">{error}</p>}

        <div className="modal__acciones">
          <button type="button" className="btn btn--ghost" onClick={onCerrar}>
            Cancelar
          </button>
          <button type="submit" className="btn btn--primary" disabled={guardando}>
            {guardando ? "Guardando…" : esEdicion ? "Guardar cambios" : "Crear proveedor"}
          </button>
        </div>
      </form>
    </Modal>
  );
}