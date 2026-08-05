import { useCallback, useEffect, useState } from "react";
import { proveedores, reportes } from "../shell/lib/api";
import { DashboardFinanciero, Periodo, ProductoCritico, VentasPorMetodoPago } from "../shell/types/reportes";

export function useDashboard(periodo: Periodo, mesActual: string) {
  const [financiero, setFinanciero] = useState<DashboardFinanciero | null>(null);
  const [ventasPorMetodo, setVentasPorMetodo] = useState<VentasPorMetodoPago[]>([]);
  const [stockCritico, setStockCritico] = useState<ProductoCritico[]>([]);
  const [deudaProveedores, setDeudaProveedores] = useState(0);
  const [cargando, setCargando] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const cargar = useCallback(async () => {
    setCargando(true);
    setError(null);
    try {
      const [fin, metodos, critico, deuda] = await Promise.all([
        reportes.dashboardFinanciero(periodo),
        reportes.ventasPorMetodoPago(mesActual),
        reportes.productosStockCritico(),
        proveedores.deudaTotalGlobal(),
      ]);
      setFinanciero(fin);
      setVentasPorMetodo(metodos);
      setStockCritico(critico);
      setDeudaProveedores(deuda);
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  }, [periodo, mesActual]);

  useEffect(() => {
    cargar();
  }, [cargar]);

  return { financiero, ventasPorMetodo, stockCritico, deudaProveedores, cargando, error, recargar: cargar };
}