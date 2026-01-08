---
layout: default
title: 股票接口
nav_order: 4
permalink: /stocks/
---

# 股票接口

提供 A 股股票的实时行情和历史 K 线数据。

---

## GET /stocks

获取股票列表。

**查询参数**：

- `limit`: 返回数量限制（可选）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/stocks?limit=20" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "600000",
      "name": "浦发银行",
      "current_price": 7.5,
      "change": 0.05,
      "change_percent": 0.67,
      "volume": 200000,
      "market_cap": 220000000000.0,
      "updated_at": "2024-05-15 15:00:00"
    },
    {
      "symbol": "600036",
      "name": "招商银行",
      "current_price": 32.5,
      "change": -0.1,
      "change_percent": -0.31,
      "volume": 150000,
      "market_cap": 820000000000.0,
      "updated_at": "2024-05-15 15:00:00"
    }
  ],
  "error": null
}
```

---

## GET /stocks/{symbol}

获取单只股票信息。

**路径参数**：

- `symbol`: 股票代码（如 600000）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/stocks/600000" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": {
    "symbol": "600000",
    "name": "浦发银行",
    "current_price": 7.5,
    "change": 0.05,
    "change_percent": 0.67,
    "volume": 200000,
    "market_cap": 220000000000.0,
    "updated_at": "2024-05-15 15:00:00"
  },
  "error": null
}
```

---

## GET /stocks/{symbol}/history

获取股票历史 K 线数据。

**路径参数**：

- `symbol`: 股票代码

**查询参数**：

- `limit`: 返回数量限制（可选，默认 30）

**请求示例**

```bash
curl -X GET "{{baseUrl}}/stocks/600000/history?limit=30" \
  -H "Authorization: Bearer {{token}}"
```

**响应示例**

```json
{
  "success": true,
  "data": [
    {
      "symbol": "600000",
      "date": "2024-05-15",
      "open": 7.45,
      "high": 7.52,
      "low": 7.44,
      "close": 7.5,
      "volume": 200000
    },
    {
      "symbol": "600000",
      "date": "2024-05-14",
      "open": 7.4,
      "high": 7.48,
      "low": 7.39,
      "close": 7.45,
      "volume": 180000
    }
  ],
  "error": null
}
```

---

[返回首页](index)

# 股票接口

提供 A 股股票的实时行情和历史 K 线数据。

---

## GET /stocks

获取股票列表。

**查询参数**：

- `limit`: 返回数量限制（可选）

```bash
curl -X GET "{{baseUrl}}/stocks?limit=20" \
  -H "Authorization: Bearer {{token}}"
```

---

## GET /stocks/{symbol}

获取单只股票信息。

**路径参数**：

- `symbol`: 股票代码（如 600000）

```bash
curl -X GET "{{baseUrl}}/stocks/600000" \
  -H "Authorization: Bearer {{token}}"
```

---

## GET /stocks/{symbol}/history

获取股票历史 K 线数据。

**路径参数**：

- `symbol`: 股票代码

**查询参数**：

- `limit`: 返回数量限制（可选，默认 30）

```bash
curl -X GET "{{baseUrl}}/stocks/600000/history?limit=30" \
  -H "Authorization: Bearer {{token}}"
```

---

[返回首页](index)
