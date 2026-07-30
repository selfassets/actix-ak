# 波动率与量化计算接口

提供 Yang-Zhang 已实现波动率 (Yang-Zhang Realized Volatility) 估算等算法接口。

## 基础路径

`/api/v1/ak/cal`

---

## 1. Yang-Zhang 已实现波动率计算

- **URL**: `/api/v1/ak/cal/volatility_yz`
- **Method**: `POST`
- **Auth**: 需要 Bearer Token
- **Content-Type**: `application/json`

### 请求体示例

```json
[
  { "date": "2024-01-01", "open": 10.0, "high": 10.5, "low": 9.8, "close": 10.2 },
  { "date": "2024-01-02", "open": 10.3, "high": 10.8, "low": 10.1, "close": 10.6 },
  { "date": "2024-01-03", "open": 10.5, "high": 10.9, "low": 10.2, "close": 10.4 }
]
```

---

## 2. 股票分钟行情清洗 (东方财富)

- **URL**: `/api/v1/ak/cal/rv_stock_em`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"000001"` | 股票代码 |
| `period` | `string` | 否 | `"5"` | 时间周期 ('1','5','15','30','60') |
| `adjust` | `string` | 否 | `"hfq"` | 复权方式 ('','qfq','hfq') |

---

## 3. 期货分钟行情清洗 (新浪源)

- **URL**: `/api/v1/ak/cal/rv_futures_sina`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"IF2008"` | 期货合约代码 |
| `period` | `string` | 否 | `"5"` | 时间周期 ('1','5','15','30','60') |

