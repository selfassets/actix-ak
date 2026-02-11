"use client";

import { ThemeProvider as NextThemesProvider } from "next-themes";

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  return (
    <NextThemesProvider
      attribute="class"
      defaultTheme="dark"
      enableSystem
      themes={["light", "dark", "rose", "ocean", "forest"]}
      disableTransitionOnChange={false}
    >
      {children}
    </NextThemesProvider>
  );
}
