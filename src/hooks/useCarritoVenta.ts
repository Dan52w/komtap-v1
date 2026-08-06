import { useState } from "react";
import type { Producto } from "../shell/types/productos";

export type ItemCarrito = { producto: Producto; cantidad: number; precio: number };

export function useCarritoVenta() {
  const [items, setItems] = useState<ItemCarrito[]>([]);

  const agregar = (producto: Producto): string | null => {
    if (items.some((x) => x.producto.id === producto.id)) {
      return "Ese artículo ya está en la venta.";
    }
    setItems((prev) => [...prev, { producto, cantidad: 1, precio: producto.precio_venta }]);
    return null;
  };

  const actualizar = (index: number, cambios: Partial<Pick<ItemCarrito, "cantidad" | "precio">>) => {
    setItems((prev) =>
      prev.map((item, i) => {
        if (i !== index) return item;
        // clamp de cantidad al stock disponible para feedback inmediato
        const cantidad = cambios.cantidad !== undefined
          ? Math.min(Math.max(1, cambios.cantidad), item.producto.stock_actual)
          : item.cantidad;
        return { ...item, ...cambios, cantidad };
      })
    );
  };

  const quitar = (index: number) => setItems((prev) => prev.filter((_, i) => i !== index));
  const limpiar = () => setItems([]);
  const total = (descuento: number) =>
    Math.max(0, items.reduce((sum, x) => sum + x.cantidad * x.precio, 0) - descuento);

  return { items, agregar, actualizar, quitar, limpiar, total };
}