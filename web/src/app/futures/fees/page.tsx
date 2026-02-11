"use client";

import { useEffect, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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
import { getFuturesFees, getCommInfo } from "@/lib/api";

const commExchanges = [
  { code: "", label: "全部" },
  { code: "上期所", label: "上期所" },
  { code: "大商所", label: "大商所" },
  { code: "郑商所", label: "郑商所" },
  { code: "中金所", label: "中金所" },
  { code: "能源中心", label: "能源中心" },
  { code: "广期所", label: "广期所" },
];

export default function FeesPage() {
  const [fees, setFees] = useState<Record<string, unknown>[]>([]);
  const [commInfo, setCommInfo] = useState<Record<string, unknown>[]>([]);
  const [feesLoading, setFeesLoading] = useState(true);
  const [commLoading, setCommLoading] = useState(true);
  const [feesError, setFeesError] = useState<string | null>(null);
  const [commError, setCommError] = useState<string | null>(null);
  const [activeExchange, setActiveExchange] = useState("");

  const fetchCommInfo = async (exchange: string) => {
    setCommLoading(true);
    setCommError(null);
    try {
      const res = await getCommInfo(exchange || undefined);
      if (res.success && res.data) setCommInfo(res.data);
      else setCommError(res.message);
    } catch (e) {
      setCommError(e instanceof Error ? e.message : "请求失败");
    }
    setCommLoading(false);
  };

  useEffect(() => {
    getFuturesFees()
      .then((res) => {
        if (res.success && res.data) setFees(res.data);
        else setFeesError(res.message);
      })
      .catch((e) => setFeesError(e.message))
      .finally(() => setFeesLoading(false));

    fetchCommInfo("");
  }, []);

  const handleExchangeChange = (code: string) => {
    setActiveExchange(code);
    fetchCommInfo(code);
  };

  const columnNameMap: Record<string, string> = {
    // FuturesFeesInfo
    exchange: "交易所",
    contract_code: "合约代码",
    contract_name: "合约名称",
    product_code: "品种代码",
    product_name: "品种名称",
    contract_size: "合约乘数",
    price_tick: "最小跳动",
    open_fee_rate: "开仓费率",
    open_fee: "开仓费用/手",
    close_fee_rate: "平仓费率",
    close_fee: "平仓费用/手",
    close_today_fee_rate: "平今费率",
    close_today_fee: "平今费用/手",
    long_margin_rate: "做多保证金率",
    short_margin_rate: "做空保证金率",
    updated_at: "更新时间",
    // FuturesCommInfo
    current_price: "现价",
    limit_up: "涨停板",
    limit_down: "跌停板",
    margin_buy: "保证金-买开(%)",
    margin_sell: "保证金-卖开(%)",
    margin_per_lot: "保证金-每手(元)",
    fee_open_ratio: "开仓-万分之",
    fee_open_yuan: "开仓-元",
    fee_close_yesterday_ratio: "平昨-万分之",
    fee_close_yesterday_yuan: "平昨-元",
    fee_close_today_ratio: "平今-万分之",
    fee_close_today_yuan: "平今-元",
    profit_per_tick: "每跳毛利",
    fee_total: "手续费(开+平)",
    net_profit_per_tick: "每跳净利",
    remark: "备注",
  };

  const renderTable = (
    data: Record<string, unknown>[],
    loading: boolean,
    error: string | null,
  ) => {
    if (loading) {
      return (
        <div className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => (
            <Skeleton key={i} className="h-10 w-full" />
          ))}
        </div>
      );
    }
    if (error) {
      return <p className="text-center text-destructive py-8">{error}</p>;
    }
    if (data.length === 0) {
      return <p className="text-center text-muted-foreground py-8">暂无数据</p>;
    }

    const columns = Object.keys(data[0]);
    return (
      <div className="rounded-md border overflow-auto max-h-[500px]">
        <Table>
          <TableHeader>
            <TableRow>
              {columns.map((col) => (
                <TableHead key={col} className="whitespace-nowrap">
                  {columnNameMap[col] || col}
                </TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {data.map((row, i) => (
              <TableRow key={i}>
                {columns.map((col) => (
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
    );
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">交易费用</h1>
        <p className="text-muted-foreground text-sm mt-1">
          期货品种手续费及交易规则参照
        </p>
      </div>

      {/* 费用参照表 */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span>💰</span> 交易费用参照表
          </CardTitle>
        </CardHeader>
        <CardContent>{renderTable(fees, feesLoading, feesError)}</CardContent>
      </Card>

      {/* 手续费信息 */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg flex items-center gap-2">
            <span>📋</span> 手续费明细
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <Tabs value={activeExchange} onValueChange={handleExchangeChange}>
            <TabsList className="flex-wrap h-auto gap-1">
              {commExchanges.map((ex) => (
                <TabsTrigger key={ex.code} value={ex.code}>
                  {ex.label}
                </TabsTrigger>
              ))}
            </TabsList>
            {commExchanges.map((ex) => (
              <TabsContent key={ex.code} value={ex.code}>
                {renderTable(commInfo, commLoading, commError)}
              </TabsContent>
            ))}
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}
