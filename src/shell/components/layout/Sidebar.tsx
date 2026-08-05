import type { Modulo } from "../../types/navegacion";

const NAV_ITEMS: { label: string; modulo: Modulo | null; icon: string }[] = [
  { label: "Dashboard", modulo: "dashboard", icon: "◧" },
  { label: "Inventario", modulo: "inventario", icon: "▤" },
  { label: "Proveedores", modulo: null, icon: "⇄" },
  { label: "Ventas", modulo: null, icon: "◈" },
  { label: "Gastos", modulo: null, icon: "◎" },
];

type Props = {
  moduloActivo: Modulo;
  onNavegar: (modulo: Modulo) => void;
};

export function Sidebar({ moduloActivo, onNavegar }: Props) {
  return (
    <aside className="app-sidebar">
      <div className="app-sidebar__brand">
        <span className="app-sidebar__mark">K</span>
        <span className="app-sidebar__name">Komtap</span>
      </div>
      <div className="app-sidebar__seam" aria-hidden="true" />
      <nav className="app-sidebar__nav">
        {NAV_ITEMS.map((item) =>
          item.modulo ? (
            <button
              key={item.label}
              className={`app-sidebar__link ${item.modulo === moduloActivo ? "activo" : ""}`}
              onClick={() => onNavegar(item.modulo!)}
            >
              <span className="app-sidebar__icon" aria-hidden="true">
                {item.icon}
              </span>
              {item.label}
            </button>
          ) : (
            <div key={item.label} className="app-sidebar__link proximamente">
              <span className="app-sidebar__icon" aria-hidden="true">
                {item.icon}
              </span>
              {item.label}
              <span className="app-sidebar__tag">pronto</span>
            </div>
          )
        )}
      </nav>
    </aside>
  );
}