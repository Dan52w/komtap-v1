import { ReactNode } from "react";
import { useAuth } from "../../context/AuthContext";
import { Sidebar } from "./Sidebar";

export function AppShell({ children }: { children: ReactNode }) {
  const { usuario, cerrarSesion } = useAuth();

  return (
    <div className="app-shell">
      <Sidebar />
      <div className="app-main">
        <header className="app-header">
          <div className="app-header__usuario">
            <span>{usuario?.username}</span>
            <button onClick={cerrarSesion}>Cerrar sesión</button>
          </div>
        </header>
        <main className="app-content">{children}</main>
      </div>
    </div>
  );
}