import { useCallback, useEffect, useState } from "react";
import { ventas } from "../shell/lib/api";
import type { TopPrenda } from "../shell/types/ventas";

export function useTopPrendas(dias: number | null, limite = 10) {
  const [lista, setLista] = useState<TopPrenda[]>([]);
  const [cargando, setCargando] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const cargar = useCallback(async () => {
    setCargando(true);
    setError(null);
    try {
      setLista(await ventas.topPrendas({ dias, limite }));
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  }, [dias, limite]);

  useEffect(() => {
    cargar();
  }, [cargar]);

  return { lista, cargando, error, recargar: cargar };
}