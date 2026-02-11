// API 工具层 - 封装后端 API 调用

const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_BASE_URL || "http://localhost:8080/api/v1";
const API_TOKEN = process.env.NEXT_PUBLIC_API_TOKEN || "12345678";

// ==================== 类型定义 ====================

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  message: string;
  timestamp: string;
}

export interface FuturesInfo {
  symbol: string;
  name: string;
  current_price: number;
  change: number;
  change_percent: number;
  volume: number;
  amount: number;
  open: number;
  high: number;
  low: number;
  prev_close: number;
  updated_at: string;
}

export interface FuturesHistoryData {
  symbol: string;
  date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
  hold: number;
}

export interface FuturesExchange {
  code: string;
  name: string;
  name_en: string;
}

export interface FuturesSymbolMark {
  symbol: string;
  name: string;
  exchange: string;
  node: string;
}

export interface FuturesMainContract {
  symbol: string;
  name: string;
  current_price: number;
  change_percent: number;
}

export interface StockInfo {
  symbol: string;
  name: string;
  current_price: number;
  change: number;
  change_percent: number;
  volume: number;
  amount: number;
  open: number;
  high: number;
  low: number;
  prev_close: number;
  market_cap: number | null;
  updated_at: string;
}

export interface StockHistoryData {
  symbol: string;
  date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface ForeignFuturesSymbol {
  symbol: string;
  name: string;
}

// ==================== 通用请求函数 ====================

export async function fetchApi<T>(
  endpoint: string,
  options?: RequestInit,
): Promise<ApiResponse<T>> {
  const url = `${API_BASE_URL}${endpoint}`;
  const res = await fetch(url, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_TOKEN}`,
      ...options?.headers,
    },
  });

  if (!res.ok) {
    return {
      success: false,
      data: null,
      message: `请求失败: ${res.status} ${res.statusText}`,
      timestamp: new Date().toISOString(),
    };
  }

  return res.json();
}

// ==================== 健康检查 ====================

export const getHealth = () => fetchApi<{ status: string }>("/health");

// ==================== 期货 - 交易所和品种 ====================

export const getExchanges = () =>
  fetchApi<FuturesExchange[]>("/futures/exchanges");

export const getSymbols = (exchange?: string) =>
  fetchApi<FuturesSymbolMark[]>(
    exchange ? `/futures/symbols/${exchange}` : "/futures/symbols",
  );

// ==================== 期货 - 实时行情 ====================

export const getFuturesInfo = (symbol: string) =>
  fetchApi<FuturesInfo>(`/futures/${symbol}`);

export const getFuturesBatch = (symbols: string[]) =>
  fetchApi<FuturesInfo[]>("/futures/batch", {
    method: "POST",
    body: JSON.stringify(symbols),
  });

export const getFuturesRealtime = (name: string) =>
  fetchApi<FuturesInfo[]>(`/futures/realtime/${encodeURIComponent(name)}`);

// ==================== 期货 - K线历史 ====================

export const getFuturesHistory = (symbol: string, limit = 30) =>
  fetchApi<FuturesHistoryData[]>(`/futures/${symbol}/history?limit=${limit}`);

export const getFuturesMinute = (symbol: string, period = 5) =>
  fetchApi<unknown[]>(`/futures/${symbol}/minute?period=${period}`);

// ==================== 期货 - 列表 ====================

export const listFutures = (exchange?: string, limit = 20) => {
  const params = new URLSearchParams();
  if (exchange) params.set("exchange", exchange);
  params.set("limit", String(limit));
  return fetchApi<FuturesInfo[]>(`/futures?${params.toString()}`);
};

// ==================== 期货 - 主力合约 ====================

export const getMainDisplay = () =>
  fetchApi<FuturesMainContract[]>("/futures/main/display");

export const getMainByExchange = (exchange: string) =>
  fetchApi<FuturesMainContract[]>(`/futures/main/${exchange}`);

// ==================== 期货 - 交易费用 ====================

export const getFuturesFees = () =>
  fetchApi<Record<string, unknown>[]>("/futures/fees");

export const getCommInfo = (exchange?: string) => {
  const params = exchange ? `?exchange=${encodeURIComponent(exchange)}` : "";
  return fetchApi<Record<string, unknown>[]>(`/futures/comm_info${params}`);
};

// ==================== 期货 - 外盘 ====================

export const getForeignSymbols = () =>
  fetchApi<ForeignFuturesSymbol[]>("/futures/foreign/symbols");

export const getForeignRealtime = (symbols: string[]) =>
  fetchApi<Record<string, unknown>[]>("/futures/foreign/realtime", {
    method: "POST",
    body: JSON.stringify(symbols),
  });

export const getForeignHistory = (symbol: string) =>
  fetchApi<Record<string, unknown>[]>(`/futures/foreign/${symbol}/history`);

export const getForeignDetail = (symbol: string) =>
  fetchApi<Record<string, unknown>>(`/futures/foreign/${symbol}/detail`);

// ==================== 股票 ====================

export const getStockList = (limit = 50) =>
  fetchApi<StockInfo[]>(`/stocks?limit=${limit}`);

export const getStockInfo = (symbol: string) =>
  fetchApi<StockInfo>(`/stocks/${symbol}`);

export const getStockHistory = (symbol: string, limit = 30) =>
  fetchApi<StockHistoryData[]>(`/stocks/${symbol}/history?limit=${limit}`);

// ==================== 期货 - 合约详情 ====================

export interface FuturesContractDetail {
  symbol: string;
  name: string;
  exchange: string;
  trading_unit: string;
  quote_unit: string;
  min_price_change: string;
  price_limit: string;
  contract_months: string;
  trading_hours: string;
  last_trading_day: string;
  last_delivery_day: string;
  delivery_grade: string;
  margin: string;
  delivery_method: string;
}

export const getContractDetail = (symbol: string) =>
  fetchApi<FuturesContractDetail>(`/futures/${symbol}/detail`);
