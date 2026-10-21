import { useState } from "react";
import { useAuth } from "../../context/AuthContext";
import { Sidebar } from "./Sidebar";
import { Dashboard } from "../dashboard/Dashboard";
import { Inventario } from "../productos/Inventario";
import { Proveedores } from "../proveedores/Proveedores";
import { Ventas } from "../ventas/Ventas";
import { Clientes } from "../clientes/Clientes";
import { Gastos } from "../gastos/Gastos";
import { Reportes } from "../reportes/Reportes";
import { Ajustes } from "../ajustes/Ajustes";
import type { Modulo } from "../../types/navegacion";

export function AppShell() {
  const { usuario, cerrarSesion } = useAuth();
  const [moduloActivo, setModuloActivo] = useState<Modulo>("dashboard");

  return (
    <div className="app-shell">
      <Sidebar moduloActivo={moduloActivo} onNavegar={setModuloActivo} />
      <div className="app-main">
        <header className="app-header">
          <div className="app-header__identity">
            <strong>Komtap</strong>
            <span>Gestión comercial</span>
          </div>
          <div className="app-header__usuario">
            <span>{usuario?.username}</span>
            <button onClick={cerrarSesion}>Cerrar sesión</button>
          </div>
        </header>
        <main className="app-content">
          {moduloActivo === "dashboard" && <Dashboard />}
          {moduloActivo === "inventario" && <Inventario />}
          {moduloActivo === "proveedores" && <Proveedores />}
          {moduloActivo === "ventas" && <Ventas />}
          {moduloActivo === "clientes" && <Clientes />}
          {moduloActivo === "gastos" && <Gastos />}
          {moduloActivo === "reportes" && <Reportes />}
          {moduloActivo === "ajustes" && <Ajustes />}
        </main>
      </div>
    </div>
  );
}
