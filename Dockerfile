# 构建阶段
FROM rust:1.87-slim-bookworm AS builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 构建项目
RUN cargo build --release

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

EXPOSE 8080

# 支持通过挂载覆盖配置文件: -v /path/to/config.json:/app/config.json
CMD ["./actix-ak"]
