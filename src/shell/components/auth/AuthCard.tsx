import type { ReactNode } from "react";

type Props = {
  titulo: string;
  subtitulo?: string;
  children: ReactNode;
};

/** Tarjeta compartida de las pantallas de inicio: marca + título + contenido. */
export function AuthCard({ titulo, subtitulo, children }: Props) {
  return (
    <div className="auth-card">
      <div className="auth-card__brand">
        <span className="auth-card__mark" aria-hidden="true">K</span>
        <span className="auth-card__name">KOMTAP</span>
      </div>
      <h2>{titulo}</h2>
      {subtitulo && <p className="auth-card__sub">{subtitulo}</p>}
      {children}
    </div>
  );
}
