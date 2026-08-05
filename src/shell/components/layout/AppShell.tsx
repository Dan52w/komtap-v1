import { useState } from "react";
import { useAuth } from "../../context/AuthContext";
import { Sidebar } from "./Sidebar";
import { Dashboard } from "../dashboard/Dashboard";
import { Inventario } from "../productos/Inventario";
import type { Modulo } from "../../types/navegacion";

export function AppShell() {
  const { usuario, cerrarSesion } = useAuth();
  const [moduloActivo, setModuloActivo] = useState<Modulo>("dashboard");

  return (
    <div className="app-shell">
      <Sidebar moduloActivo={moduloActivo} onNavegar={setModuloActivo} />
      <div className="app-main">
        <header className="app-header">
          <div className="app-header__usuario">
            <span>{usuario?.username}</span>
            <button onClick={cerrarSesion}>Cerrar sesión</button>
          </div>
        </header>
        <main className="app-content">
          {moduloActivo === "dashboard" ? <Dashboard /> : <Inventario />}
        </main>
      </div>
    </div>
  );
}