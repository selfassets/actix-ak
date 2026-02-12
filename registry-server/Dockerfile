# 构建阶段
FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 workspace 文件
COPY Cargo.toml Cargo.lock ./
# workspace 成员占位（仅需 Cargo.toml 满足解析）
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
COPY registry-server ./registry-server

# 构建 registry-server
RUN cargo build --release -p registry-server

# 运行阶段
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/registry-server /app/registry-server

# 复制默认配置文件
COPY registry-server/config.json /app/config.json

# 设置环境变量
ENV RUST_LOG=info
ENV SERVER_HOST="0.0.0.0"
ENV SERVER_PORT=8081
ENV HEARTBEAT_TIMEOUT_SECS=30
ENV LOG_LEVEL="info"
ENV JWT_SECRET="registry-server-secret-key"
ENV ADMIN_USERNAME="admin"
ENV ADMIN_PASSWORD="admin123"
ENV TOKEN_EXPIRE_HOURS=24

EXPOSE 8081

# 支持通过挂载覆盖配置文件: -v /path/to/config.json:/app/config.json
# 也可以通过环境变量覆盖: -e SERVER_PORT=8081 -e HEARTBEAT_TIMEOUT_SECS=60
CMD ["./registry-server"]
