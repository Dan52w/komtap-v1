import { ReactNode } from "react";

type Props = {
  titulo: string;
  onCerrar: () => void;
  children: ReactNode;
};

export function Modal({ titulo, onCerrar, children }: Props) {
  return (
    <div className="modal-overlay" onClick={onCerrar}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal__header">
          <h3>{titulo}</h3>
          <button className="modal__cerrar" onClick={onCerrar} aria-label="Cerrar">
            ×
          </button>
        </div>
        {children}
      </div>
    </div>
  );
}