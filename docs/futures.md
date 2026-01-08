---
layout: default
title: 期货接口
nav_order: 3
permalink: /futures/
---

# 期货接口

提供国内期货和外盘期货的实时行情、历史数据、持仓排名等信息。

## 目录

- [交易所和品种](#交易所和品种)
- [实时行情](#实时行情)
- [K 线数据](#k线数据)
- [主力连续合约](#主力连续合约)
- [持仓排名](#持仓排名)
- [交易费用和规则](#交易费用和规则)
- [库存数据](#库存数据)
- [现货价格及基差](#现货价格及基差)
- [外盘期货](#外盘期货)

---

## 交易所和品种

### GET /futures/exchanges

获取支持的交易所列表。

```bash
curl -X GET "{{baseUrl}}/futures/exchanges" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/symbols

获取所有品种映射表（从新浪 JS 动态解析）。

```bash
curl -X GET "{{baseUrl}}/futures/symbols" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/symbols/{exchange}

获取指定交易所的品种列表。

**路径参数**：

- `exchange`: 交易所代码（SHFE/DCE/CZCE/CFFEX）

```bash
curl -X GET "{{baseUrl}}/futures/symbols/SHFE" \
  -H "Authorization: Bearer {{token}}"
```

---

## 实时行情

### GET /futures/{symbol}

获取单个合约实时数据。

**路径参数**：

- `symbol`: 合约代码（如 CU2602, RB2605, IF2603）

```bash
curl -X GET "{{baseUrl}}/futures/CU2602" \
  -H "Authorization: Bearer {{token}}"
```

### POST /futures/batch

批量获取期货实时数据。

**请求体**：合约代码数组

```bash
curl -X POST "{{baseUrl}}/futures/batch" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["CU2602", "AL2602", "RB2605", "AU2602"]'
```

### GET /futures/realtime/{symbol}

获取品种所有合约实时数据（按品种名称）。

**路径参数**：

- `symbol`: 品种名称（如 沪铜、螺纹钢）

```bash
curl -X GET "{{baseUrl}}/futures/realtime/沪铜" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures

获取期货列表（按交易所筛选）。

**查询参数**：

- `exchange`: 交易所代码（可选）
- `limit`: 返回数量限制（可选）

```bash
curl -X GET "{{baseUrl}}/futures?exchange=SHFE&limit=10" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/{symbol}/detail

获取合约详情。

```bash
curl -X GET "{{baseUrl}}/futures/CU2602/detail" \
  -H "Authorization: Bearer {{token}}"
```

---

## K 线数据

### GET /futures/{symbol}/history

获取日 K 线历史数据。

**查询参数**：

- `limit`: 返回数量限制（可选，默认 30）

```bash
curl -X GET "{{baseUrl}}/futures/CU2602/history?limit=10" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/{symbol}/minute

获取分钟 K 线数据。

**查询参数**：

- `period`: K 线周期（1/5/15/30/60，默认 5）

```bash
# 5分钟K线
curl -X GET "{{baseUrl}}/futures/CU2602/minute?period=5" \
  -H "Authorization: Bearer {{token}}"

# 60分钟K线
curl -X GET "{{baseUrl}}/futures/CU2602/minute?period=60" \
  -H "Authorization: Bearer {{token}}"
```

---

## 主力连续合约

### GET /futures/main/display

获取主力连续合约一览表。

```bash
curl -X GET "{{baseUrl}}/futures/main/display" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/main/{exchange}

获取指定交易所的主力合约列表。

```bash
curl -X GET "{{baseUrl}}/futures/main/SHFE" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/main/{symbol}/daily

获取主力连续日 K 线数据。

**查询参数**：

- `start_date`: 开始日期（YYYYMMDD）
- `end_date`: 结束日期（YYYYMMDD）

```bash
curl -X GET "{{baseUrl}}/futures/main/RB0/daily?start_date=20240101&end_date=20240301" \
  -H "Authorization: Bearer {{token}}"
```

---

## 持仓排名

### GET /futures/hold_pos

获取期货持仓排名数据。

**查询参数**：

- `pos_type`: 排名类型（volume/long/short）
- `contract`: 合约代码
- `date`: 日期（YYYYMMDD）

```bash
# 成交量排名
curl -X GET "{{baseUrl}}/futures/hold_pos?pos_type=volume&contract=RB2510&date=20250107" \
  -H "Authorization: Bearer {{token}}"

# 多头持仓排名
curl -X GET "{{baseUrl}}/futures/hold_pos?pos_type=long&contract=RB2510&date=20250107" \
  -H "Authorization: Bearer {{token}}"

# 空头持仓排名
curl -X GET "{{baseUrl}}/futures/hold_pos?pos_type=short&contract=RB2510&date=20250107" \
  -H "Authorization: Bearer {{token}}"
```

---

## 交易费用和规则

### GET /futures/fees

获取期货交易费用参照表。

```bash
curl -X GET "{{baseUrl}}/futures/fees" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/comm_info

获取期货手续费信息（九期网）。

**查询参数**：

- `exchange`: 交易所名称（可选，如 上期所）

```bash
# 所有交易所
curl -X GET "{{baseUrl}}/futures/comm_info" \
  -H "Authorization: Bearer {{token}}"

# 指定交易所
curl -X GET "{{baseUrl}}/futures/comm_info?exchange=上期所" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/rule

获取期货交易规则。

**查询参数**：

- `date`: 日期（YYYYMMDD，可选）

```bash
curl -X GET "{{baseUrl}}/futures/rule?date=20250328" \
  -H "Authorization: Bearer {{token}}"
```

---

## 库存数据

### GET /futures/inventory99/symbols

获取 99 期货网品种映射表。

```bash
curl -X GET "{{baseUrl}}/futures/inventory99/symbols" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/inventory99

获取 99 期货网库存数据。

**查询参数**：

- `symbol`: 品种名称（如 豆一）

```bash
curl -X GET "{{baseUrl}}/futures/inventory99?symbol=豆一" \
  -H "Authorization: Bearer {{token}}"
```

---

## 现货价格及基差

### GET /futures/spot_price

获取现货价格及基差数据。

**查询参数**：

- `date`: 日期（YYYYMMDD）
- `symbols`: 品种代码，逗号分隔（可选）

```bash
curl -X GET "{{baseUrl}}/futures/spot_price?date=20240430&symbols=RB,CU" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/spot_price_previous

获取现货价格历史数据（含 180 日统计）。

**查询参数**：

- `date`: 日期（YYYYMMDD）

```bash
curl -X GET "{{baseUrl}}/futures/spot_price_previous?date=20240430" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/spot_price_daily

获取现货价格日线数据（日期范围）。

**查询参数**：

- `start_date`: 开始日期（YYYYMMDD）
- `end_date`: 结束日期（YYYYMMDD）
- `symbols`: 品种代码，逗号分隔（可选）

```bash
curl -X GET "{{baseUrl}}/futures/spot_price_daily?start_date=20240101&end_date=20240105&symbols=RB,CU" \
  -H "Authorization: Bearer {{token}}"
```

---

## 外盘期货

### GET /futures/foreign/symbols

获取外盘期货品种列表。

```bash
curl -X GET "{{baseUrl}}/futures/foreign/symbols" \
  -H "Authorization: Bearer {{token}}"
```

### POST /futures/foreign/realtime

获取外盘期货实时行情。

**请求体**：品种代码数组

```bash
# 贵金属
curl -X POST "{{baseUrl}}/futures/foreign/realtime" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["GC", "SI", "XAU", "XAG"]'

# 原油
curl -X POST "{{baseUrl}}/futures/foreign/realtime" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["CL", "OIL"]'

# LME金属
curl -X POST "{{baseUrl}}/futures/foreign/realtime" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["CAD", "AHD", "ZSD", "NID"]'

# 农产品
curl -X POST "{{baseUrl}}/futures/foreign/realtime" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["S", "C", "W", "BO", "SM"]'
```

### GET /futures/foreign/{symbol}/history

获取外盘期货历史数据（日 K 线）。

```bash
curl -X GET "{{baseUrl}}/futures/foreign/GC/history" \
  -H "Authorization: Bearer {{token}}"
```

### GET /futures/foreign/{symbol}/detail

获取外盘期货合约详情。

```bash
curl -X GET "{{baseUrl}}/futures/foreign/GC/detail" \
  -H "Authorization: Bearer {{token}}"
```

---

[返回首页](index)
