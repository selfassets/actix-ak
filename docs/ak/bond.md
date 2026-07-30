# 债券与国债收益率接口

提供可转债实时行情、中国/美国国债收益率历史 K 线及中美国债收益率对比数据。

## 基础路径

`/api/v1/ak/bond`

---

## 1. 沪深可转债实时行情

- **URL**: `/api/v1/ak/bond/zh_cov_spot`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 2. 中国国债收益率历史数据（新浪源）

- **URL**: `/api/v1/ak/bond/gb_zh_sina`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"中国10年期国债"` | 期限选择，如 `"中国1年期国债"`, `"中国10年期国债"` 等 |

---

## 3. 美国国债收益率历史数据（新浪源）

- **URL**: `/api/v1/ak/bond/gb_us_sina`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"美国10年期国债"` | 期限选择，如 `"美国1年期国债"`, `"美国10年期国债"` 等 |

---

## 4. 中美国债收益率对比数据（东方财富源）

- **URL**: `/api/v1/ak/bond/zh_us_rate`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 5. 上证质押式国债逆回购行情

- **URL**: `/api/v1/ak/bond/sh_buy_back`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 6. 深证质押式国债逆回购行情

- **URL**: `/api/v1/ak/bond/sz_buy_back`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 7. 集思录可转债等权指数历史

- **URL**: `/api/v1/ak/bond/cb_index_jsl`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 8. 集思录可转债强赎信息列表

- **URL**: `/api/v1/ak/bond/cb_redeem_jsl`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 9. 新浪财经可转债详情资料

- **URL**: `/api/v1/ak/bond/cb_profile_sina`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"sz128039"` | 带市场前缀的可转债代码（如 `"sz128039"`, `"sh113527"`） |

---

## 10. 东方财富网可转债比价表数据

- **URL**: `/api/v1/ak/bond/cov_comparison`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token




