"use client";

import { useEffect, useRef, useState } from "react";
import {
  createChart,
  CandlestickSeries,
  HistogramSeries,
  ColorType,
  CrosshairMode,
  type IChartApi,
  type CandlestickData,
  type HistogramData,
  type Time,
} from "lightweight-charts";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { getFuturesHistory, FuturesHistoryData } from "@/lib/api";

interface KlineChartProps {
  symbol: string;
  name?: string;
}

export default function KlineChart({ symbol, name }: KlineChartProps) {
  const chartContainerRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<IChartApi | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [limit, setLimit] = useState(60);

  useEffect(() => {
    if (!chartContainerRef.current || !symbol) return;

    // Cleanup previous chart
    if (chartRef.current) {
      chartRef.current.remove();
      chartRef.current = null;
    }

    setLoading(true);
    setError(null);

    getFuturesHistory(symbol, limit)
      .then((res) => {
        if (!res.success || !res.data || res.data.length === 0) {
          setError(res.message || "暂无K线数据");
          setLoading(false);
          return;
        }

        if (!chartContainerRef.current) return;

        const chart = createChart(chartContainerRef.current, {
          width: chartContainerRef.current.clientWidth,
          height: 400,
          layout: {
            background: { type: ColorType.Solid, color: "transparent" },
            textColor: "#9ca3af",
            fontSize: 12,
          },
          grid: {
            vertLines: { color: "rgba(255,255,255,0.04)" },
            horzLines: { color: "rgba(255,255,255,0.04)" },
          },
          crosshair: {
            mode: CrosshairMode.Normal,
          },
          rightPriceScale: {
            borderColor: "rgba(255,255,255,0.1)",
          },
          timeScale: {
            borderColor: "rgba(255,255,255,0.1)",
            timeVisible: false,
          },
        });

        chartRef.current = chart;

        // Candlestick
        const candlestickSeries = chart.addSeries(CandlestickSeries, {
          upColor: "#ef4444",
          downColor: "#22c55e",
          borderUpColor: "#ef4444",
          borderDownColor: "#22c55e",
          wickUpColor: "#ef4444",
          wickDownColor: "#22c55e",
        });

        // Volume
        const volumeSeries = chart.addSeries(HistogramSeries, {
          priceFormat: { type: "volume" },
          priceScaleId: "volume",
        });

        chart.priceScale("volume").applyOptions({
          scaleMargins: { top: 0.8, bottom: 0 },
        });

        const candleData: CandlestickData<Time>[] = [];
        const volumeData: HistogramData<Time>[] = [];

        // Sort by date ascending
        const sorted = [...res.data].sort(
          (a: FuturesHistoryData, b: FuturesHistoryData) =>
            a.date.localeCompare(b.date),
        );

        sorted.forEach((d: FuturesHistoryData) => {
          const time = d.date as Time;
          const isUp = d.close >= d.open;

          candleData.push({
            time,
            open: d.open,
            high: d.high,
            low: d.low,
            close: d.close,
          });

          volumeData.push({
            time,
            value: d.volume,
            color: isUp ? "rgba(239, 68, 68, 0.4)" : "rgba(34, 197, 94, 0.4)",
          });
        });

        candlestickSeries.setData(candleData);
        volumeSeries.setData(volumeData);
        chart.timeScale().fitContent();

        // Resize handler
        const handleResize = () => {
          if (chartContainerRef.current && chartRef.current) {
            chartRef.current.applyOptions({
              width: chartContainerRef.current.clientWidth,
            });
          }
        };

        const resizeObserver = new ResizeObserver(() => handleResize());
        resizeObserver.observe(chartContainerRef.current);
        setLoading(false);

        return () => {
          resizeObserver.disconnect();
        };
      })
      .catch((e) => {
        setError(e instanceof Error ? e.message : "K线数据加载失败");
        setLoading(false);
      });

    return () => {
      if (chartRef.current) {
        chartRef.current.remove();
        chartRef.current = null;
      }
    };
  }, [symbol, limit]);

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between flex-wrap gap-2">
          <CardTitle className="text-lg flex items-center gap-2">
            <span>📈</span>
            {name || symbol} 日K线
          </CardTitle>
          <div className="flex gap-1">
            {[30, 60, 120, 250].map((n) => (
              <Button
                key={n}
                size="sm"
                variant={limit === n ? "default" : "outline"}
                onClick={() => setLimit(n)}
                className="text-xs h-7 px-2"
              >
                {n}日
              </Button>
            ))}
          </div>
        </div>
      </CardHeader>
      <CardContent className="relative h-[400px]">
        {loading && (
          <Skeleton className="absolute inset-0 w-full h-full rounded-lg z-10" />
        )}
        {error && (
          <div className="absolute inset-0 flex items-center justify-center text-muted-foreground z-10 bg-background/80 backdrop-blur-sm">
            {error}
          </div>
        )}
        <div ref={chartContainerRef} className="w-full h-full" />
      </CardContent>
    </Card>
  );
}
