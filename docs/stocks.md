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
