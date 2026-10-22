import { useEffect, useState } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import { auth, respaldo, seguridad, seed } from "../../lib/api";
import { useAuth } from "../../context/AuthContext";
import type { EstadoLicencia } from "../../types/auth";
import { fechaHora } from "../../lib/format";

function nombreRespaldo(): string {
  const f = new Date();
  const p = (n: number) => String(n).padStart(2, "0");
  return `komtap-respaldo-${f.getFullYear()}${p(f.getMonth() + 1)}${p(f.getDate())}-${p(f.getHours())}${p(f.getMinutes())}.db`;
}

export function Ajustes() {
  const { licencia, refrescarLicencia } = useAuth();
  const [estadoLicencia, setEstadoLicencia] = useState<EstadoLicencia | null>(licencia);
  // Cambio de código en dos pasos: primero se verifica el actual.
  const [tieneCodigo, setTieneCodigo] = useState(false);
  const [pasoCodigo, setPasoCodigo] = useState<"verificar" | "nuevo">("nuevo");
  const [codigoActual, setCodigoActual] = useState("");
  const [codigo, setCodigo] = useState("");
  const [confirmar, setConfirmar] = useState("");
  const [mensaje, setMensaje] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [ocupado, setOcupado] = useState(false);

  useEffect(() => {
    auth.estadoLicencia().then(setEstadoLicencia).catch(() => undefined);
    seguridad
      .existeCodigoAdmin()
      .then((existe) => {
        setTieneCodigo(existe);
        setPasoCodigo(existe ? "verificar" : "nuevo");
      })
      .catch(() => undefined);
  }, []);

  const mostrarOk = (texto: string) => {
    setMensaje(texto);
    setError(null);
    window.setTimeout(() => setMensaje(null), 4000);
  };

  const limpiarCodigo = () => {
    setCodigoActual("");
    setCodigo("");
    setConfirmar("");
  };

  const verificarActual = async () => {
    setError(null);
    if (!codigoActual.trim()) {
      setError("Ingresa tu código actual para continuar");
      return;
    }
    setOcupado(true);
    try {
      const ok = await seguridad.verificarCodigoAdmin(codigoActual.trim());
      if (!ok) {
        setError("El código actual no es correcto");
        return;
      }
      limpiarCodigo();
      setPasoCodigo("nuevo");
    } catch (e) {
      setError(String(e));
    } finally {
      setOcupado(false);
    }
  };

  const cambiarCodigo = async () => {
    setError(null);
    if (codigo.trim().length < 4) {
      setError("El código debe tener al menos 4 caracteres");
      return;
    }
    if (codigo !== confirmar) {
      setError("Los códigos no coinciden");
      return;
    }
    setOcupado(true);
    try {
      await seguridad.establecerCodigoAdmin(codigo.trim());
      limpiarCodigo();
      setTieneCodigo(true);
      // Volver a pedir verificación en el próximo cambio.
      setPasoCodigo("verificar");
      mostrarOk("Código de seguridad actualizado.");
    } catch (e) {
      setError(String(e));
    } finally {
      setOcupado(false);
    }
  };

  const cargarEjemplo = async () => {
    setError(null);
    if (
      !window.confirm(
        "¿Cargar datos de ejemplo? Se agregarán clientes, proveedores y productos de muestra. Solo funciona si esas tablas están vacías.",
      )
    )
      return;
    setOcupado(true);
    try {
      const r = await seed.cargarDatosEjemplo();
      mostrarOk(
        `Datos de ejemplo cargados: ${r.clientes} clientes, ${r.proveedores} proveedores y ${r.productos} productos.`,
      );
    } catch (e) {
      setError(String(e));
    } finally {
      setOcupado(false);
    }
  };

  const exportar = async () => {
    setError(null);
    const ruta = await save({
      defaultPath: nombreRespaldo(),
      filters: [{ name: "Respaldo Komtap", extensions: ["db"] }],
    });
    if (!ruta) return;
    setOcupado(true);
    try {
      await respaldo.exportar(ruta);
      mostrarOk("Respaldo guardado correctamente.");
    } catch (e) {
      setError(String(e));
    } finally {
      setOcupado(false);
    }
  };

  const importar = async () => {
    setError(null);
    const ruta = await open({
      multiple: false,
      filters: [{ name: "Respaldo Komtap", extensions: ["db"] }],
    });
    if (!ruta || Array.isArray(ruta)) return;
    if (!window.confirm("¿Restaurar este respaldo? Se reemplazarán todos los datos actuales.")) return;
    setOcupado(true);
    try {
      await respaldo.importar(ruta);
      await refrescarLicencia();
      mostrarOk("Respaldo restaurado correctamente.");
    } catch (e) {
      setError(String(e));
    } finally {
      setOcupado(false);
    }
  };

  return (
    <div className="module-page">
      <div className="module-header">
        <div>
          <p className="eyebrow">Configuración</p>
          <h1>Ajustes</h1>
          <p>Licencia, seguridad y copias de seguridad de tu información.</p>
        </div>
      </div>

      {error && <p className="form-error">{error}</p>}
      {mensaje && <p className="form-ok">{mensaje}</p>}

      <div className="ajustes-grid">
        <section className="ajustes-card">
          <h3>Licencia</h3>
          {estadoLicencia ? (
            <dl className="detail-summary">
              <div><dt>Estado</dt><dd>{estadoLicencia.activa ? "Activa" : "Inactiva"}</dd></div>
              <div><dt>Tipo</dt><dd>{estadoLicencia.tipo_licencia === "permanente" ? "Permanente" : estadoLicencia.tipo_licencia === "temporal" ? "Temporal" : "—"}</dd></div>
              {estadoLicencia.fecha_expiracion && (
                <div><dt>Vence</dt><dd>{fechaHora(estadoLicencia.fecha_expiracion)}</dd></div>
              )}
            </dl>
          ) : (
            <p className="empty-state">No se pudo leer el estado de la licencia.</p>
          )}
        </section>

        <section className="ajustes-card">
          <h3>Código de seguridad</h3>
          <p>Protege la edición y eliminación de ventas. Para cambiarlo, primero confirma el código actual.</p>
          {pasoCodigo === "verificar" ? (
            <>
              <label>
                Código actual
                <input
                  type="password"
                  inputMode="numeric"
                  value={codigoActual}
                  onChange={(e) => setCodigoActual(e.target.value)}
                  placeholder="Ingresa tu código actual"
                  autoComplete="off"
                />
              </label>
              {error && <p className="form-error">{error}</p>}
              <div className="ajustes-card__acciones">
                <button className="btn btn--primary" disabled={ocupado} onClick={() => void verificarActual()}>
                  {ocupado ? "Verificando…" : "Verificar y continuar"}
                </button>
              </div>
            </>
          ) : (
            <>
              <label>
                {tieneCodigo ? "Nuevo código" : "Crea tu código"}
                <input
                  type="password"
                  inputMode="numeric"
                  value={codigo}
                  onChange={(e) => setCodigo(e.target.value)}
                  placeholder="Mínimo 4 caracteres"
                  autoComplete="new-password"
                />
              </label>
              <label>
                Confirmar código
                <input
                  type="password"
                  inputMode="numeric"
                  value={confirmar}
                  onChange={(e) => setConfirmar(e.target.value)}
                  autoComplete="new-password"
                />
              </label>
              {error && <p className="form-error">{error}</p>}
              <div className="ajustes-card__acciones">
                <button className="btn btn--primary" disabled={ocupado} onClick={() => void cambiarCodigo()}>
                  {ocupado ? "Guardando…" : tieneCodigo ? "Guardar nuevo código" : "Crear código"}
                </button>
                {tieneCodigo && (
                  <button
                    className="btn btn--ghost"
                    disabled={ocupado}
                    onClick={() => {
                      limpiarCodigo();
                      setError(null);
                      setPasoCodigo("verificar");
                    }}
                  >
                    Cancelar
                  </button>
                )}
              </div>
            </>
          )}
        </section>

        <section className="ajustes-card">
          <h3>Respaldo de la información</h3>
          <p>
            Guarda una copia de toda la base de datos (productos, ventas, proveedores,
            clientes y gastos) en un archivo <code>.db</code>. Guárdala en un lugar seguro.
          </p>
          <div className="ajustes-card__acciones">
            <button className="btn btn--primary" disabled={ocupado} onClick={() => void exportar()}>
              Descargar respaldo
            </button>
            <button className="btn btn--ghost" disabled={ocupado} onClick={() => void importar()}>
              Restaurar respaldo…
            </button>
          </div>
        </section>

        <section className="ajustes-card">
          <h3>Datos de ejemplo</h3>
          <p>
            Carga clientes, proveedores y productos de muestra para probar la aplicación.
            Solo funciona si esas tablas están vacías.
          </p>
          <div className="ajustes-card__acciones">
            <button className="btn btn--ghost" disabled={ocupado} onClick={() => void cargarEjemplo()}>
              {ocupado ? "Cargando…" : "Cargar datos de ejemplo"}
            </button>
          </div>
        </section>
      </div>
    </div>
  );
}
