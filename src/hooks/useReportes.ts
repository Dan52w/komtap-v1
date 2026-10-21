import { useCallback, useEffect, useState } from "react";
import { gastos as gastosApi, proveedores as proveedoresApi, reportes as reportesApi, ventas as ventasApi } from "../shell/lib/api";
import type { DashboardFinanciero } from "../shell/types/reportes";
import type { Gasto } from "../shell/types/gastos";
import type { Venta } from "../shell/types/ventas";
import type { FacturaPendiente } from "../shell/types/proveedores";

export type FacturaConProveedor = FacturaPendiente & { proveedor: string };

export function useReportes(mes: string) {
  const [financiero, setFinanciero] = useState<DashboardFinanciero | null>(null);
  const [ventas, setVentas] = useState<Venta[]>([]);
  const [gastos, setGastos] = useState<Gasto[]>([]);
  const [facturasPendientes, setFacturasPendientes] = useState<FacturaConProveedor[]>([]);
  const [deudaProveedores, setDeudaProveedores] = useState(0);
  const [cargando, setCargando] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const cargar = useCallback(async () => {
    setCargando(true);
    setError(null);
    try {
      // Una sola consulta trae todas las facturas pendientes con el nombre
      // del proveedor (antes era una consulta por proveedor: patrón N+1).
      const [fin, v, g, pendientes, deuda] = await Promise.all([
        reportesApi.dashboardFinancieroMes(mes),
        ventasApi.listar({ mes, limite: 500 }),
        gastosApi.listar(mes),
        proveedoresApi.listarFacturasPendientes(),
        proveedoresApi.deudaTotalGlobal(),
      ]);

      setFinanciero(fin);
      setVentas(v);
      setGastos(g);
      setFacturasPendientes(pendientes.map((f) => ({ ...f, proveedor: f.proveedor_nombre })));
      setDeudaProveedores(deuda);
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  }, [mes]);

  useEffect(() => {
    void cargar();
  }, [cargar]);

  return { financiero, ventas, gastos, facturasPendientes, deudaProveedores, cargando, error, recargar: cargar };
}
