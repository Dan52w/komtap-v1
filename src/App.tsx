import { ActivarLicencia } from "./shell/components/auth/ActivarLicencia";
import { Login } from "./shell/components/auth/Login";
import { RegistrarUsuario } from "./shell/components/auth/RegistrarUsuario";
import { Dashboard } from "./shell/components/dashboard/Dashboard";
import { AppShell } from "./shell/components/layout/AppShell";
import { AuthProvider, useAuth } from "./shell/context/AuthContext";

function FlujoPrincipal() {
  const { fase, error } = useAuth();

  switch (fase) {
    case "cargando":
      return <div className="centered-screen">Verificando licencia…</div>;
    case "licencia_inactiva":
      return (
        <div className="centered-screen">
          {error && <p className="form-error">{error}</p>}
          <ActivarLicencia />
        </div>
      );
    case "requiere_registro":
      return (
        <div className="centered-screen">
          <RegistrarUsuario />
        </div>
      );
    case "requiere_login":
      return (
        <div className="centered-screen">
          <Login />
        </div>
      );
    case "autenticado":
      return (
        <AppShell>
          <Dashboard />
        </AppShell>
      );
  }
}

function App() {
  return (
    <AuthProvider>
      <FlujoPrincipal />
    </AuthProvider>
  );
}

export default App;