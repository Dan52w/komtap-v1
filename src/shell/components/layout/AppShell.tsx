import { ReactNode } from "react";
import { useAuth } from "../../context/AuthContext";

export function AppShell({ children }: { children: ReactNode }) {
  const { usuario, cerrarSesion } = useAuth();

  return (
    <div className="app-shell">
      <header className="app-header">
        <h1>Komtap</h1>
        <div className="app-header__usuario">
          <span>{usuario?.username}</span>
          <button onClick={cerrarSesion}>Cerrar sesión</button>
        </div>
      </header>
      {/* La Sidebar con navegación a Productos / Proveedores / Ventas / Gastos
          se agrega cuando construyamos esos módulos. */}
      <main className="app-content">{children}</main>
    </div>
  );
}