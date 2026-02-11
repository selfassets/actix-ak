"use client";

import { useState, useEffect, Suspense } from "react";
import { useSearchParams } from "next/navigation";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { getFuturesInfo, getFuturesBatch, FuturesInfo } from "@/lib/api";

const quickSymbols = [
  "CU2602",
  "AL2602",
  "RB2605",
  "AU2602",
  "AG2602",
  "IF2603",
];

function PriceCard({ data }: { data: FuturesInfo }) {
  const isUp = data.change >= 0;
  return (
    <Card
      className={`border ${isUp ? "border-green-500/30" : "border-red-500/30"}`}
    >
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between">
          <CardTitle className="text-base font-mono">{data.symbol}</CardTitle>
          <Badge variant={isUp ? "default" : "destructive"} className="gap-1">
            {isUp ? "▲" : "▼"} {data.change_percent?.toFixed(2)}%
          </Badge>
        </div>
        <p className="text-xs text-muted-foreground">{data.name}</p>
      </CardHeader>
      <CardContent className="space-y-3">
        <div className="text-2xl font-bold font-mono">
          <span className={isUp ? "text-green-400" : "text-red-400"}>
            {data.current_price?.toFixed(2)}
          </span>
        </div>
        <div className="grid grid-cols-2 gap-2 text-xs">
          <div className="flex justify-between">
            <span className="text-muted-foreground">涨跌额</span>
            <span
              className={`font-mono ${isUp ? "text-green-400" : "text-red-400"}`}
            >
              {isUp ? "+" : ""}
              {data.change?.toFixed(2)}
            </span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">昨收</span>
            <span className="font-mono">{data.prev_close?.toFixed(2)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">开盘</span>
            <span className="font-mono">{data.open?.toFixed(2)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">成交量</span>
            <span className="font-mono">{data.volume?.toLocaleString()}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">最高</span>
            <span className="font-mono text-green-400">
              {data.high?.toFixed(2)}
            </span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">最低</span>
            <span className="font-mono text-red-400">
              {data.low?.toFixed(2)}
            </span>
          </div>
        </div>
        <div className="text-[10px] text-muted-foreground text-right">
          更新于 {data.updated_at}
        </div>
      </CardContent>
    </Card>
  );
}

function RealtimeContent() {
  const searchParams = useSearchParams();
  const urlSymbol = searchParams.get("symbol") || "";

  const [symbol, setSymbol] = useState(urlSymbol);
  const [results, setResults] = useState<FuturesInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (urlSymbol) {
      setSymbol(urlSymbol);
      setLoading(true);
      getFuturesInfo(urlSymbol)
        .then((res) => {
          if (res.success && res.data) setResults([res.data]);
          else setError(res.message);
        })
        .catch((e) => setError(e instanceof Error ? e.message : "请求失败"))
        .finally(() => setLoading(false));
    }
  }, [urlSymbol]);

  const handleSearch = async () => {
    if (!symbol.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const res = await getFuturesInfo(symbol.trim().toUpperCase());
      if (res.success && res.data) {
        setResults([res.data]);
      } else {
        setError(res.message);
        setResults([]);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "请求失败");
    }
    setLoading(false);
  };

  const handleBatchQuery = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await getFuturesBatch(quickSymbols);
      if (res.success && res.data) {
        setResults(res.data);
      } else {
        setError(res.message);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "请求失败");
    }
    setLoading(false);
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">实时行情</h1>
        <p className="text-muted-foreground text-sm mt-1">
          查询期货合约的实时行情数据
        </p>
      </div>

      {/* Search */}
      <Card>
        <CardContent className="py-4">
          <div className="flex gap-3">
            <Input
              placeholder="输入合约代码，如 CU2602、RB2605..."
              value={symbol}
              onChange={(e) => setSymbol(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSearch()}
              className="font-mono"
            />
            <Button onClick={handleSearch} disabled={loading}>
              {loading ? "查询中..." : "查询"}
            </Button>
            <Button
              variant="outline"
              onClick={handleBatchQuery}
              disabled={loading}
            >
              批量查询示例
            </Button>
          </div>
          <div className="flex gap-2 mt-3 flex-wrap">
            {quickSymbols.map((s) => (
              <Badge
                key={s}
                variant="secondary"
                className="cursor-pointer hover:bg-primary hover:text-primary-foreground transition-colors"
                onClick={() => {
                  setSymbol(s);
                  getFuturesInfo(s).then((res) => {
                    if (res.success && res.data) setResults([res.data]);
                  });
                }}
              >
                {s}
              </Badge>
            ))}
          </div>
        </CardContent>
      </Card>

      {/* Error */}
      {error && (
        <Card className="border-destructive">
          <CardContent className="py-4 text-center text-destructive">
            {error}
          </CardContent>
        </Card>
      )}

      {/* Loading */}
      {loading && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {Array.from({ length: 3 }).map((_, i) => (
            <Skeleton key={i} className="h-48 rounded-xl" />
          ))}
        </div>
      )}

      {/* Results - Cards */}
      {!loading && results.length > 0 && (
        <>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
            {results.map((r) => (
              <PriceCard key={r.symbol} data={r} />
            ))}
          </div>

          {/* Results - Table */}
          {results.length > 1 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">📊 数据汇总</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="rounded-md border overflow-auto">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>合约</TableHead>
                        <TableHead>名称</TableHead>
                        <TableHead className="text-right">当前价</TableHead>
                        <TableHead className="text-right">涨跌</TableHead>
                        <TableHead className="text-right">涨跌幅</TableHead>
                        <TableHead className="text-right">成交量</TableHead>
                        <TableHead className="text-right">成交额</TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {results.map((r) => {
                        const isUp = r.change >= 0;
                        return (
                          <TableRow key={r.symbol}>
                            <TableCell className="font-mono font-medium">
                              {r.symbol}
                            </TableCell>
                            <TableCell>{r.name}</TableCell>
                            <TableCell
                              className={`text-right font-mono ${isUp ? "text-green-400" : "text-red-400"}`}
                            >
                              {r.current_price?.toFixed(2)}
                            </TableCell>
                            <TableCell
                              className={`text-right font-mono ${isUp ? "text-green-400" : "text-red-400"}`}
                            >
                              {isUp ? "+" : ""}
                              {r.change?.toFixed(2)}
                            </TableCell>
                            <TableCell
                              className={`text-right font-mono ${isUp ? "text-green-400" : "text-red-400"}`}
                            >
                              {isUp ? "+" : ""}
                              {r.change_percent?.toFixed(2)}%
                            </TableCell>
                            <TableCell className="text-right font-mono">
                              {r.volume?.toLocaleString()}
                            </TableCell>
                            <TableCell className="text-right font-mono">
                              {r.amount?.toLocaleString()}
                            </TableCell>
                          </TableRow>
                        );
                      })}
                    </TableBody>
                  </Table>
                </div>
              </CardContent>
            </Card>
          )}
        </>
      )}
    </div>
  );
}

export default function RealtimePage() {
  return (
    <Suspense
      fallback={
        <div className="space-y-4">
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-48 w-full rounded-xl" />
        </div>
      }
    >
      <RealtimeContent />
    </Suspense>
  );
}
