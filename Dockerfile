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
COPY src ./src
# workspace 成员占位（仅需 Cargo.toml 满足解析）
COPY registry-server/Cargo.toml ./registry-server/Cargo.toml
RUN mkdir -p registry-server/src && echo "fn main() {}" > registry-server/src/main.rs

# 构建 actix-ak
RUN cargo build --release -p actix-ak

# 运行阶段
FROM debian:bookworm-slim

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/actix-ak /app/actix-ak

# 复制默认配置文件
COPY config.json /app/config.json

# 设置环境变量
ENV RUST_LOG=info

# 支持通过环境变量覆盖配置（优先级高于 config.json）
# docker run -e API_KEY=your_key -e LOG_LEVEL=debug ...
ENV API_KEY=""
ENV SERVER_HOST="0.0.0.0"
ENV PORT=8080
ENV LOG_LEVEL="info"
ENV REGISTRY_ENABLED="false"
ENV REGISTRY_URL=""


EXPOSE 8080

# 支持通过挂载覆盖配置文件: -v /path/to/config.json:/app/config.json
# 也可以通过环境变量覆盖: -e API_KEY=your_key
CMD ["./actix-ak"]
