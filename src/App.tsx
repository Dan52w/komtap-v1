import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type EstadoLicencia = {
  activa: boolean;
  tipo_licencia: string | null;
  fecha_expiracion: string | null;
};

/**
 * Punto de entrada de la interfaz. Aquí solo se resuelve el flujo mínimo de
 * arranque descrito en HU-11 (licencia -> registro de usuario único -> login).
 * Las pantallas de cada módulo (proveedores, inventario, ventas, gastos,
 * dashboard) se agregan como componentes en src/components/.
 */
function App() {
  const [licencia, setLicencia] = useState<EstadoLicencia | null>(null);
  const [cargando, setCargando] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<EstadoLicencia>("estado_licencia")
      .then(setLicencia)
      .catch((e) => setError(String(e)))
      .finally(() => setCargando(false));
  }, []);

  return (
    <div className="app-shell">
      <header className="app-header">
        <h1>Komtap</h1>
      </header>
      <main className="app-content">
        {cargando && <p>Verificando licencia…</p>}
        {error && <p style={{ color: "crimson" }}>{error}</p>}
        {!cargando && !error && licencia && (
          <p>
            Licencia {licencia.activa ? "activa" : "inactiva"}
            {licencia.tipo_licencia ? ` (${licencia.tipo_licencia})` : ""}
          </p>
        )}
        {/* TODO: enrutar a Activación de licencia / Registro / Login / Dashboard
            según el estado devuelto por `estado_licencia` y `existe_usuario_registrado`. */}
      </main>
    </div>
  );
}

export default App;
