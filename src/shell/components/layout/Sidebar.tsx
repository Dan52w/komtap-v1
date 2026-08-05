import type { Modulo } from "../../types/navegacion";

const NAV_ITEMS: { label: string; modulo: Modulo | null }[] = [
  { label: "Dashboard", modulo: "dashboard" },
  { label: "Inventario", modulo: "inventario" },
  { label: "Proveedores", modulo: null },
  { label: "Ventas", modulo: null },
  { label: "Gastos", modulo: null },
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
              {item.label}
            </button>
          ) : (
            <div key={item.label} className="app-sidebar__link proximamente">
              {item.label}
              <span className="app-sidebar__tag">pronto</span>
            </div>
          )
        )}
      </nav>
    </aside>
  );
}