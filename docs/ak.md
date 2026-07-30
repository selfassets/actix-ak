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
