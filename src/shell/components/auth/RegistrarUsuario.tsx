import { FormEvent, useState } from "react";
import { auth } from "../../lib/api";
import { useAuth } from "../../context/AuthContext";

export function RegistrarUsuario() {
  const { onUsuarioRegistrado } = useAuth();
  const [codigoMaestro, setCodigoMaestro] = useState("");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirmar, setConfirmar] = useState("");
  const [nombreCompleto, setNombreCompleto] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [cargando, setCargando] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);

    if (password !== confirmar) {
      setError("Las contraseñas no coinciden");
      return;
    }

    setCargando(true);
    try {
      const usuario = await auth.registrarUsuario({
        codigoMaestro: codigoMaestro.trim(),
        username: username.trim(),
        password,
        nombreCompleto: nombreCompleto.trim() || undefined,
      });
      onUsuarioRegistrado(usuario);
    } catch (err) {
      setError(String(err));
    } finally {
      setCargando(false);
    }
  };

  return (
    <div className="auth-card">
      <h2>Crear usuario administrador</h2>
      <p>Este registro solo puede hacerse una vez.</p>
      <form onSubmit={handleSubmit}>
        <label htmlFor="codigoMaestro">Código maestro de registro</label>
        <input
          id="codigoMaestro"
          value={codigoMaestro}
          onChange={(e) => setCodigoMaestro(e.target.value)}
          required
        />

        <label htmlFor="nombreCompleto">Nombre completo (opcional)</label>
        <input
          id="nombreCompleto"
          value={nombreCompleto}
          onChange={(e) => setNombreCompleto(e.target.value)}
        />

        <label htmlFor="username">Usuario</label>
        <input
          id="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          required
        />

        <label htmlFor="password">Contraseña</label>
        <input
          id="password"
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />

        <label htmlFor="confirmar">Confirmar contraseña</label>
        <input
          id="confirmar"
          type="password"
          value={confirmar}
          onChange={(e) => setConfirmar(e.target.value)}
          required
        />

        {error && <p className="form-error">{error}</p>}
        <button type="submit" disabled={cargando}>
          {cargando ? "Creando…" : "Crear usuario"}
        </button>
      </form>
    </div>
  );
}