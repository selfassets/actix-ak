import { Badge } from "@/components/ui/badge";
import { FuturesInfo } from "@/lib/api";

export function FuturesDetailHeader({ data }: { data: FuturesInfo }) {
  const isUp = data.change >= 0;
  const changeColor = isUp
    ? "text-red-500 dark:text-red-400"
    : "text-green-500 dark:text-green-400";

  return (
    <div className="bg-card border-b px-6 py-6 shadow-sm">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <div className="flex items-center gap-3">
            <h1 className="text-3xl font-bold tracking-tight text-foreground font-mono">
              {data.symbol}
            </h1>
            <Badge variant="outline" className="text-base py-1 px-3">
              {data.name}
            </Badge>
          </div>
          <p className="text-sm text-muted-foreground mt-2 font-mono">
            更新时间: {data.updated_at}
          </p>
        </div>

        <div className="flex items-end gap-6">
          <div>
            <div
              className={`text-4xl font-bold font-mono tracking-tighter ${changeColor}`}
            >
              {data.current_price?.toFixed(2)}
            </div>
            <div
              className={`text-lg font-medium flex items-center justify-end gap-2 mt-1 ${changeColor}`}
            >
              <span>
                {isUp ? "+" : ""}
                {data.change?.toFixed(2)}
              </span>
              <span>
                ({isUp ? "+" : ""}
                {data.change_percent?.toFixed(2)}%)
              </span>
            </div>
          </div>

          <div className="grid grid-cols-2 gap-x-8 gap-y-2 text-sm border-l pl-6 border-border/50">
            <div className="flex justify-between w-32">
              <span className="text-muted-foreground">开盘</span>
              <span
                className={`font-mono font-medium ${data.open > data.prev_close ? "text-red-500" : "text-green-500"}`}
              >
                {data.open?.toFixed(2)}
              </span>
            </div>
            <div className="flex justify-between w-32">
              <span className="text-muted-foreground">最高</span>
              <span className="font-mono font-medium text-red-500">
                {data.high?.toFixed(2)}
              </span>
            </div>
            <div className="flex justify-between w-32">
              <span className="text-muted-foreground">成交量</span>
              <span className="font-mono font-medium">
                {data.volume?.toLocaleString()}
              </span>
            </div>
            <div className="flex justify-between w-32">
              <span className="text-muted-foreground">最低</span>
              <span className="font-mono font-medium text-green-500">
                {data.low?.toFixed(2)}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
