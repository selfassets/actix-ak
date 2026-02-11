"use client";

import { useState, useEffect, Suspense } from "react";
import { useSearchParams, useRouter } from "next/navigation";
import { RefreshCw } from "lucide-react";
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
import {
  getFuturesInfo,
  getFuturesBatch,
  getFuturesRealtime,
  FuturesInfo,
} from "@/lib/api";

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
  const changeColor = isUp
    ? "text-red-500 dark:text-red-400"
    : "text-green-500 dark:text-green-400";
  const bgClass = isUp
    ? "bg-red-50/50 dark:bg-red-950/20 border-red-200 dark:border-red-900/50"
    : "bg-green-50/50 dark:bg-green-950/20 border-green-200 dark:border-green-900/50";

  return (
    <Card className={`transition-all hover:shadow-md ${bgClass}`}>
      <CardHeader className="pb-2 pt-4 px-4">
        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-2">
              <CardTitle className="text-xl font-bold font-mono tracking-tight text-foreground">
                {data.symbol}
              </CardTitle>
              <Badge
                variant={isUp ? "outline" : "outline"}
                className={`${changeColor} border-current bg-transparent`}
              >
                {data.name}
              </Badge>
            </div>
            <p className="text-xs text-muted-foreground mt-1 font-mono">
              {data.updated_at.split(" ")[1]} 更新
            </p>
          </div>
          <div className={`text-right ${changeColor}`}>
            <div className="text-3xl font-bold font-mono tracking-tighter">
              {data.current_price?.toFixed(2)}
            </div>
            <div className="text-sm font-medium flex items-center justify-end gap-1">
              <span>{isUp ? "▲" : "▼"}</span>
              <span className="font-mono">
                {Math.abs(data.change).toFixed(2)}
              </span>
              <span className="font-mono">
                ({data.change_percent?.toFixed(2)}%)
              </span>
            </div>
          </div>
        </div>
      </CardHeader>
      <CardContent className="px-4 pb-4 pt-2">
        <div className="grid grid-cols-2 gap-x-4 gap-y-2 text-xs">
          <div className="flex justify-between items-center py-1 border-b border-border/50">
            <span className="text-muted-foreground">开盘</span>
            <span
              className={`font-mono font-medium ${data.open > data.prev_close ? "text-red-500" : "text-green-500"}`}
            >
              {data.open?.toFixed(2)}
            </span>
          </div>
          <div className="flex justify-between items-center py-1 border-b border-border/50">
            <span className="text-muted-foreground">成交量</span>
            <span className="font-mono font-medium text-foreground">
              {data.volume?.toLocaleString()}
            </span>
          </div>
          <div className="flex justify-between items-center py-1 border-b border-border/50">
            <span className="text-muted-foreground">最高</span>
            <span className="font-mono font-medium text-red-500">
              {data.high?.toFixed(2)}
            </span>
          </div>
          <div className="flex justify-between items-center py-1 border-b border-border/50">
            <span className="text-muted-foreground">持仓量</span>
            {/* API might not return open_interest always, handling safely if needed, though current type doesn't show it explicitly, assuming amount or similar for now or just keeping volume context */}
            <span className="font-mono font-medium text-foreground">-</span>
          </div>
          <div className="flex justify-between items-center py-1 border-b border-border/50">
            <span className="text-muted-foreground">最低</span>
            <span className="font-mono font-medium text-green-500">
              {data.low?.toFixed(2)}
            </span>
          </div>
          <div className="flex justify-between items-center py-1 border-b border-border/50">
            <span className="text-muted-foreground">昨收</span>
            <span className="font-mono font-medium text-foreground">
              {data.prev_close?.toFixed(2)}
            </span>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

function RealtimeContent() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const urlSymbol = searchParams.get("symbol") || "";

  const [symbol, setSymbol] = useState(urlSymbol);
  const [results, setResults] = useState<FuturesInfo[]>([]);
  const [loading, setLoading] = useState(!!urlSymbol);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (urlSymbol) {
      setSymbol(urlSymbol);
      setLoading(true);
      // 尝试用品种简码查询所有合约，若失败则用精确合约代码查询
      getFuturesRealtime(urlSymbol)
        .then((res) => {
          if (res.success && res.data && res.data.length > 0) {
            setResults(res.data);
          } else {
            // 回退到精确合约查询
            return getFuturesInfo(urlSymbol).then((r) => {
              if (r.success && r.data) setResults([r.data]);
              else setError(r.message);
            });
          }
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
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          {urlSymbol && (
            <Button
              variant="ghost"
              size="sm"
              onClick={() => router.back()}
              className="gap-1"
            >
              ← 返回
            </Button>
          )}
          <div>
            <h1 className="text-3xl font-bold tracking-tight bg-gradient-to-r from-primary to-primary/60 bg-clip-text text-transparent">
              实时行情
            </h1>
            <p className="text-muted-foreground text-sm mt-1">
              查询期货合约的实时行情数据
            </p>
          </div>
        </div>
      </div>

      {/* Search */}
      <Card className="overflow-hidden border-border/50 bg-background/50 backdrop-blur-sm">
        <CardContent className="py-6">
          <div className="flex gap-3">
            <Input
              placeholder="输入合约代码，如 CU2602、RB2605..."
              value={symbol}
              onChange={(e) => setSymbol(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSearch()}
              className="font-mono max-w-md"
            />
            <Button onClick={handleSearch} disabled={loading} className="gap-2">
              查询
            </Button>
            <Button
              variant="outline"
              onClick={handleBatchQuery}
              disabled={loading}
              className="hidden sm:flex"
            >
              批量看板
            </Button>
            <Button
              variant="ghost"
              size="icon"
              onClick={handleSearch}
              title="刷新"
              disabled={loading}
            >
              <RefreshCw
                className={`h-4 w-4 ${loading ? "animate-spin" : ""}`}
              />
            </Button>
          </div>
          <div className="flex gap-2 mt-4 flex-wrap items-center">
            <span className="text-xs text-muted-foreground mr-1">
              热门合约:
            </span>
            {quickSymbols.map((s) => (
              <Badge
                key={s}
                variant="outline"
                className="cursor-pointer hover:bg-primary/10 hover:text-primary hover:border-primary transition-all font-mono py-1 px-2"
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
              <div key={r.symbol} className="space-y-2">
                <PriceCard data={r} />
                <Button
                  variant="outline"
                  size="sm"
                  className="w-full gap-1.5"
                  onClick={() =>
                    router.push(`/futures/detail?symbol=${r.symbol}`)
                  }
                >
                  📈 查看详情
                </Button>
              </div>
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
                              className={`text-right font-mono ${isUp ? "text-red-400" : "text-green-400"}`}
                            >
                              {r.current_price?.toFixed(2)}
                            </TableCell>
                            <TableCell
                              className={`text-right font-mono ${isUp ? "text-red-400" : "text-green-400"}`}
                            >
                              {isUp ? "+" : ""}
                              {r.change?.toFixed(2)}
                            </TableCell>
                            <TableCell
                              className={`text-right font-mono ${isUp ? "text-red-400" : "text-green-400"}`}
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
