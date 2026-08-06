import { useCallback, useEffect, useState } from "react";
import { gastos as gastosApi, proveedores as proveedoresApi, reportes as reportesApi, ventas as ventasApi } from "../shell/lib/api";
import type { DashboardFinanciero } from "../shell/types/reportes";
import type { Gasto } from "../shell/types/gastos";
import type { Venta } from "../shell/types/ventas";
import type { FacturaProveedor, Proveedor } from "../shell/types/proveedores";

export type FacturaConProveedor = FacturaProveedor & { proveedor: string };

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
      const [fin, v, g, proveedoresList, deuda] = await Promise.all([
        reportesApi.dashboardFinancieroMes(mes),
        ventasApi.listar({ mes, limite: 500 }),
        gastosApi.listar(mes),
        proveedoresApi.listar(),
        proveedoresApi.deudaTotalGlobal(),
      ]);

      const facturasPorProveedor = await Promise.all(
        (proveedoresList as Proveedor[]).map(async (prov) => {
          const facturas = await proveedoresApi.listarFacturas(prov.id!);
          return facturas.filter((f) => f.saldo_pendiente > 0).map((f) => ({ ...f, proveedor: prov.nombre }));
        })
      );

      setFinanciero(fin);
      setVentas(v);
      setGastos(g);
      setFacturasPendientes(facturasPorProveedor.flat());
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