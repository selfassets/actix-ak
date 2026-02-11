"use client";

import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
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
  getExchanges,
  getSymbols,
  FuturesExchange,
  FuturesSymbolMark,
} from "@/lib/api";

const exchangeColors: Record<string, string> = {
  SHFE: "from-blue-500/20 to-blue-600/20 border-blue-500/30",
  DCE: "from-green-500/20 to-green-600/20 border-green-500/30",
  CZCE: "from-purple-500/20 to-purple-600/20 border-purple-500/30",
  CFFEX: "from-red-500/20 to-red-600/20 border-red-500/30",
  INE: "from-amber-500/20 to-amber-600/20 border-amber-500/30",
  GFEX: "from-teal-500/20 to-teal-600/20 border-teal-500/30",
};

export default function ExchangesPage() {
  const [exchanges, setExchanges] = useState<FuturesExchange[]>([]);
  const [symbols, setSymbols] = useState<FuturesSymbolMark[]>([]);
  const [selectedExchange, setSelectedExchange] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [symbolsLoading, setSymbolsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getExchanges()
      .then((res) => {
        if (res.success && res.data) setExchanges(res.data);
        else setError(res.message);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, []);

  const handleExchangeClick = async (code: string) => {
    setSelectedExchange(code);
    setSymbolsLoading(true);
    try {
      const res = await getSymbols(code);
      if (res.success && res.data) setSymbols(res.data);
      else setSymbols([]);
    } catch {
      setSymbols([]);
    }
    setSymbolsLoading(false);
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">交易所 & 品种</h1>
        <p className="text-muted-foreground text-sm mt-1">
          查看国内期货交易所列表及其品种映射信息
        </p>
      </div>

      {/* Exchanges Grid */}
      {loading ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <Skeleton key={i} className="h-28 rounded-xl" />
          ))}
        </div>
      ) : error ? (
        <Card className="border-destructive">
          <CardContent className="py-6 text-center text-destructive">
            {error}
          </CardContent>
        </Card>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          {exchanges.map((ex) => (
            <Card
              key={ex.code}
              onClick={() => handleExchangeClick(ex.code)}
              className={`cursor-pointer transition-all duration-200 hover:scale-[1.02] hover:shadow-lg border bg-gradient-to-br ${
                exchangeColors[ex.code] ||
                "from-gray-500/20 to-gray-600/20 border-gray-500/30"
              } ${selectedExchange === ex.code ? "ring-2 ring-primary" : ""}`}
            >
              <CardHeader className="pb-2">
                <div className="flex items-center justify-between">
                  <CardTitle className="text-base">{ex.name}</CardTitle>
                  <Badge variant="outline">{ex.code}</Badge>
                </div>
              </CardHeader>
              <CardContent>
                <p className="text-xs text-muted-foreground">{ex.name_en}</p>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      {/* Symbols Table */}
      {selectedExchange && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg flex items-center gap-2">
              <span>📋</span>
              {selectedExchange} 品种列表
            </CardTitle>
          </CardHeader>
          <CardContent>
            {symbolsLoading ? (
              <div className="space-y-2">
                {Array.from({ length: 5 }).map((_, i) => (
                  <Skeleton key={i} className="h-10 w-full" />
                ))}
              </div>
            ) : symbols.length === 0 ? (
              <p className="text-center text-muted-foreground py-8">
                暂无品种数据
              </p>
            ) : (
              <div className="rounded-md border overflow-auto max-h-[500px]">
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
                    {symbols.map((s, i) => (
                      <TableRow key={i}>
                        <TableCell className="font-mono font-medium">
                          {s.symbol}
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
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
