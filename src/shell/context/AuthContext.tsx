import { createContext, useCallback, useContext, useEffect, useState, ReactNode } from "react";
import { auth } from "../lib/api";
import type { EstadoLicencia, Usuario } from "../types/auth";

type Fase =
  | "cargando"
  | "licencia_inactiva"
  | "requiere_registro"
  | "requiere_login"
  | "autenticado";

type AuthContextValue = {
  fase: Fase;
  licencia: EstadoLicencia | null;
  usuario: Usuario | null;
  error: string | null;
  refrescarLicencia: () => Promise<void>;
  onLicenciaActivada: (l: EstadoLicencia) => void;
  onUsuarioRegistrado: (u: Usuario) => void;
  onLogin: (u: Usuario) => void;
  cerrarSesion: () => void;
};

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [fase, setFase] = useState<Fase>("cargando");
  const [licencia, setLicencia] = useState<EstadoLicencia | null>(null);
  const [usuario, setUsuario] = useState<Usuario | null>(null);
  const [error, setError] = useState<string | null>(null);

  const evaluarFlujo = useCallback(async () => {
    setError(null);
    try {
      const estado = await auth.estadoLicencia();
      setLicencia(estado);

      if (!estado.activa) {
        setFase("licencia_inactiva");
        return;
      }

      const existeUsuario = await auth.existeUsuarioRegistrado();
      setFase(existeUsuario ? "requiere_login" : "requiere_registro");
    } catch (e) {
      setError(String(e));
      setFase("licencia_inactiva");
    }
  }, []);

  useEffect(() => {
    evaluarFlujo();
  }, [evaluarFlujo]);

  const onLicenciaActivada = (l: EstadoLicencia) => {
    setLicencia(l);
    // Tras activar, hay que revisar si ya existe usuario o falta registrarlo.
    evaluarFlujo();
  };

  const onUsuarioRegistrado = (u: Usuario) => {
    setUsuario(u);
    setFase("autenticado");
  };

  const onLogin = (u: Usuario) => {
    setUsuario(u);
    setFase("autenticado");
  };

  const cerrarSesion = () => {
    setUsuario(null);
    setFase("requiere_login");
  };

  return (
    <AuthContext.Provider
      value={{
        fase,
        licencia,
        usuario,
        error,
        refrescarLicencia: evaluarFlujo,
        onLicenciaActivada,
        onUsuarioRegistrado,
        onLogin,
        cerrarSesion,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth debe usarse dentro de <AuthProvider>");
  return ctx;
}