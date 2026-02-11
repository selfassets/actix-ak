"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import { getHealth } from "@/lib/api";

const modules = [
  {
    title: "交易所 & 品种",
    description: "查看国内各期货交易所及其品种映射信息",
    href: "/futures/exchanges",
    icon: "🏛️",
    gradient: "from-blue-500/20 to-cyan-500/20",
    border: "border-blue-500/30",
  },
  {
    title: "实时行情",
    description: "查询期货合约的实时行情数据",
    href: "/futures/realtime",
    icon: "⚡",
    gradient: "from-amber-500/20 to-orange-500/20",
    border: "border-amber-500/30",
  },
  {
    title: "主力合约",
    description: "主力连续合约一览表及行情走势",
    href: "/futures/main",
    icon: "🎯",
    gradient: "from-green-500/20 to-emerald-500/20",
    border: "border-green-500/30",
  },
  {
    title: "交易费用",
    description: "期货品种手续费及交易规则参照",
    href: "/futures/fees",
    icon: "💰",
    gradient: "from-purple-500/20 to-pink-500/20",
    border: "border-purple-500/30",
  },
  {
    title: "外盘期货",
    description: "国际市场期货品种实时行情数据",
    href: "/futures/foreign",
    icon: "🌍",
    gradient: "from-teal-500/20 to-sky-500/20",
    border: "border-teal-500/30",
  },
  {
    title: "股票数据",
    description: "A 股实时行情及历史 K 线数据",
    href: "/stocks",
    icon: "📉",
    gradient: "from-rose-500/20 to-red-500/20",
    border: "border-rose-500/30",
  },
];

export default function HomePage() {
  const [health, setHealth] = useState<"loading" | "online" | "offline">(
    "loading",
  );

  useEffect(() => {
    getHealth()
      .then((res) => setHealth(res.success ? "online" : "offline"))
      .catch(() => setHealth("offline"));
  }, []);

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
          数据展示平台
        </h1>
        <p className="text-muted-foreground">
          基于 AkShare 的期货与股票实时数据查询系统
        </p>
      </div>

      {/* Health Status */}
      <Card className="border-dashed">
        <CardHeader className="pb-2">
          <CardTitle className="flex items-center gap-2 text-base font-medium">
            <span className="text-lg">🖥️</span>
            后端服务状态
          </CardTitle>
        </CardHeader>
        <CardContent className="flex items-center justify-between">
          <div className="text-xs text-muted-foreground">
            API Server: localhost:8080
          </div>
          {health === "loading" ? (
            <Skeleton className="h-6 w-16 rounded-full" />
          ) : (
            <Badge
              variant={health === "online" ? "default" : "destructive"}
              className="gap-1.5"
            >
              <span
                className={`w-2 h-2 rounded-full ${health === "online" ? "bg-green-400 animate-pulse" : "bg-red-400"}`}
              />
              {health === "online" ? "在线" : "离线"}
            </Badge>
          )}
        </CardContent>
      </Card>

      {/* Module Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {modules.map((mod) => (
          <Link key={mod.href} href={mod.href}>
            <Card
              className={`group h-full cursor-pointer transition-all duration-300 hover:scale-[1.02] hover:shadow-lg hover:shadow-primary/5 border ${mod.border} bg-gradient-to-br ${mod.gradient}`}
            >
              <CardHeader className="pb-3">
                <div className="flex items-center gap-3">
                  <span className="text-2xl group-hover:scale-110 transition-transform duration-300">
                    {mod.icon}
                  </span>
                  <CardTitle className="text-base">{mod.title}</CardTitle>
                </div>
              </CardHeader>
              <CardContent>
                <p className="text-sm text-muted-foreground">
                  {mod.description}
                </p>
              </CardContent>
            </Card>
          </Link>
        ))}
      </div>

      {/* Stats Overview */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        {[
          { label: "交易所", value: "6", icon: "🏛️" },
          { label: "期货品种", value: "70+", icon: "📊" },
          { label: "外盘品种", value: "30+", icon: "🌐" },
          { label: "API 接口", value: "30+", icon: "🔗" },
        ].map((stat) => (
          <Card key={stat.label} className="text-center">
            <CardHeader className="pt-6 pb-2">
              <span className="text-2xl mx-auto">{stat.icon}</span>
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">{stat.value}</div>
              <p className="text-xs text-muted-foreground mt-1">{stat.label}</p>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
