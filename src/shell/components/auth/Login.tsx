import { FormEvent, useState } from "react";
import { auth } from "../../lib/api";
import { useAuth } from "../../context/AuthContext";
import { AuthCard } from "./AuthCard";

export function Login() {
  const { onLogin } = useAuth();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [cargando, setCargando] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    setCargando(true);
    try {
      const usuario = await auth.iniciarSesion(username.trim(), password);
      onLogin(usuario);
    } catch (err) {
      setError(String(err));
    } finally {
      setCargando(false);
    }
  };

  return (
    <AuthCard titulo="Iniciar sesión" subtitulo="Bienvenido de nuevo a tu tienda.">
      <form onSubmit={handleSubmit}>
        <label htmlFor="username">Usuario</label>
        <input
          id="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoFocus
          autoComplete="username"
          required
        />

        <label htmlFor="password">Contraseña</label>
        <input
          id="password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          autoComplete="current-password"
          required
        />

        {error && <p className="form-error">{error}</p>}
        <button type="submit" className="btn btn--primary btn--block" disabled={cargando}>
          {cargando ? "Ingresando…" : "Ingresar"}
        </button>
      </form>
    </AuthCard>
  );
}
