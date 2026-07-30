# AkShare (AK) 模块接口

AK 模块提供 AkShare 基础元数据信息以及文章/宏观不确定性指数等数据接口。

## 基础路径

`/api/v1/ak`

---

## 1. 获取 AK 模块信息

获取服务元数据、版本以及支持的数据分类列表。

- **URL**: `/api/v1/ak/info`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

无

### 响应示例

```json
{
  "success": true,
  "data": {
    "name": "AkShare Rust Service",
    "version": "0.1.0",
    "description": "AkShare financial data service API in Rust",
    "categories": [
      "stocks",
      "futures",
      "bond",
      "fx",
      "crypto",
      "index",
      "macro",
      "article"
    ]
  },
  "error": null
}
```

---

## 2. 经济政策不确定性指数 (EPU Index)

获取指定国家或地区的经济政策不确定性指数历史数据。数据来源于 [Economic Policy Uncertainty](https://www.policyuncertainty.com/index.html)。

- **URL**: `/api/v1/ak/article_epu_index`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"China"` | 指定国家或地区名称，如 `"China"`, `"USA"`, `"Hong Kong"`, `"Germany"` 等 |

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "year": 2023,
      "month": 1,
      "epu": 120.5,
      "SCMP_China_EPU": 120.5
    },
    {
      "year": 2023,
      "month": 2,
      "epu": 115.3,
      "SCMP_China_EPU": 115.3
    }
  ],
  "error": null
}
```

---

## 3. 美联储 FRED-MD (月度) 宏观经济数据

获取美联储圣路易斯分行 (Federal Reserve Bank of St. Louis) 的 FRED-MD 月度宏观经济数据。

- **URL**: `/api/v1/ak/fred_md`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `date` | `string` | 否 | `"2020-01"` | 年月字符串，如 `"2020-03"`, `"2023-01"` |

---

## 4. 美联储 FRED-QD (季度) 宏观经济数据

获取美联储圣路易斯分行的 FRED-QD 季度宏观经济数据。

- **URL**: `/api/v1/ak/fred_qd`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `date` | `string` | 否 | `"2020-01"` | 年月字符串，如 `"2020-03"`, `"2023-01"` |

---

## 5. Oxford-Man 研究所 Realized Volatility 数据

获取 Oxford-Man 研究所金融资产/指数的实际波动率序列数据。

- **URL**: `/api/v1/ak/article_oman_rv`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"FTSE"` | 指数/资产代码，如 `"FTSE"`, `"SPX"`, `"SSEC"` 等 |
| `index` | `string` | 否 | `"rk_th2"` | 波动率计算指标类型，如 `"rk_th2"`, `"rv5"`, `"rv10"` 等 |

---

## 6. Oxford-Man 研究所 Realized Volatility 简易数据

获取 Oxford-Man 研究所首页的实际波动率简化折线数据。

- **URL**: `/api/v1/ak/article_oman_rv_short`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"FTSE"` | 指数/资产代码 |

---

## 7. 修大成 Risk Lab Realized Volatility 数据

获取芝加哥大学修大成教授团队 Risk Lab 的资产实际波动率数据。

- **URL**: `/api/v1/ak/article_rlab_rv`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"39693"` | 股票/品种 Ticker 代号 |

### 响应示例

```json
{
  "success": true,
  "data": [
    {
      "date": "1996-01-02",
      "value": 0.1234
    },
    {
      "date": "1996-01-04",
      "value": 0.5678
    }
  ],
  "error": null
}
```
