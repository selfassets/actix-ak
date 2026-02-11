"use client";

import { useEffect, useState } from "react";
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
  getForeignSymbols,
  getForeignRealtime,
  ForeignFuturesSymbol,
} from "@/lib/api";

const categories = [
  { name: "贵金属", symbols: ["GC", "SI", "XAU", "XAG"], icon: "💎" },
  { name: "原油", symbols: ["CL", "OIL"], icon: "🛢️" },
  { name: "LME金属", symbols: ["CAD", "AHD", "ZSD", "NID"], icon: "🔩" },
  { name: "农产品", symbols: ["S", "C", "W", "BO", "SM"], icon: "🌾" },
];

export default function ForeignPage() {
  const [allSymbols, setAllSymbols] = useState<ForeignFuturesSymbol[]>([]);
  const [realtimeData, setRealtimeData] = useState<Record<string, unknown>[]>(
    [],
  );
  const [loading, setLoading] = useState(true);
  const [rtLoading, setRtLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [activeCategory, setActiveCategory] = useState<string | null>(null);

  useEffect(() => {
    getForeignSymbols()
      .then((res) => {
        if (res.success && res.data) setAllSymbols(res.data);
        else setError(res.message);
      })
      .catch((e) => setError(e.message))
      .finally(() => setLoading(false));
  }, []);

  const queryCategory = async (cat: (typeof categories)[0]) => {
    setActiveCategory(cat.name);
    setRtLoading(true);
    try {
      const res = await getForeignRealtime(cat.symbols);
      if (res.success && res.data) setRealtimeData(res.data);
      else setRealtimeData([]);
    } catch {
      setRealtimeData([]);
    }
    setRtLoading(false);
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">外盘期货</h1>
        <p className="text-muted-foreground text-sm mt-1">
          国际市场期货品种实时行情数据
        </p>
      </div>

      {/* Category Quick Buttons */}
      <div className="flex gap-3 flex-wrap">
        {categories.map((cat) => (
          <Button
            key={cat.name}
            variant={activeCategory === cat.name ? "default" : "outline"}
            onClick={() => queryCategory(cat)}
            disabled={rtLoading}
            className="gap-2"
          >
            <span>{cat.icon}</span>
            {cat.name}
          </Button>
        ))}
      </div>

      {/* Realtime Data */}
      {rtLoading && (
        <Card>
          <CardContent className="py-6 space-y-3">
            {Array.from({ length: 4 }).map((_, i) => (
              <Skeleton key={i} className="h-10 w-full" />
            ))}
          </CardContent>
        </Card>
      )}

      {!rtLoading && realtimeData.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-lg flex items-center gap-2">
              <span>⚡</span> {activeCategory} 实时行情
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border overflow-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    {Object.keys(realtimeData[0]).map((col) => (
                      <TableHead key={col} className="whitespace-nowrap">
                        {col}
                      </TableHead>
                    ))}
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {realtimeData.map((row, i) => (
                    <TableRow key={i}>
                      {Object.keys(row).map((col) => (
                        <TableCell
                          key={col}
                          className="font-mono text-xs whitespace-nowrap"
                        >
                          {String(row[col] ?? "-")}
                        </TableCell>
                      ))}
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          </CardContent>
        </Card>
      )}

      {/* All Symbols */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span>🌐</span> 外盘品种列表
          </CardTitle>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="space-y-2">
              {Array.from({ length: 5 }).map((_, i) => (
                <Skeleton key={i} className="h-10 w-full" />
              ))}
            </div>
          ) : error ? (
            <p className="text-center text-destructive py-8">{error}</p>
          ) : allSymbols.length === 0 ? (
            <p className="text-center text-muted-foreground py-8">暂无数据</p>
          ) : (
            <div className="flex flex-wrap gap-2">
              {allSymbols.map((s, i) => (
                <Badge
                  key={i}
                  variant="outline"
                  className="font-mono gap-1.5 py-1.5 px-3"
                >
                  <span className="font-bold">{s.symbol}</span>
                  <span className="text-muted-foreground text-xs">
                    {s.name}
                  </span>
                </Badge>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
