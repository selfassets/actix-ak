"use client";

import { useTheme } from "next-themes";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";

const themes = [
  { id: "light", label: "浅色", icon: "☀️", color: "bg-white border-zinc-300" },
  {
    id: "dark",
    label: "深色",
    icon: "🌙",
    color: "bg-zinc-900 border-zinc-700",
  },
  {
    id: "rose",
    label: "玫瑰",
    icon: "🌹",
    color: "bg-rose-950 border-rose-700",
  },
  {
    id: "ocean",
    label: "海洋",
    icon: "🌊",
    color: "bg-sky-950 border-sky-700",
  },
  {
    id: "forest",
    label: "森林",
    icon: "🌲",
    color: "bg-emerald-950 border-emerald-700",
  },
];

export function ThemeSwitcher() {
  const { theme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  useEffect(() => setMounted(true), []);

  if (!mounted) return null;

  return (
    <div className="flex flex-wrap gap-1.5 px-1">
      {themes.map((t) => (
        <Button
          key={t.id}
          variant={theme === t.id ? "default" : "ghost"}
          size="sm"
          onClick={() => setTheme(t.id)}
          className={`h-7 px-2 text-xs gap-1 ${theme === t.id ? "" : "opacity-70 hover:opacity-100"}`}
          title={t.label}
        >
          <span>{t.icon}</span>
          <span className="hidden sm:inline">{t.label}</span>
        </Button>
      ))}
    </div>
  );
}
