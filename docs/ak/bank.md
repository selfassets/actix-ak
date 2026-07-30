# 银行与金融监管接口

提供国家金融监督管理总局（原银保监会）行政处罚公开表等数据接口。

## 基础路径

`/api/v1/ak/bank`

---

## 1. 获取行政处罚数据总条数

- **URL**: `/api/v1/ak/bank/fjcf_total_num`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `item` | `string` | 否 | `"分局本级"` | 处罚级别：选择范围 `{"机关", "本级", "分局本级"}` |

---

## 2. 获取行政处罚数据总页数

- **URL**: `/api/v1/ak/bank/fjcf_total_page`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `item` | `string` | 否 | `"分局本级"` | 处罚级别 |
| `begin` | `integer` | 否 | `1` | 起始页码 |

---

## 3. 获取行政处罚列表概要

- **URL**: `/api/v1/ak/bank/fjcf_list`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `page` | `integer` | 否 | `1` | 获取页数 |
| `item` | `string` | 否 | `"分局本级"` | 处罚级别 |
| `begin` | `integer` | 否 | `1` | 起始页码 |

---

## 4. 获取行政处罚信息公开表详情

- **URL**: `/api/v1/ak/bank/fjcf_detail`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `page` | `integer` | 否 | `1` | 获取页数 |
| `item` | `string` | 否 | `"分局本级"` | 处罚级别 |
| `begin` | `integer` | 否 | `1` | 起始页码 |
