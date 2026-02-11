"use client";

import { Suspense } from "react";
import { useSearchParams, useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import KlineChart from "@/components/kline-chart";
import { Skeleton } from "@/components/ui/skeleton";

function KlinePageContent() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const symbol = searchParams.get("symbol");
  const name = searchParams.get("name") || "";

  if (!symbol) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[50vh] space-y-4">
        <p className="text-muted-foreground">未指定合约代码</p>
        <Button onClick={() => router.back()}>返回</Button>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => router.back()}
          className="gap-1"
        >
          ← 返回
        </Button>
        <div>
          <h1 className="text-2xl font-bold tracking-tight">
            {name ? `${name} (${symbol})` : symbol} K线图
          </h1>
          <p className="text-muted-foreground text-sm mt-1">
            查看期货合约历史行情数据
          </p>
        </div>
      </div>

      <KlineChart symbol={symbol} name={name} />
    </div>
  );
}

export default function KlinePage() {
  return (
    <Suspense
      fallback={
        <div className="space-y-4">
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-[400px] w-full rounded-xl" />
        </div>
      }
    >
      <KlinePageContent />
    </Suspense>
  );
}
