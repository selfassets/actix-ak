# 加密货币行情与持仓接口

提供芝加哥商业交易所 (CME) 比特币成交量及全球机构/上市公司持仓报告。

## 基础路径

`/api/v1/ak/crypto`

---

## 1. CME 比特币成交量及持仓报告

- **URL**: `/api/v1/ak/crypto/bitcoin_cme`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `date` | `string` | 否 | `"20230830"` | 查询日期 YYYYMMDD |

---

## 2. 全球机构及上市公司比特币持仓报告

- **URL**: `/api/v1/ak/crypto/bitcoin_hold_report`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token
