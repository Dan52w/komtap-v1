import { FormEvent, useState } from "react";
import { auth } from "../../lib/api";
import { useAuth } from "../../context/AuthContext";
import { AuthCard } from "./AuthCard";

export function ActivarLicencia() {
  const { onLicenciaActivada } = useAuth();
  const [codigo, setCodigo] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [cargando, setCargando] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    setCargando(true);
    try {
      const estado = await auth.activarLicencia(codigo.trim());
      onLicenciaActivada(estado);
    } catch (err) {
      setError(String(err));
    } finally {
      setCargando(false);
    }
  };

  return (
    <AuthCard titulo="Activación de licencia" subtitulo="Ingresa el código de activación entregado para usar Komtap.">
      <form onSubmit={handleSubmit}>
        <label htmlFor="codigo">Código de activación</label>
        <input
          id="codigo"
          value={codigo}
          onChange={(e) => setCodigo(e.target.value)}
          placeholder="KOMTAP-XXXX-XXXX-XXXX"
          autoFocus
          autoComplete="off"
          spellCheck={false}
          required
        />
        {error && <p className="form-error">{error}</p>}
        <button type="submit" className="btn btn--primary btn--block" disabled={cargando || !codigo.trim()}>
          {cargando ? "Activando…" : "Activar licencia"}
        </button>
      </form>
    </AuthCard>
  );
}
