import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { FuturesContractDetail } from "@/lib/api";

export function FuturesContractInfo({
  detail,
}: {
  detail: FuturesContractDetail;
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-lg">合约详情</CardTitle>
      </CardHeader>
      <CardContent>
        <dl className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-4 text-sm">
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">交易单位</dt>
            <dd className="font-medium">{detail.trading_unit}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">报价单位</dt>
            <dd className="font-medium">{detail.quote_unit}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">最小变动价位</dt>
            <dd className="font-medium">{detail.min_price_change}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">涨跌停板幅度</dt>
            <dd className="font-medium">{detail.price_limit}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">合约月份</dt>
            <dd className="font-medium">{detail.contract_months}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">最后交易日</dt>
            <dd className="font-medium">{detail.last_trading_day}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">最后交割日</dt>
            <dd className="font-medium">{detail.last_delivery_day}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">交割品级</dt>
            <dd className="font-medium">{detail.delivery_grade}</dd>
          </div>
          <div className="flex flex-col gap-1">
            <dt className="text-muted-foreground">最低交易保证金</dt>
            <dd className="font-medium">{detail.margin}</dd>
          </div>
        </dl>
      </CardContent>
    </Card>
  );
}
