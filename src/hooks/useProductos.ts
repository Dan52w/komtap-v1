import { useCallback, useEffect, useState } from "react";
import { productos as productosApi } from "../shell/lib/api";
import { FiltroProductos, Producto } from "../shell/types/productos";

export function useProductos(filtro: FiltroProductos) {
  const [lista, setLista] = useState<Producto[]>([]);
  const [cargando, setCargando] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const cargar = useCallback(async () => {
    setCargando(true);
    setError(null);
    try {
      setLista(await productosApi.listar(filtro));
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [filtro.categoria, filtro.talla, filtro.nombre, filtro.estado_stock]);

  useEffect(() => {
    cargar();
  }, [cargar]);

  return { lista, cargando, error, recargar: cargar };
}