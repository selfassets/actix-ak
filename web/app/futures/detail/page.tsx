"use client";

import { useEffect, useState, Suspense } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import {
  getFuturesInfo,
  getContractDetail,
  FuturesInfo,
  FuturesContractDetail,
} from "@/lib/api";
import { FuturesDetailHeader } from "@/components/futures-detail-header";
import { FuturesContractInfo } from "@/components/futures-contract-info";
import KlineChart from "@/components/kline-chart";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { AlertCircle } from "lucide-react";

function DetailContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const symbol = searchParams.get("symbol");

  const [info, setInfo] = useState<FuturesInfo | null>(null);
  const [detail, setDetail] = useState<FuturesContractDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!symbol) return;

    const fetchData = async () => {
      setLoading(true);
      setError(null);
      try {
        const [infoRes, detailRes] = await Promise.all([
          getFuturesInfo(symbol),
          getContractDetail(symbol),
        ]);

        if (infoRes.success && infoRes.data) {
          setInfo(infoRes.data);
        } else {
          throw new Error(infoRes.message || "Failed to fetch futures info");
        }

        if (detailRes.success && detailRes.data) {
          setDetail(detailRes.data);
        } else {
          console.warn("Detail fetch failed:", detailRes.message);
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : "An error occurred");
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, [symbol]);

  if (!symbol) {
    return (
      <div className="container mx-auto p-6 flex flex-col items-center justify-center min-h-[50vh] space-y-4">
        <div className="text-muted-foreground">No symbol specified</div>
        <Button onClick={() => router.back()}>Go Back</Button>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="space-y-6 container mx-auto p-6">
        <Skeleton className="h-32 w-full" />
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <Skeleton className="h-[500px] lg:col-span-2" />
          <Skeleton className="h-[500px]" />
        </div>
      </div>
    );
  }

  if (error || !info) {
    return (
      <div className="container mx-auto p-6 flex flex-col items-center justify-center min-h-[50vh] space-y-4">
        <div className="flex items-center gap-2 p-4 text-destructive border border-destructive/50 rounded-lg bg-destructive/10 max-w-md">
          <AlertCircle className="h-4 w-4" />
          <div className="flex flex-col">
            <span className="font-medium">Error</span>
            <span className="text-sm">{error || "Contract not found"}</span>
          </div>
        </div>
        <Button onClick={() => router.back()}>Go Back</Button>
      </div>
    );
  }

  return (
    <div className="space-y-6 container mx-auto p-6 max-w-7xl">
      <Button
        variant="ghost"
        size="sm"
        onClick={() => router.back()}
        className="gap-1 mb-2"
      >
        ← Back to List
      </Button>

      <FuturesDetailHeader data={info} />

      <Tabs defaultValue="chart" className="space-y-4">
        <TabsList>
          <TabsTrigger value="chart">K-Line Chart</TabsTrigger>
          <TabsTrigger value="info">Contract Info</TabsTrigger>
        </TabsList>

        <TabsContent value="chart" className="space-y-4">
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div className="lg:col-span-2 border rounded-xl overflow-hidden bg-card shadow-sm">
              <div className="p-4 border-b">
                <h3 className="font-semibold">Trend Analysis</h3>
              </div>
              <div className="p-4">
                <KlineChart symbol={symbol} name={info.name} height={500} />
              </div>
            </div>
            <div className="space-y-6">
              {detail && <FuturesContractInfo detail={detail} />}
            </div>
          </div>
        </TabsContent>

        <TabsContent value="info">
          {detail ? (
            <FuturesContractInfo detail={detail} />
          ) : (
            <div className="p-12 text-center text-muted-foreground border rounded-xl border-dashed">
              No contract details available for this symbol.
            </div>
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default function VarietyDetailPage() {
  return (
    <Suspense
      fallback={
        <div className="container mx-auto p-6">
          <Skeleton className="h-32 w-full" />
        </div>
      }
    >
      <DetailContent />
    </Suspense>
  );
}
