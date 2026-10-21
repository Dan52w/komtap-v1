import { FormEvent, useState } from "react";
import { auth } from "../../lib/api";
import { useAuth } from "../../context/AuthContext";

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
    <div className="auth-card">
      <h2>Crear usuario administrador</h2>
      <p>Este registro solo puede hacerse una vez.</p>
      <form onSubmit={handleSubmit}>
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

        <label htmlFor="codigoAdmin">Código para editar o eliminar ventas</label>
        <input
          id="codigoAdmin"
          type="password"
          inputMode="numeric"
          minLength={4}
          value={codigoAdmin}
          onChange={(e) => setCodigoAdmin(e.target.value)}
          required
        />

        <label htmlFor="confirmarCodigoAdmin">Confirmar código de seguridad</label>
        <input
          id="confirmarCodigoAdmin"
          type="password"
          inputMode="numeric"
          minLength={4}
          value={confirmarCodigoAdmin}
          onChange={(e) => setConfirmarCodigoAdmin(e.target.value)}
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
