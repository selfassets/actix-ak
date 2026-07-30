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

---

## 11. 沪深债券实时行情数据（新浪源）

- **URL**: `/api/v1/ak/bond/zh_hs_spot`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 12. 质押式国债逆回购历史 K 线行情（东方财富）

- **URL**: `/api/v1/ak/bond/buy_back_hist`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"204001"` | 质押式国债逆回购代码，如 `"204001"` (GC001), `"131810"` (R-001) |

---

## 13. 中国外汇交易中心收益率曲线品种映射表

- **URL**: `/api/v1/ak/bond/china_close_return_map`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 14. 上交所债券现货市场概览

- **URL**: `/api/v1/ak/bond/cash_summary_sse`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `start_date` | `string` | 否 | `"20210111"` | 查询日期 YYYYMMDD |

---

## 15. 上交所债券成交概览汇总

- **URL**: `/api/v1/ak/bond/deal_summary_sse`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `start_date` | `string` | 否 | `"20210104"` | 查询日期 YYYYMMDD |

---

## 16. 同花顺可转债基本信息列表

- **URL**: `/api/v1/ak/bond/zh_cov_info_ths`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 17. 新浪财经可转债概况汇总

- **URL**: `/api/v1/ak/bond/cb_summary_sina`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"sh155255"` | 带市场前缀的可转债代码 |

---

## 18. 中国外汇交易中心现券做市报价

- **URL**: `/api/v1/ak/bond/spot_quote`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 19. 中国外汇交易中心现券成交行情

- **URL**: `/api/v1/ak/bond/spot_deal`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 20. 中国债券信息网国债及各期限收益率曲线

- **URL**: `/api/v1/ak/bond/china_yield`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `start_date` | `string` | 否 | `"20200204"` | 开始日期 YYYYMMDD |
| `end_date` | `string` | 否 | `"20210124"` | 结束日期 YYYYMMDD |

---

## 21. 中国货币网债券信息查询参数

- **URL**: `/api/v1/ak/bond/info_cm_query`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"评级等级"` | 查询指标分类：选择范围 `{"主承销商", "债券类型", "息票类型", "发行年份", "评级等级"}` |

---

## 22. 中国货币网债券信息列表

- **URL**: `/api/v1/ak/bond/info_cm`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `""` | 债券代码进行精确或模糊匹配过滤 |

---

## 23. 中国银行间市场交易商协会 (NAFMII) 非金融企业债务融资工具注册信息

- **URL**: `/api/v1/ak/bond/debt_nafmii`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"1"` | 分页页码 |

---

## 24. 中国货币网单只债券详情信息

- **URL**: `/api/v1/ak/bond/info_detail_cm`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"egfjh08154"` | 债券定义代码 bondDefinedCode |

---

## 25. 巨潮资讯债券发行数据

- **URL**: `/api/v1/ak/bond/issue_cninfo`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `start_date` | `string` | 否 | `"20210910"` | 开始日期 YYYYMMDD |
| `end_date` | `string` | 否 | `"20211109"` | 结束日期 YYYYMMDD |

---

## 26. 东方财富可转债价值分析 (溢价率分析)

- **URL**: `/api/v1/ak/bond/zh_cov_value_analysis`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"113527"` | 可转债代码 |

---

## 27. 中国债券信息网中债国债指数

- **URL**: `/api/v1/ak/bond/treasury_index_cbond`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"5Y"` | 指数期限，选择范围 `{'0-1Y', '0-3Y', '0-5Y', '0-10Y', '1-3Y', '1-5Y', '1-10Y', '3-5Y', '5Y', '7Y', '7-10Y', '10Y', '30Y'}` |

---

## 28. 沪深可转债历史日 K 线行情（新浪源）

- **URL**: `/api/v1/ak/bond/zh_hs_cov_daily`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"sh010107"` | 沪深可转债代码，如 `"sh010107"`, `"sz128039"` |

---

## 29. 沪深现券/债券历史日 K 线行情（新浪源）

- **URL**: `/api/v1/ak/bond/zh_hs_daily`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"sh010107"` | 沪深债券代码，如 `"sh010107"` |

---

## 30. 集思录可转债转股价调整日志

- **URL**: `/api/v1/ak/bond/cb_adj_logs_jsl`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"128013"` | 可转债代码 |

---

## 31. 中国债券信息网中债指数可选项列表

- **URL**: `/api/v1/ak/bond/available_index_cbond`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

---

## 32. 中国债券信息网中债通用指数查询

- **URL**: `/api/v1/ak/bond/index_general_cbond`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `index_category` | `string` | 否 | `"新综合指数"` | 指数分类（如 `"新综合指数"`, `"中债-国债指数"`, `"金融债指数"`） |
| `indicator` | `string` | 否 | `"全价"` | 指标类型（如 `"全价"`, `"净价"`, `"财富"`） |
| `period` | `string` | 否 | `"总值"` | 期限分段（如 `"总值"`, `"1年以下"`, `"1-3年"`, `"3-5年"`） |

---

## 33. 中国外汇交易中心收盘收益率曲线历史数据

- **URL**: `/api/v1/ak/bond/china_close_return`
- **Method**: `GET`
- **Auth**: 需要 Bearer Token

### 请求参数

| 参数名 | 类型 | 必填 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- | :--- |
| `symbol` | `string` | 否 | `"CYCC000"` | 债券收益率曲线类型代码 |
| `start_date` | `string` | 否 | `"20231101"` | 开始日期 YYYYMMDD |
| `end_date` | `string` | 否 | `"20231101"` | 结束日期 YYYYMMDD |






















