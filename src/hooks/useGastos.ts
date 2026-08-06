import { useCallback, useEffect, useState } from "react";
import { gastos as api } from "../shell/lib/api";
import type { Gasto, NuevoGasto } from "../shell/types/gastos";

export function useGastos(mes: string) {
  const [lista, setLista] = useState<Gasto[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [cargando, setCargando] = useState(true);

  const cargar = useCallback(async () => {
    setCargando(true);
    setError(null);
    try {
      setLista(await api.listar(mes));
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  }, [mes]);

  useEffect(() => {
    void cargar();
  }, [cargar]);

  const crear = async (gasto: NuevoGasto) => {
    await api.crear(gasto);
    await cargar();
  };

  const eliminar = async (id: number) => {
    await api.eliminar(id);
    await cargar();
  };

  return { lista, cargando, error, crear, eliminar, recargar: cargar };
}