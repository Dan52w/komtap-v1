import { FormEvent, useState } from "react";
import { auth } from "../../lib/api";
import { useAuth } from "../../context/AuthContext";
import { AuthCard } from "./AuthCard";

export function RegistrarUsuario() {
  const { onUsuarioRegistrado } = useAuth();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [confirmar, setConfirmar] = useState("");
  const [codigoAdmin, setCodigoAdmin] = useState("");
  const [confirmarCodigoAdmin, setConfirmarCodigoAdmin] = useState("");
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
    if (codigoAdmin.trim().length < 4) {
      setError("El código de seguridad debe tener al menos 4 caracteres");
      return;
    }
    if (codigoAdmin !== confirmarCodigoAdmin) {
      setError("Los códigos de seguridad no coinciden");
      return;
    }

    setCargando(true);
    try {
      const usuario = await auth.registrarUsuario({
        username: username.trim(),
        password,
        codigoAdmin: codigoAdmin.trim(),
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
    <AuthCard titulo="Crear usuario administrador" subtitulo="Este registro solo puede hacerse una vez.">
      <form onSubmit={handleSubmit}>
        <label htmlFor="nombreCompleto">Nombre completo (opcional)</label>
        <input
          id="nombreCompleto"
          value={nombreCompleto}
          onChange={(e) => setNombreCompleto(e.target.value)}
          autoComplete="name"
        />

        <label htmlFor="username">Usuario</label>
        <input
          id="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoComplete="username"
          required
        />

        <div className="form-grid__cols-2">
          <label htmlFor="password">Contraseña
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              autoComplete="new-password"
              required
            />
          </label>
          <label htmlFor="confirmar">Confirmar contraseña
            <input
              id="confirmar"
              type="password"
              value={confirmar}
              onChange={(e) => setConfirmar(e.target.value)}
              autoComplete="new-password"
              required
            />
          </label>
        </div>

        <div className="form-grid__cols-2">
          <label htmlFor="codigoAdmin">Código de seguridad
            <input
              id="codigoAdmin"
              type="password"
              inputMode="numeric"
              minLength={4}
              value={codigoAdmin}
              onChange={(e) => setCodigoAdmin(e.target.value)}
              placeholder="Mínimo 4 caracteres"
              required
            />
          </label>
          <label htmlFor="confirmarCodigoAdmin">Confirmar código
            <input
              id="confirmarCodigoAdmin"
              type="password"
              inputMode="numeric"
              minLength={4}
              value={confirmarCodigoAdmin}
              onChange={(e) => setConfirmarCodigoAdmin(e.target.value)}
              required
            />
          </label>
        </div>
        <p className="form-hint">El código de seguridad autoriza editar o eliminar ventas.</p>

        {error && <p className="form-error">{error}</p>}
        <button type="submit" className="btn btn--primary btn--block" disabled={cargando}>
          {cargando ? "Creando…" : "Crear usuario"}
        </button>
      </form>
    </AuthCard>
  );
}
