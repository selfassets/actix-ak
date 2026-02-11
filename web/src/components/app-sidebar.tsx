"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import {
  Sheet,
  SheetContent,
  SheetTrigger,
  SheetTitle,
} from "@/components/ui/sheet";

const navItems = [
  {
    title: "首页",
    href: "/",
    icon: "📊",
  },
  {
    title: "期货",
    icon: "📈",
    children: [
      { title: "交易所 & 品种", href: "/futures/exchanges", icon: "🏛️" },
      { title: "实时行情", href: "/futures/realtime", icon: "⚡" },
      { title: "主力合约", href: "/futures/main", icon: "🎯" },
      { title: "交易费用", href: "/futures/fees", icon: "💰" },
      { title: "外盘期货", href: "/futures/foreign", icon: "🌍" },
    ],
  },
  {
    title: "股票",
    icon: "📉",
    children: [{ title: "股票列表", href: "/stocks", icon: "📋" }],
  },
];

function NavContent({ onNavigate }: { onNavigate?: () => void }) {
  const pathname = usePathname();
  const [openGroups, setOpenGroups] = useState<Record<string, boolean>>({
    期货: true,
    股票: true,
  });

  const toggleGroup = (title: string) => {
    setOpenGroups((prev) => ({ ...prev, [title]: !prev[title] }));
  };

  return (
    <div className="flex flex-col h-full">
      <div className="p-6">
        <Link href="/" className="flex items-center gap-3" onClick={onNavigate}>
          <div className="w-9 h-9 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white font-bold text-sm shadow-lg">
            AK
          </div>
          <div>
            <h1 className="text-base font-bold tracking-tight">AkShare</h1>
            <p className="text-[11px] text-muted-foreground">数据展示平台</p>
          </div>
        </Link>
      </div>
      <Separator />
      <nav className="flex-1 py-4 px-3 space-y-1 overflow-y-auto">
        {navItems.map((item) =>
          item.children ? (
            <div key={item.title} className="mb-1">
              <button
                onClick={() => toggleGroup(item.title)}
                className="w-full flex items-center gap-2.5 px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground rounded-md transition-colors"
              >
                <span className="text-base">{item.icon}</span>
                <span>{item.title}</span>
                <svg
                  className={`ml-auto w-4 h-4 transition-transform ${openGroups[item.title] ? "rotate-90" : ""}`}
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 5l7 7-7 7"
                  />
                </svg>
              </button>
              {openGroups[item.title] && (
                <div className="ml-3 pl-3 border-l border-border/50 space-y-0.5 mt-0.5">
                  {item.children.map((child) => (
                    <Link
                      key={child.href}
                      href={child.href}
                      onClick={onNavigate}
                      className={`flex items-center gap-2.5 px-3 py-1.5 text-sm rounded-md transition-all ${
                        pathname === child.href
                          ? "bg-primary text-primary-foreground font-medium shadow-sm"
                          : "text-muted-foreground hover:text-foreground hover:bg-muted"
                      }`}
                    >
                      <span className="text-sm">{child.icon}</span>
                      <span>{child.title}</span>
                    </Link>
                  ))}
                </div>
              )}
            </div>
          ) : (
            <Link
              key={item.href}
              href={item.href!}
              onClick={onNavigate}
              className={`flex items-center gap-2.5 px-3 py-2 text-sm rounded-md transition-all ${
                pathname === item.href
                  ? "bg-primary text-primary-foreground font-medium shadow-sm"
                  : "text-muted-foreground hover:text-foreground hover:bg-muted"
              }`}
            >
              <span className="text-base">{item.icon}</span>
              <span>{item.title}</span>
            </Link>
          ),
        )}
      </nav>
      <Separator />
      <div className="p-4">
        <div className="text-xs text-muted-foreground text-center">
          Powered by Actix + AkShare
        </div>
      </div>
    </div>
  );
}

export function AppSidebar() {
  const [open, setOpen] = useState(false);

  return (
    <>
      {/* Desktop sidebar */}
      <aside className="hidden md:flex w-64 border-r bg-card/50 backdrop-blur-xl flex-col fixed inset-y-0 left-0 z-30">
        <NavContent />
      </aside>

      {/* Mobile hamburger */}
      <div className="md:hidden fixed top-4 left-4 z-50">
        <Sheet open={open} onOpenChange={setOpen}>
          <SheetTrigger asChild>
            <Button
              variant="outline"
              size="icon"
              className="backdrop-blur-sm bg-background/80"
            >
              <svg
                className="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 6h16M4 12h16M4 18h16"
                />
              </svg>
            </Button>
          </SheetTrigger>
          <SheetContent side="left" className="w-64 p-0">
            <SheetTitle className="sr-only">导航菜单</SheetTitle>
            <NavContent onNavigate={() => setOpen(false)} />
          </SheetContent>
        </Sheet>
      </div>
    </>
  );
}
