# AkShare Backend Service

一个使用 Actix-web 框架开发的类似 GitHub akshare 的后端服务，提供股票和期货数据查询API。

## 功能特性

- 🚀 基于 Actix-web 高性能异步框架
- 📊 股票信息查询
- 🔮 期货实时数据查询（新浪数据源）
- 🌍 外盘期货实时行情（LME金属、COMEX贵金属等）
- 📈 历史数据获取
- 🔍 股票/期货列表查询
- 🏥 健康检查接口
- 📝 结构化API响应
- ⏱️ 请求超时保护（防止连接挂起）

## API 接口

### 健康检查
```
GET /api/v1/health
```

### 股票相关接口

#### 获取股票列表
```
GET /api/v1/stocks?limit=10
```

#### 获取单个股票信息
```
GET /api/v1/stocks/{symbol}
```

#### 获取股票历史数据
```
GET /api/v1/stocks/{symbol}/history?start_date=2024-01-01&end_date=2024-01-31&limit=30
```

### 期货相关接口

#### 获取期货列表（主力合约）
```
GET /api/v1/futures?exchange=DCE&limit=10
```

#### 获取单个期货合约信息
```
GET /api/v1/futures/{symbol}
```

#### 获取期货历史数据
```
GET /api/v1/futures/{symbol}/history?start_date=2024-01-01&end_date=2024-01-31&limit=30
```

#### 获取支持的交易所列表
```
GET /api/v1/futures/exchanges
```

#### 批量获取期货数据
```
POST /api/v1/futures/batch
Content-Type: application/json

["CU2405", "AL2405", "ZN2405"]
```

#### 获取外盘期货品种列表
```
GET /api/v1/futures/foreign/symbols
```

#### 获取外盘期货实时行情
```
POST /api/v1/futures/foreign/realtime
Content-Type: application/json

["CAD", "AHD", "ZSD", "NID"]
```

支持的外盘期货品种：
- **LME金属**: CAD(伦铜), AHD(伦铝), ZSD(伦锌), NID(伦镍), PBD(伦铅), SND(伦锡)
- **COMEX贵金属**: GC(黄金), SI(白银)
- **NYMEX能源**: CL(原油)

## 期货合约代码说明

支持的交易所及合约格式：

- **大连商品交易所 (DCE)**: A2405, M2405, Y2405, C2405, L2405, V2405, PP2405, J2405, JM2405, I2405
- **郑州商品交易所 (CZCE)**: WH405, CF405, SR405, TA405, OI405, RM405, MA405, ZC405, FG405
- **上海期货交易所 (SHFE)**: CU2405, AL2405, ZN2405, PB2405, NI2405, AU2406, AG2406, RB2405, HC2405, RU2405
- **上海国际能源交易中心 (INE)**: SC2405, NR2405, LU2405
- **中国金融期货交易所 (CFFEX)**: IF2404, IC2404, IH2404, T2406, TF2406

## 快速开始

### 安装依赖
```bash
cargo build
```

### 运行服务
```bash
cargo run
```

服务将在 `http://127.0.0.1:8080` 启动

### 测试接口
```bash
# 健康检查
curl http://127.0.0.1:8080/api/v1/health

# 获取期货列表
curl http://127.0.0.1:8080/api/v1/futures

# 获取铜期货主力合约信息
curl http://127.0.0.1:8080/api/v1/futures/CU2405

# 获取大商所期货列表
curl "http://127.0.0.1:8080/api/v1/futures?exchange=DCE&limit=5"

# 获取交易所列表
curl http://127.0.0.1:8080/api/v1/futures/exchanges

# 批量获取期货数据
curl -X POST http://127.0.0.1:8080/api/v1/futures/batch \
  -H "Content-Type: application/json" \
  -d '["CU2405", "AL2405", "ZN2405"]'

# 获取外盘期货品种列表
curl http://127.0.0.1:8080/api/v1/futures/foreign/symbols

# 获取外盘期货实时行情（LME金属）
curl -X POST http://127.0.0.1:8080/api/v1/futures/foreign/realtime \
  -H "Content-Type: application/json" \
  -d '["CAD", "AHD", "ZSD", "NID"]'

# 获取期货历史数据
curl "http://127.0.0.1:8080/api/v1/futures/CU2405/history?limit=10"
```

## 项目结构

```
src/
├── main.rs              # 应用入口
├── handlers/            # HTTP处理器
│   ├── mod.rs
│   ├── health.rs        # 健康检查
│   ├── stock.rs         # 股票相关接口
│   └── futures.rs       # 期货相关接口
├── models/              # 数据模型
│   ├── mod.rs
│   ├── stock.rs         # 股票数据结构
│   ├── futures.rs       # 期货数据结构
│   └── response.rs      # API响应结构
└── services/            # 业务逻辑
    ├── mod.rs
    ├── stock_service.rs # 股票数据服务
    └── futures_service.rs # 期货数据服务
```

## 数据源说明

### 期货数据
- **实时数据**: 新浪财经期货API
- **外盘数据**: 新浪财经外盘期货API（LME、COMEX、NYMEX）
- **数据更新**: 交易时间内实时更新
- **支持合约**: 国内四大期货交易所主力合约 + 外盘主要品种
- **超时保护**: 30秒请求超时，10秒连接超时

### 股票数据
- **当前版本**: 使用模拟数据
- **扩展计划**: 可集成新浪、腾讯等股票API

## 扩展功能

当前版本的扩展计划：

1. **更多数据源**：集成Wind、同花顺、东方财富等数据API
2. **数据库支持**：添加PostgreSQL或MongoDB存储历史数据
3. **缓存机制**：使用Redis缓存热门合约数据
4. **认证授权**：添加JWT认证和API限流
5. **WebSocket**：实时期货价格推送
6. **监控日志**：集成Prometheus和日志聚合
7. **期权数据**：添加期权合约数据支持

## 开发环境

- Rust 1.70+
- Cargo
- Actix-web 4.4