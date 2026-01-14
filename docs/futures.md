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

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/exchanges" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "code": "SHFE",
      "name": "上期所",
      "description": "Shanghai Futures Exchange"
    },
    {
      "code": "DCE",
      "name": "大商所",
      "description": "Dalian Commodity Exchange"
    }
  ],
  "error": null
}
```

### GET /futures/symbols

获取所有品种映射表（从新浪 JS 动态解析）。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/symbols" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "exchange": "上期所",
      "symbol": "铜",
      "mark": "cu"
    },
    {
      "exchange": "大商所",
      "symbol": "豆一",
      "mark": "a"
    }
  ],
  "error": null
}
```

### GET /futures/symbols/{exchange}

获取指定交易所的品种列表。

**路径参数**：

- `exchange`: 交易所代码（SHFE/DCE/CZCE/CFFEX）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/symbols/SHFE" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "exchange": "上期所",
      "symbol": "铜",
      "mark": "cu"
    },
    {
      "exchange": "上期所",
      "symbol": "铝",
      "mark": "al"
    }
  ],
  "error": null
}
```

---

## 实时行情

### GET /futures/{symbol}

获取单个合约实时数据。

**路径参数**：

- `symbol`: 合约代码（如 CU2602, RB2605, IF2603）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/CU2602" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": {
    "symbol": "CU2602",
    "name": "沪铜2602",
    "current_price": 68500.0,
    "change": 120.0,
    "change_percent": 0.18,
    "volume": 15000,
    "open": 68400.0,
    "high": 68600.0,
    "low": 68350.0,
    "settlement": 68450.0,
    "prev_settlement": 68380.0,
    "open_interest": 45000,
    "updated_at": "2024-05-15 14:30:00"
  },
  "error": null
}
```

### POST /futures/batch

批量获取期货实时数据。

**请求体**：合约代码数组

**请求示例**

```bash
curl -X POST "{{baseUrl}}/futures/batch" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["CU2602", "AL2602", "RB2605", "AU2602"]'
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "CU2602",
      "name": "沪铜2602",
      "current_price": 68500.0,
      "change": 120.0,
      "change_percent": 0.18,
      "volume": 15000,
      "updated_at": "2024-05-15 14:30:00"
      // ... 其他字段
    },
    {
      "symbol": "AL2602",
      "name": "沪铝2602",
      "current_price": 19500.0,
      "change": -50.0,
      "change_percent": -0.26,
      "volume": 8000,
      "updated_at": "2024-05-15 14:30:00"
      // ... 其他字段
    }
  ],
  "error": null
}
```

### GET /futures/realtime/{symbol}

获取品种所有合约实时数据（按品种名称）。

**路径参数**：

- `symbol`: 品种名称（如 沪铜、螺纹钢）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/realtime/沪铜" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "CU2602",
      "name": "沪铜2602",
      "current_price": 68500.0,
      "change": 120.0,
      "change_percent": 0.18,
      "volume": 15000,
      "updated_at": "2024-05-15 14:30:00"
    },
    {
      "symbol": "CU2603",
      "name": "沪铜2603",
      "current_price": 68600.0,
      "change": 150.0,
      "change_percent": 0.22,
      "volume": 5000,
      "updated_at": "2024-05-15 14:30:00"
    }
  ],
  "error": null
}
```

### GET /futures

获取期货列表（按交易所筛选）。

**查询参数**：

- `exchange`: 交易所代码（可选）
- `limit`: 返回数量限制（可选）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures?exchange=SHFE&limit=10" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "CU2602",
      "name": "沪铜2602",
      "current_price": 68500.0,
      "change": 120.0,
      "change_percent": 0.18,
      "volume": 15000,
      "updated_at": "2024-05-15 14:30:00"
      // ...
    }
  ],
  "error": null
}
```

### GET /futures/{symbol}/detail

获取合约详情。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/CU2602/detail" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": {
    "symbol": "CU2602",
    "name": "沪铜2602",
    "exchange": "上海期货交易所",
    "trading_unit": "5吨/手",
    "quote_unit": "元(人民币)/吨",
    "min_price_change": "10元/吨",
    "price_limit": "上一交易日结算价±3%",
    "contract_months": "1-12月",
    "trading_hours": "上午9:00-11:30，下午1:30-3:00",
    "last_trading_day": "合约月份的15日",
    "contract_delivery_month": "2026年02月"
  },
  "error": null
}
```

---

## K 线数据

### GET /futures/{symbol}/history

获取日 K 线历史数据。

**查询参数**：

- `limit`: 返回数量限制（可选，默认 30）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/CU2602/history?limit=10" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "CU2602",
      "date": "2024-05-15",
      "open": 68400.0,
      "high": 68600.0,
      "low": 68350.0,
      "close": 68500.0,
      "volume": 15000,
      "settlement": 68450.0,
      "open_interest": 45000
    }
    // ... 更多数据
  ],
  "error": null
}
```

### GET /futures/{symbol}/minute

获取分钟 K 线数据。

**查询参数**：

- `period`: K 线周期（1/5/15/30/60，默认 5）

**请求示例**

```bash
# 5分钟K线
curl -X GET "{{baseUrl}}/futures/CU2602/minute?period=5" \
  -H "Authorization: Bearer {{token}}"

# 60分钟K线
curl -X GET "{{baseUrl}}/futures/CU2602/minute?period=60" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "CU2602",
      "date": "2024-05-15 14:55:00",
      "open": 68480.0,
      "high": 68520.0,
      "low": 68480.0,
      "close": 68500.0,
      "volume": 200,
      "settlement": null,
      "open_interest": 45000
    }
    // ...
  ],
  "error": null
}
```

---

## 主力连续合约

### GET /futures/main/display

获取主力连续合约一览表。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/main/display" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "V0",
      "name": "PVC连续",
      "exchange": "DCE"
    },
    {
      "symbol": "RB0",
      "name": "螺纹钢连续",
      "exchange": "SHFE"
    }
  ],
  "error": null
}
```

### GET /futures/main/{exchange}

获取指定交易所的主力合约列表。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/main/SHFE" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": ["CU0", "AL0", "ZN0", "PB0", "RB0", "HC0"],
  "error": null
}
```

### GET /futures/main/{symbol}/daily

获取主力连续日 K 线数据。

**查询参数**：

- `start_date`: 开始日期（YYYYMMDD）
- `end_date`: 结束日期（YYYYMMDD）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/main/RB0/daily?start_date=20240101&end_date=20240301" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "date": "2024-01-02",
      "open": 3900.0,
      "high": 3950.0,
      "low": 3880.0,
      "close": 3920.0,
      "volume": 1200000,
      "hold": 1800000,
      "settle": 3915.0
    }
    // ...
  ],
  "error": null
}
```

---

## 持仓排名

### GET /futures/hold_pos

获取期货持仓排名数据。

**查询参数**：

- `pos_type`: 排名类型（volume/long/short）
- `contract`: 合约代码
- `date`: 日期（YYYYMMDD）

**请求示例**

```bash
# 成交量排名
curl -X GET "{{baseUrl}}/futures/hold_pos?pos_type=volume&contract=RB2510&date=20250107" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "rank": 1,
      "company": "东证期货",
      "value": 150000,
      "change": 5000
    },
    {
      "rank": 2,
      "company": "中信期货",
      "value": 140000,
      "change": -2000
    }
  ],
  "error": null
}
```

---

## 交易费用和规则

### GET /futures/fees

获取期货交易费用参照表。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/fees" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "exchange": "上海期货交易所",
      "contract_code": "rb",
      "contract_name": "螺纹钢",
      "open_fee": "成交金额的万分之1",
      "close_fee": "成交金额的万分之1",
      "long_margin_rate": "10%",
      "short_margin_rate": "10%",
      "updated_at": "2024-05-01"
    }
  ],
  "error": null
}
```

### GET /futures/comm_info

获取期货手续费信息（九期网）。

**查询参数**：

- `exchange`: 交易所名称（可选，如 上期所）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/comm_info?exchange=上期所" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "exchange": "上海期货交易所",
      "contract_name": "黄金",
      "contract_code": "au",
      "current_price": 450.0,
      "limit_up": 472.5,
      "limit_down": 427.5,
      "margin_buy": 10.0,
      "margin_sell": 10.0,
      "fee_open_yuan": 10.0,
      "fee_close_today_yuan": 0.0,
      "remark": "主力合约"
    }
  ],
  "error": null
}
```

### GET /futures/rule

获取期货交易规则。

**查询参数**：

- `date`: 日期（YYYYMMDD，可选）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/rule?date=20250328" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "exchange": "上期所",
      "product": "螺纹钢",
      "code": "RB",
      "margin_rate": 7.0,
      "price_limit": 5.0,
      "contract_size": 10.0,
      "price_tick": 1.0,
      "max_order_size": 500,
      "special_note": "无",
      "remark": null
    }
  ],
  "error": null
}
```

---

## 库存数据

### GET /futures/inventory99/symbols

获取 99 期货网品种映射表。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/inventory99/symbols" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "product_id": 1,
      "name": "豆一",
      "code": "A"
    },
    {
      "product_id": 2,
      "name": "豆二",
      "code": "B"
    }
  ],
  "error": null
}
```

### GET /futures/inventory99

获取 99 期货网库存数据。

**查询参数**：

- `symbol`: 品种名称（如 豆一）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/inventory99?symbol=豆一" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "date": "2024-05-15",
      "close_price": 5000.0,
      "inventory": 200000.0
    },
    {
      "date": "2024-05-14",
      "close_price": 5010.0,
      "inventory": 201000.0
    }
  ],
  "error": null
}
```

---

## 现货价格及基差

### GET /futures/spot_price

获取现货价格及基差数据。

**查询参数**：

- `date`: 日期（YYYYMMDD）
- `symbols`: 品种代码，逗号分隔（可选）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/spot_price?date=20240430&symbols=RB,CU" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "date": "2024-04-30",
      "symbol": "RB",
      "spot_price": 3800.0,
      "near_contract": "RB2405",
      "near_contract_price": 3780.0,
      "near_basis": -20.0,
      "dominant_contract": "RB2410",
      "dominant_contract_price": 3750.0,
      "dom_basis": -50.0
    }
  ],
  "error": null
}
```

### GET /futures/spot_price_previous

获取现货价格历史数据（含 180 日统计）。

**查询参数**：

- `date`: 日期（YYYYMMDD）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/spot_price_previous?date=20240430" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "commodity": "螺纹钢",
      "spot_price": 3800.0,
      "dominant_contract": "RB2410",
      "dominant_price": 3750.0,
      "basis": -50.0,
      "basis_rate": -1.33,
      "basis_180d_high": 100.0,
      "basis_180d_low": -100.0,
      "basis_180d_avg": 20.0
    }
  ],
  "error": null
}
```

### GET /futures/spot_price_daily

获取现货价格日线数据（日期范围）。

**查询参数**：

- `start_date`: 开始日期（YYYYMMDD）
- `end_date`: 结束日期（YYYYMMDD）
- `symbols`: 品种代码，逗号分隔（可选）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/spot_price_daily?start_date=20240101&end_date=20240105&symbols=RB,CU" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "date": "2024-01-01",
      "symbol": "RB",
      "spot_price": 3900.0
      // ... 其他字段
    }
  ],
  "error": null
}
```

---

## 外盘期货

### GET /futures/foreign/symbols

获取外盘期货品种列表。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/foreign/symbols" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "伦敦金",
      "code": "XAU"
    },
    {
      "symbol": "美原油",
      "code": "CL"
    }
  ],
  "error": null
}
```

### POST /futures/foreign/realtime

获取外盘期货实时行情。

**请求体**：品种代码数组

**请求示例**

```bash
# 贵金属
curl -X POST "{{baseUrl}}/futures/foreign/realtime" \
  -H "Authorization: Bearer {{token}}" \
  -H "Content-Type: application/json" \
  -d '["GC", "SI", "XAU", "XAG"]'
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "GC",
      "name": "纽约黄金",
      "current_price": 2050.5,
      "change": 15.2,
      "change_percent": 0.75,
      "updated_at": "2024-05-15 14:30:00"
      // ...
    }
  ],
  "error": null
}
```

### GET /futures/foreign/{symbol}/history

获取外盘期货历史数据（日 K 线）。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/foreign/GC/history" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "date": "2024-05-15",
      "open": 2040.0,
      "high": 2060.0,
      "low": 2035.0,
      "close": 2050.5,
      "volume": 100000
    }
  ],
  "error": null
}
```

### GET /futures/foreign/{symbol}/detail

获取外盘期货合约详情。

**请求示例**

```bash
curl -X GET "{{baseUrl}}/futures/foreign/GC/detail" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "name": "合约名称",
        "value": "纽约黄金"
      },
      {
        "name": "交易单位",
        "value": "100盎司/手"
      }
    ]
  },
  "error": null
}
```

---

[返回首页](index)

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
