import { useEffect, useMemo, useState } from "react";
import { clientes as api } from "../../lib/api";
import type { Cliente } from "../../types/clientes";
import { Modal } from "../common/Modal";

function ClienteForm({
  inicial,
  onCerrar,
  onGuardar,
}: {
  inicial: Cliente | null;
  onCerrar: () => void;
  onGuardar: (datos: Cliente) => Promise<void>;
}) {
  const [nombre, setNombre] = useState(inicial?.nombre ?? "");
  const [telefono, setTelefono] = useState(inicial?.telefono ?? "");
  const [notas, setNotas] = useState(inicial?.notas ?? "");
  const [error, setError] = useState<string | null>(null);
  const [guardando, setGuardando] = useState(false);

  const guardar = async () => {
    setError(null);
    if (!nombre.trim()) {
      setError("El nombre del cliente es obligatorio");
      return;
    }
    setGuardando(true);
    try {
      await onGuardar({
        id: inicial?.id ?? null,
        nombre: nombre.trim(),
        telefono: telefono.trim() || null,
        notas: notas.trim() || null,
      });
    } catch (e) {
      setError(String(e));
    } finally {
      setGuardando(false);
    }
  };

  return (
    <Modal titulo={inicial ? "Editar cliente" : "Nuevo cliente"} onCerrar={onCerrar}>
      <div className="form-grid">
        <label>
          Nombre
          <input value={nombre} onChange={(e) => setNombre(e.target.value)} autoFocus placeholder="Ej. María Pérez" />
        </label>
        <label>
          Teléfono (opcional)
          <input value={telefono} onChange={(e) => setTelefono(e.target.value)} placeholder="Ej. 300 123 4567" />
        </label>
        <label>
          Notas (opcional)
          <textarea value={notas} onChange={(e) => setNotas(e.target.value)} rows={2} placeholder="Ej. Paga quincenal" />
        </label>
      </div>
      {error && <p className="form-error">{error}</p>}
      <div className="modal__acciones">
        <button type="button" className="btn btn--ghost" onClick={onCerrar}>Cancelar</button>
        <button type="button" className="btn btn--primary" disabled={guardando} onClick={() => void guardar()}>
          {guardando ? "Guardando…" : "Guardar"}
        </button>
      </div>
    </Modal>
  );
}

export function Clientes() {
  const [lista, setLista] = useState<Cliente[]>([]);
  const [busqueda, setBusqueda] = useState("");
  const [formAbierto, setFormAbierto] = useState(false);
  const [editando, setEditando] = useState<Cliente | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [cargando, setCargando] = useState(true);

  const cargar = async () => {
    setCargando(true);
    setError(null);
    try {
      setLista(await api.listar());
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  };

  useEffect(() => {
    void cargar();
  }, []);

  const filtrados = useMemo(() => {
    const t = busqueda.trim().toLowerCase();
    if (!t) return lista;
    return lista.filter((c) =>
      [c.nombre, c.telefono, c.notas].filter(Boolean).some((v) => String(v).toLowerCase().includes(t))
    );
  }, [busqueda, lista]);

  const eliminar = async (cliente: Cliente) => {
    if (!window.confirm(`¿Eliminar al cliente "${cliente.nombre}"?`)) return;
    try {
      await api.eliminar(cliente.id!);
      await cargar();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div className="module-page">
      <div className="module-header">
        <div>
          <p className="eyebrow">Cuentas por cobrar</p>
          <h1>Clientes</h1>
          <p>Registra a quién le vendes a crédito para identificar cada saldo pendiente.</p>
        </div>
        <div className="header-actions">
          <button className="btn btn--primary" onClick={() => { setEditando(null); setFormAbierto(true); }}>
            + Nuevo cliente
          </button>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}

      <div className="filtros-panel filtros-panel--simple">
        <div className="filtros-panel__campo">
          <label htmlFor="clientes-busqueda">Buscar cliente</label>
          <input
            id="clientes-busqueda"
            type="search"
            value={busqueda}
            onChange={(e) => setBusqueda(e.target.value)}
            placeholder="Buscar por nombre, teléfono o nota…"
          />
        </div>
      </div>

      {cargando ? (
        <p>Cargando clientes…</p>
      ) : filtrados.length === 0 ? (
        <div className="empty-card">
          {lista.length === 0
            ? "Aún no hay clientes registrados. Crea el primero para llevar las cuentas por cobrar."
            : "No encontramos clientes con esa búsqueda."}
        </div>
      ) : (
        <div className="tabla-card">
          <table className="tabla-base">
            <thead>
              <tr>
                <th>Nombre</th>
                <th>Teléfono</th>
                <th>Notas</th>
                <th />
              </tr>
            </thead>
            <tbody>
              {filtrados.map((c) => (
                <tr key={c.id}>
                  <td><strong>{c.nombre}</strong></td>
                  <td>{c.telefono ?? "—"}</td>
                  <td>{c.notas ?? "—"}</td>
                  <td className="tabla-base__acciones">
                    <button className="btn btn--ghost" onClick={() => { setEditando(c); setFormAbierto(true); }}>
                      Editar
                    </button>
                    <button className="btn-icono btn-icono--peligro" onClick={() => void eliminar(c)} title="Eliminar">
                      ×
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {formAbierto && (
        <ClienteForm
          inicial={editando}
          onCerrar={() => { setFormAbierto(false); setEditando(null); }}
          onGuardar={async (datos) => {
            if (datos.id) await api.actualizar(datos);
            else await api.crear(datos);
            setFormAbierto(false);
            setEditando(null);
            await cargar();
          }}
        />
      )}
    </div>
  );
}
