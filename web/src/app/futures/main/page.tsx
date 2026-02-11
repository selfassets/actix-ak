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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  getMainDisplay,
  getMainByExchange,
  FuturesMainContract,
} from "@/lib/api";

const exchanges = [
  { code: "all", label: "全部" },
  { code: "SHFE", label: "上期所" },
  { code: "DCE", label: "大商所" },
  { code: "CZCE", label: "郑商所" },
  { code: "CFFEX", label: "中金所" },
  { code: "GFEX", label: "广期所" },
];

export default function MainContractsPage() {
  const [contracts, setContracts] = useState<FuturesMainContract[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState("all");

  const fetchData = async (exchange: string) => {
    setLoading(true);
    setError(null);
    try {
      const res =
        exchange === "all"
          ? await getMainDisplay()
          : await getMainByExchange(exchange);
      if (res.success && res.data) {
        setContracts(res.data);
      } else {
        setError(res.message);
        setContracts([]);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "请求失败");
      setContracts([]);
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchData("all");
  }, []);

  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
    fetchData(tab);
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">主力合约</h1>
        <p className="text-muted-foreground text-sm mt-1">
          主力连续合约一览表，实时展示各品种主力合约行情
        </p>
      </div>

      <Tabs value={activeTab} onValueChange={handleTabChange}>
        <TabsList className="flex-wrap h-auto gap-1">
          {exchanges.map((ex) => (
            <TabsTrigger key={ex.code} value={ex.code}>
              {ex.label}
            </TabsTrigger>
          ))}
        </TabsList>

        {exchanges.map((ex) => (
          <TabsContent key={ex.code} value={ex.code}>
            {loading ? (
              <Card>
                <CardContent className="py-6 space-y-3">
                  {Array.from({ length: 8 }).map((_, i) => (
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
            ) : contracts.length === 0 ? (
              <Card>
                <CardContent className="py-12 text-center text-muted-foreground">
                  暂无数据
                </CardContent>
              </Card>
            ) : (
              <Card>
                <CardHeader>
                  <CardTitle className="text-lg flex items-center gap-2">
                    <span>🎯</span>
                    {ex.label}主力合约 ({contracts.length} 个)
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="rounded-md border overflow-auto max-h-[600px]">
                    <Table>
                      <TableHeader>
                        <TableRow>
                          <TableHead>合约代码</TableHead>
                          <TableHead>品种名称</TableHead>
                          <TableHead className="text-right">当前价格</TableHead>
                          <TableHead className="text-right">涨跌幅</TableHead>
                        </TableRow>
                      </TableHeader>
                      <TableBody>
                        {contracts.map((c, i) => {
                          const isUp = c.change_percent >= 0;
                          return (
                            <TableRow key={i}>
                              <TableCell className="font-mono font-medium">
                                {c.symbol}
                              </TableCell>
                              <TableCell>{c.name}</TableCell>
                              <TableCell
                                className={`text-right font-mono ${isUp ? "text-green-400" : "text-red-400"}`}
                              >
                                {c.current_price?.toFixed(2)}
                              </TableCell>
                              <TableCell className="text-right">
                                <Badge
                                  variant={isUp ? "default" : "destructive"}
                                  className="font-mono"
                                >
                                  {isUp ? "+" : ""}
                                  {c.change_percent?.toFixed(2)}%
                                </Badge>
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
          </TabsContent>
        ))}
      </Tabs>
    </div>
  );
}
