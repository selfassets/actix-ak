"use client";

import * as React from "react";

type ThemeColor = "default" | "blue" | "green" | "rose" | "ocean" | "forest";

interface ThemeColorProviderProps {
  children: React.ReactNode;
  defaultThemeColor?: ThemeColor;
  storageKey?: string;
}

interface ThemeColorProviderState {
  themeColor: ThemeColor;
  setThemeColor: (themeColor: ThemeColor) => void;
}

const initialState: ThemeColorProviderState = {
  themeColor: "default",
  setThemeColor: () => null,
};

const ThemeColorProviderContext =
  React.createContext<ThemeColorProviderState>(initialState);

export function ThemeColorProvider({
  children,
  defaultThemeColor = "default",
  storageKey = "vite-ui-theme-color",
  ...props
}: ThemeColorProviderProps) {
  const [themeColor, setThemeColor] =
    React.useState<ThemeColor>(defaultThemeColor);
  const [isMounted, setIsMounted] = React.useState(false);

  React.useEffect(() => {
    const stored = localStorage.getItem(storageKey) as ThemeColor;
    if (stored) {
      setThemeColor(stored);
    }
    setIsMounted(true);
  }, [storageKey]);

  React.useEffect(() => {
    if (!isMounted) return;
    const root = window.document.body;
    root.removeAttribute("data-theme-color");
    if (themeColor !== "default") {
      root.setAttribute("data-theme-color", themeColor);
    }
  }, [themeColor, isMounted]);

  const value = {
    themeColor,
    setThemeColor: (color: ThemeColor) => {
      localStorage.setItem(storageKey, color);
      setThemeColor(color);
    },
  };

  return (
    <ThemeColorProviderContext.Provider {...props} value={value}>
      {children}
    </ThemeColorProviderContext.Provider>
  );
}

export const useThemeColor = () => {
  const context = React.useContext(ThemeColorProviderContext);

  if (context === undefined)
    throw new Error("useThemeColor must be used within a ThemeColorProvider");

  return context;
};
