"use client";

import { createContext, useContext, useState, useCallback, ReactNode, useEffect } from "react";
import { cn } from "./cn";

export interface ToastProps {
  id: number;
  message: ReactNode;
  tone?: "neutral" | "success" | "danger";
  duration?: number;
}

type ToastCtx = {
  toasts: ToastProps[];
  push: (t: Omit<ToastProps, "id">) => void;
  dismiss: (id: number) => void;
};

const Ctx = createContext<ToastCtx | null>(null);

export function ToastProvider({ children }: { children: ReactNode }) {
  const [toasts, setToasts] = useState<ToastProps[]>([]);
  const dismiss = useCallback((id: number) => setToasts((xs) => xs.filter((t) => t.id !== id)), []);
  const push = useCallback((t: Omit<ToastProps, "id">) => {
    const id = Date.now() + Math.random();
    setToasts((xs) => [...xs, { ...t, id }]);
    const duration = t.duration ?? 3000;
    if (duration > 0) setTimeout(() => dismiss(id), duration);
  }, [dismiss]);
  return (
    <Ctx.Provider value={{ toasts, push, dismiss }}>
      {children}
      <div role="region" aria-live="polite" aria-label="Notifications" className="aph-toast-region">
        {toasts.map((t) => <Toast key={t.id} {...t} />)}
      </div>
    </Ctx.Provider>
  );
}

export function useToast() {
  const ctx = useContext(Ctx);
  if (!ctx) throw new Error("useToast must be inside <ToastProvider>");
  return ctx;
}

export function Toast({ message, tone = "neutral" }: ToastProps) {
  return <div className={cn("aph-toast", tone !== "neutral" && `aph-toast--${tone}`)}>{message}</div>;
}
