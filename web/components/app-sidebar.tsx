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
import { ThemeSwitcher } from "@/components/theme-switcher";
import {
  LayoutDashboard,
  Globe,
  ChevronRight,
  Menu,
  Landmark,
  List,
  Zap,
  Target,
  Coins,
} from "lucide-react";
import { LucideIcon } from "lucide-react";
import { Logo } from "@/components/ui/logo";

interface NavItem {
  title: string;
  href: string;
  icon: LucideIcon;
  children?: NavItem[];
}

const navItems: NavItem[] = [
  {
    title: "首页",
    href: "/",
    icon: LayoutDashboard,
  },
  { title: "交易所 & 品种", href: "/futures/exchanges", icon: Landmark },
  { title: "品种列表", href: "/futures/symbols", icon: List },
  { title: "实时行情", href: "/futures/realtime", icon: Zap },
  { title: "主力合约", href: "/futures/main", icon: Target },
  { title: "交易费用", href: "/futures/fees", icon: Coins },
  { title: "外盘期货", href: "/futures/foreign", icon: Globe },
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
          <Logo />
          <div>
            <h1 className="text-base font-bold tracking-tight">Ak</h1>
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
                className="w-full flex items-center gap-2.5 px-3 py-2 text-sm font-medium text-muted-foreground hover:text-foreground rounded-md transition-colors group"
              >
                <item.icon className="w-4 h-4" />
                <span>{item.title}</span>
                <ChevronRight
                  className={`ml-auto w-4 h-4 transition-transform ${openGroups[item.title] ? "rotate-90" : ""}`}
                />
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
                      <child.icon className="w-4 h-4" />
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
              <item.icon className="w-4 h-4" />
              <span>{item.title}</span>
            </Link>
          ),
        )}
      </nav>
      <Separator />
      <div className="p-3 space-y-3">
        <ThemeSwitcher />
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
              <Menu className="w-5 h-5" />
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
