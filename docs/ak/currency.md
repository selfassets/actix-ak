# 外汇与货币数据接口

提供中国银行人民币牌价历史数据查询。

## 基础路径

`/api/v1/ak/currency`

---

## 1. 中国银行人民币牌价历史数据

- **URL**: `/api/v1/ak/currency/boc_sina`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"美元"` | 货币名称，如 `"美元"`, `"欧元"`, `"日元"`, `"港币"`, `"英镑"` 等 |
| `start_date` | `string` | 否 | `"20230101"` | 起始日期 YYYYMMDD |
| `end_date` | `string` | 否 | `"20231231"` | 结束日期 YYYYMMDD |
