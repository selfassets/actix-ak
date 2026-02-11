"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
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
import { getStockList, StockInfo } from "@/lib/api";

export default function StocksPage() {
  const [stocks, setStocks] = useState<StockInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");

  useEffect(() => {
    getStockList(100)
      .then((res) => {
        if (res.success && res.data) setStocks(res.data);
        else setError(res.message);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, []);

  const filtered = stocks.filter(
    (s) =>
      s.symbol.toLowerCase().includes(search.toLowerCase()) ||
      s.name.toLowerCase().includes(search.toLowerCase()),
  );

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">股票列表</h1>
        <p className="text-muted-foreground text-sm mt-1">A 股实时行情数据</p>
      </div>

      {/* Search */}
      <div className="max-w-sm">
        <Input
          placeholder="搜索代码或名称..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>

      {loading ? (
        <Card>
          <CardContent className="py-6 space-y-3">
            {Array.from({ length: 10 }).map((_, i) => (
              <Skeleton key={i} className="h-10 w-full" />
            ))}
          </CardContent>
        </Card>
      ) : error ? (
        <Card className="border-destructive">
          <CardContent className="py-6 text-center text-destructive">
            {error}
          </CardContent>
        </Card>
      ) : (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg flex items-center gap-2">
              <span>📋</span> 股票行情 ({filtered.length} 只)
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border overflow-auto max-h-[600px]">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>代码</TableHead>
                    <TableHead>名称</TableHead>
                    <TableHead className="text-right">当前价</TableHead>
                    <TableHead className="text-right">涨跌额</TableHead>
                    <TableHead className="text-right">涨跌幅</TableHead>
                    <TableHead className="text-right">成交量</TableHead>
                    <TableHead className="text-right">成交额</TableHead>
                    <TableHead className="text-right">最高</TableHead>
                    <TableHead className="text-right">最低</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {filtered.map((s) => {
                    const isUp = s.change >= 0;
                    return (
                      <TableRow
                        key={s.symbol}
                        className="cursor-pointer hover:bg-muted/50"
                      >
                        <TableCell>
                          <Link
                            href={`/stocks/detail?symbol=${s.symbol}`}
                            className="font-mono font-medium text-primary hover:underline"
                          >
                            {s.symbol}
                          </Link>
                        </TableCell>
                        <TableCell className="font-medium">{s.name}</TableCell>
                        <TableCell
                          className={`text-right font-mono ${isUp ? "text-red-400" : "text-green-400"}`}
                        >
                          {s.current_price?.toFixed(2)}
                        </TableCell>
                        <TableCell
                          className={`text-right font-mono ${isUp ? "text-red-400" : "text-green-400"}`}
                        >
                          {isUp ? "+" : ""}
                          {s.change?.toFixed(2)}
                        </TableCell>
                        <TableCell className="text-right">
                          <Badge
                            variant={isUp ? "default" : "destructive"}
                            className="font-mono text-xs"
                          >
                            {isUp ? "+" : ""}
                            {s.change_percent?.toFixed(2)}%
                          </Badge>
                        </TableCell>
                        <TableCell className="text-right font-mono text-xs">
                          {s.volume?.toLocaleString()}
                        </TableCell>
                        <TableCell className="text-right font-mono text-xs">
                          {s.amount?.toLocaleString()}
                        </TableCell>
                        <TableCell className="text-right font-mono text-xs">
                          {s.high?.toFixed(2)}
                        </TableCell>
                        <TableCell className="text-right font-mono text-xs">
                          {s.low?.toFixed(2)}
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
    </div>
  );
}
