"use client";

import { useEffect, useState, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import Link from "next/link";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  getStockInfo,
  getStockHistory,
  StockInfo,
  StockHistoryData,
} from "@/lib/api";

function StockDetailContent() {
  const searchParams = useSearchParams();
  const symbol = searchParams.get("symbol") || "";

  const [stock, setStock] = useState<StockInfo | null>(null);
  const [history, setHistory] = useState<StockHistoryData[]>([]);
  const [loading, setLoading] = useState(!!symbol);
  const [historyLoading, setHistoryLoading] = useState(!!symbol);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!symbol) return;

    getStockInfo(symbol)
      .then((res) => {
        if (res.success && res.data) setStock(res.data);
        else setError(res.message);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));

    getStockHistory(symbol, 50)
      .then((res) => {
        if (res.success && res.data) setHistory(res.data);
      })
      .catch(() => {})
      .finally(() => setHistoryLoading(false));
  }, [symbol]);

  if (!symbol) {
    return (
      <div className="space-y-4">
        <Card>
          <CardContent className="py-12 text-center text-muted-foreground">
            请从股票列表选择一只股票查看详情
          </CardContent>
        </Card>
        <Link href="/stocks">
          <Button variant="outline">← 返回股票列表</Button>
        </Link>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="space-y-6">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-48 w-full rounded-xl" />
        <Skeleton className="h-64 w-full rounded-xl" />
      </div>
    );
  }

  if (error || !stock) {
    return (
      <div className="space-y-4">
        <Link href="/stocks">
          <Button variant="outline" size="sm">
            ← 返回列表
          </Button>
        </Link>
        <Card className="border-destructive">
          <CardContent className="py-6 text-center text-destructive">
            {error || "未找到股票数据"}
          </CardContent>
        </Card>
      </div>
    );
  }

  const isUp = stock.change >= 0;

  return (
    <div className="space-y-6">
      {/* Breadcrumb */}
      <div className="flex items-center gap-2 text-sm">
        <Link
          href="/stocks"
          className="text-muted-foreground hover:text-foreground transition-colors"
        >
          股票列表
        </Link>
        <span className="text-muted-foreground">/</span>
        <span className="font-medium">
          {stock.name} ({stock.symbol})
        </span>
      </div>

      {/* Stock Header Card */}
      <Card
        className={`border ${isUp ? "border-green-500/30" : "border-red-500/30"}`}
      >
        <CardContent className="py-6">
          <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
            <div>
              <h1 className="text-2xl font-bold">{stock.name}</h1>
              <p className="text-muted-foreground font-mono">{stock.symbol}</p>
            </div>
            <div className="text-right">
              <p
                className={`text-4xl font-bold font-mono ${isUp ? "text-green-400" : "text-red-400"}`}
              >
                {stock.current_price?.toFixed(2)}
              </p>
              <div className="flex items-center gap-2 justify-end mt-1">
                <Badge
                  variant={isUp ? "default" : "destructive"}
                  className="font-mono"
                >
                  {isUp ? "▲" : "▼"} {isUp ? "+" : ""}
                  {stock.change?.toFixed(2)}
                </Badge>
                <Badge
                  variant={isUp ? "default" : "destructive"}
                  className="font-mono"
                >
                  {isUp ? "+" : ""}
                  {stock.change_percent?.toFixed(2)}%
                </Badge>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Detail Grid */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {[
          { label: "今开", value: stock.open?.toFixed(2) },
          { label: "昨收", value: stock.prev_close?.toFixed(2) },
          {
            label: "最高",
            value: stock.high?.toFixed(2),
            color: "text-green-400",
          },
          {
            label: "最低",
            value: stock.low?.toFixed(2),
            color: "text-red-400",
          },
          { label: "成交量", value: stock.volume?.toLocaleString() },
          { label: "成交额", value: stock.amount?.toLocaleString() },
          {
            label: "市值",
            value: stock.market_cap
              ? (stock.market_cap / 100000000).toFixed(2) + " 亿"
              : "-",
          },
          { label: "更新时间", value: stock.updated_at },
        ].map((item) => (
          <Card key={item.label}>
            <CardContent className="py-4 text-center">
              <p className="text-xs text-muted-foreground">{item.label}</p>
              <p
                className={`text-lg font-mono font-medium mt-1 ${"color" in item ? item.color : ""}`}
              >
                {item.value || "-"}
              </p>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* History Table */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span>📊</span> 历史 K 线数据
          </CardTitle>
        </CardHeader>
        <CardContent>
          {historyLoading ? (
            <div className="space-y-2">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : history.length === 0 ? (
            <p className="text-center text-muted-foreground py-8">
              暂无历史数据
            </p>
          ) : (
            <div className="rounded-md border overflow-auto max-h-[500px]">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>日期</TableHead>
                    <TableHead className="text-right">开盘</TableHead>
                    <TableHead className="text-right">最高</TableHead>
                    <TableHead className="text-right">最低</TableHead>
                    <TableHead className="text-right">收盘</TableHead>
                    <TableHead className="text-right">成交量</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {history.map((h, i) => (
                    <TableRow key={i}>
                      <TableCell className="font-mono">{h.date}</TableCell>
                      <TableCell className="text-right font-mono">
                        {h.open?.toFixed(2)}
                      </TableCell>
                      <TableCell className="text-right font-mono text-green-400">
                        {h.high?.toFixed(2)}
                      </TableCell>
                      <TableCell className="text-right font-mono text-red-400">
                        {h.low?.toFixed(2)}
                      </TableCell>
                      <TableCell className="text-right font-mono font-medium">
                        {h.close?.toFixed(2)}
                      </TableCell>
                      <TableCell className="text-right font-mono text-xs">
                        {h.volume?.toLocaleString()}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

export default function StockDetailPage() {
  return (
    <Suspense
      fallback={
        <div className="space-y-6">
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-48 w-full rounded-xl" />
          <Skeleton className="h-64 w-full rounded-xl" />
        </div>
      }
    >
      <StockDetailContent />
    </Suspense>
  );
}
