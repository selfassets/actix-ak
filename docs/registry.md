# 注册中心

独立运行的服务注册中心，接收服务注册和心跳，管理服务实例生命周期。支持 **用户名/密码 + JWT** 认证。

## 架构

注册中心作为独立服务（`registry-server`）运行，actix-ak 实例通过内置客户端 **登录获取 JWT Token** 后，向其注册并定时发送心跳。

```
registry-server (:8081)          actix-ak (:8080)
┌──────────────────────┐         ┌──────────────────────┐
│  ServiceRegistry     │◄────────│  RegistryClient      │
│  ├ 认证（JWT）       │ login   │  ├ 启动时登录+注册   │
│  ├ 注册/注销         │◄────────│  ├ 定时心跳（10s）   │
│  ├ 心跳接收          │heartbeat│  ├ Token过期自动刷新  │
│  ├ 过期清理（30s）   │         │  └ 失败自动重连       │
│  └ Web 仪表板（登录）│         └──────────────────────┘
└──────────────────────┘
```

## 快速开始

> ⚠️ **启动顺序**：必须先启动注册中心，再启动 actix-ak。actix-ak 启动时会自动登录并注册，如果注册中心未就绪，客户端会每 5 秒重试直至成功。

### 第一步：启动注册中心

```bash
cargo run -p registry-server
```

默认监听 `http://localhost:8081`，浏览器访问可查看仪表板（需登录）。

默认管理员账号：`admin` / `admin123`

### 第二步：启动 actix-ak

方式一：修改 `config.json`，开启注册并配置注册中心凭据：

```json
{
  "registry": {
    "enabled": true,
    "registry_url": "http://localhost:8081",
    "service_name": "actix-ak",
    "heartbeat_interval_secs": 10,
    "registry_username": "admin",
    "registry_password": "admin123"
  }
}
```

方式二：通过命令行环境变量直接设置：

```bash
REGISTRY_ENABLED=true \
REGISTRY_URL=http://localhost:8081 \
REGISTRY_USERNAME=admin \
REGISTRY_PASSWORD=admin123 \
cargo run -p actix-ak
```

`enabled` 为 `true` 且 `registry_url` 非空时，启动后将自动登录、注册并每 10 秒发送心跳。

## 配置

### registry-server/config.json

| 字段                              | 默认值                       | 说明                                 |
| --------------------------------- | ---------------------------- | ------------------------------------ |
| `server.host`                     | `0.0.0.0`                    | 监听地址                             |
| `server.port`                     | `8081`                       | 监听端口                             |
| `registry.heartbeat_timeout_secs` | `30`                         | 心跳超时阈值（秒），超时实例将被清理 |
| `log.level`                       | `info`                       | 日志级别                             |
| `auth.jwt_secret`                 | `registry-server-secret-key` | JWT 签名密钥                         |
| `auth.admin_username`             | `admin`                      | 默认管理员用户名                     |
| `auth.admin_password`             | `admin123`                   | 默认管理员密码                       |
| `auth.token_expire_hours`         | `24`                         | Token 有效期（小时）                 |

支持环境变量覆盖：`SERVER_HOST`、`SERVER_PORT`、`HEARTBEAT_TIMEOUT_SECS`、`LOG_LEVEL`、`JWT_SECRET`、`ADMIN_USERNAME`、`ADMIN_PASSWORD`、`TOKEN_EXPIRE_HOURS`

### actix-ak 客户端配置

| 字段                               | 默认值     | 说明                                        |
| ---------------------------------- | ---------- | ------------------------------------------- |
| `registry.enabled`                 | `false`    | 是否开启注册到注册中心                      |
| `registry.registry_url`            | `""`       | 注册中心地址                                |
| `registry.service_name`            | `actix-ak` | 服务名称                                    |
| `registry.register_host`           | `""`       | 注册用的主机地址（域名/IP），为空则自动获取 |
| `registry.heartbeat_interval_secs` | `10`       | 心跳发送间隔（秒）                          |
| `registry.registry_username`       | `""`       | 注册中心登录用户名                          |
| `registry.registry_password`       | `""`       | 注册中心登录密码                            |

支持环境变量覆盖：`REGISTRY_ENABLED`、`REGISTRY_URL`、`SERVICE_NAME`、`REGISTER_HOST`、`REGISTRY_USERNAME`、`REGISTRY_PASSWORD`

---

## API 接口

### 认证接口

Base URL: `http://localhost:8081/api/v1/auth`

> 认证接口**不需要** Token 即可访问。

#### POST /login

登录获取 JWT Token。

**请求体：**

```json
{
  "username": "admin",
  "password": "admin123"
}
```

**响应：**

```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 86400
  },
  "message": "Success",
  "timestamp": "2026-02-12T06:00:00+00:00"
}
```

#### POST /register

注册新用户。

**请求体：**

```json
{
  "username": "new_user",
  "password": "password123"
}
```

**响应：**

```json
{
  "success": true,
  "data": "注册成功",
  "message": "Success",
  "timestamp": "2026-02-12T06:00:00+00:00"
}
```

### 注册中心接口

Base URL: `http://localhost:8081/api/v1/registry`

> ⚠️ 以下接口需要在请求头携带 JWT Token：`Authorization: Bearer <token>`

#### POST /register

注册服务实例。

**请求体：**

```json
{
  "service_name": "my-service",
  "host": "127.0.0.1",
  "port": 9090,
  "metadata": {}
}
```

**响应：**

```json
{
  "success": true,
  "data": {
    "instance_id": "550e8400-e29b-41d4-a716-446655440000"
  },
  "message": "Success",
  "timestamp": "2026-02-11T22:00:00+08:00"
}
```

#### POST /heartbeat

发送心跳。

**请求体：**

```json
{
  "instance_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**响应：**

```json
{
  "success": true,
  "data": "心跳接收成功",
  "message": "Success",
  "timestamp": "2026-02-11T22:00:10+08:00"
}
```

#### GET /instances

获取所有已注册服务实例。

**请求示例：**

```bash
curl -H "Authorization: Bearer <token>" \
  http://localhost:8081/api/v1/registry/instances
```

**响应：**

```json
{
  "success": true,
  "data": [
    {
      "instance_id": "550e8400-e29b-41d4-a716-446655440000",
      "service_name": "actix-ak",
      "host": "0.0.0.0",
      "port": 8080,
      "status": "Up",
      "last_heartbeat": "2026-02-11T14:00:10Z",
      "registered_at": "2026-02-11T14:00:00Z",
      "metadata": {}
    }
  ],
  "message": "Success",
  "timestamp": "2026-02-11T22:00:15+08:00"
}
```

#### DELETE /deregister/{instance_id}

注销服务实例。

**请求示例：**

```bash
curl -X DELETE \
  -H "Authorization: Bearer <token>" \
  http://localhost:8081/api/v1/registry/deregister/550e8400-e29b-41d4-a716-446655440000
```

---

## 仪表板

浏览器访问 `http://localhost:8081/` 可查看 Web 仪表板：

- **登录页面**：首次访问需输入用户名和密码登录
- **统计卡片**：已注册服务数、在线数、离线数、服务类型数
- **实例列表**：服务名称、地址、状态、最近心跳时间、注册时间
- 每 5 秒自动刷新，Token 过期时自动跳回登录页

---

## 技术说明

### 项目结构

项目采用 **Cargo workspace** 管理，包含两个 crate：

| Crate             | 说明                           | 默认端口 |
| ----------------- | ------------------------------ | -------- |
| `actix-ak`        | 金融数据 API 服务 + 注册客户端 | 8080     |
| `registry-server` | 独立注册中心服务               | 8081     |

### 认证机制

- 用户密码使用 **bcrypt** 哈希存储
- 登录后签发 **JWT Token**（默认 24 小时有效）
- API 请求通过 `Authorization: Bearer <token>` header 认证
- 客户端 Token 过期时自动重新登录

### HTTP 客户端

注册客户端使用 `awc`（actix-web-client），与 actix-web 运行时原生兼容。启动时自动登录获取 JWT，后续请求携带 Token，Token 过期时透明重试。

### 配置加载

两个服务支持从 workspace 根目录或各自子目录运行，配置文件自动识别：

- **registry-server**：优先加载 `registry-server/config.json`，回退到当前目录 `config.json`
- **actix-ak**：优先加载当前目录 `config.json`，回退到 `../config.json`

环境变量始终优先于配置文件。

### Docker 部署

一键启动全部服务（registry-server 会先启动并健康检查通过后，actix-ak 自动注册）：

```bash
docker-compose up -d
```

单独构建：

```bash
# actix-ak
docker build -t actix-ak .

# registry-server
docker build -t registry-server -f registry-server/Dockerfile .
```

---

[返回首页](index.md)
