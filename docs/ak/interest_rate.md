# 拆借利率与利率接口

提供东方财富网各类银行间拆借利率（Shibor、Chibor、Libor、Euribor、Hibor、Sibor 等）历史数据。

## 基础路径

`/api/v1/ak/interest_rate`

---

## 1. 银行间同业拆借利率

- **URL**: `/api/v1/ak/interest_rate/rate_interbank`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `market` | `string` | 否 | `"上海银行同业拆借市场"` | 市场选择（如 `"上海银行同业拆借市场"`, `"伦敦银行同业拆借市场"`, `"香港银行同业拆借市场"`） |
| `symbol` | `string` | 否 | `"Shibor人民币"` | 拆借品种（如 `"Shibor人民币"`, `"Libor美元"`, `"Hibor港币"` 等） |
| `indicator` | `string` | 否 | `"隔夜"` | 期限（如 `"隔夜"`, `"1周"`, `"1月"`, `"1年"`） |
