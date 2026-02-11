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
import { getSymbols, FuturesSymbolMark } from "@/lib/api";

export default function SymbolsPage() {
  const [symbols, setSymbols] = useState<FuturesSymbolMark[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");

  useEffect(() => {
    getSymbols()
      .then((res) => {
        if (res.success && res.data) setSymbols(res.data);
        else setError(res.message);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, []);

  const filtered = symbols.filter(
    (s) =>
      s.symbol.toLowerCase().includes(search.toLowerCase()) ||
      s.name.toLowerCase().includes(search.toLowerCase()) ||
      s.exchange.toLowerCase().includes(search.toLowerCase()),
  );

  const exchangeCount = new Map<string, number>();
  symbols.forEach((s) =>
    exchangeCount.set(s.exchange, (exchangeCount.get(s.exchange) || 0) + 1),
  );

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">品种列表</h1>
        <p className="text-muted-foreground text-sm mt-1">
          全部期货品种映射信息一览
        </p>
      </div>

      {/* Stats */}
      {!loading && !error && (
        <div className="flex flex-wrap gap-2">
          <Badge variant="outline" className="py-1.5 px-3 text-sm">
            共 {symbols.length} 个品种
          </Badge>
          {Array.from(exchangeCount.entries()).map(([ex, count]) => (
            <Badge
              key={ex}
              variant="secondary"
              className="py-1.5 px-3 text-sm gap-1.5"
            >
              {ex} <span className="font-mono">{count}</span>
            </Badge>
          ))}
        </div>
      )}

      {/* Search */}
      <div className="max-w-sm">
        <Input
          placeholder="搜索代码、名称或交易所..."
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
              <span>📋</span> 品种列表 ({filtered.length} 个)
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border overflow-auto max-h-[600px]">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>品种代码</TableHead>
                    <TableHead>品种名称</TableHead>
                    <TableHead>交易所</TableHead>
                    <TableHead>Node</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {filtered.map((s, i) => (
                    <TableRow
                      key={i}
                      className="cursor-pointer hover:bg-muted/50"
                    >
                      <TableCell className="font-mono font-medium">
                        <Link
                          href={`/futures/realtime?symbol=${s.symbol}`}
                          className="text-primary hover:underline"
                        >
                          {s.symbol}
                        </Link>
                      </TableCell>
                      <TableCell>{s.name}</TableCell>
                      <TableCell>
                        <Badge variant="secondary">{s.exchange}</Badge>
                      </TableCell>
                      <TableCell className="font-mono text-xs text-muted-foreground">
                        {s.node}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
