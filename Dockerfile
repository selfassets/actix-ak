# 构建阶段
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 先复制依赖文件，利用 Docker 缓存
COPY Cargo.toml Cargo.lock ./

# 创建虚拟 src 目录用于预编译依赖
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src target/release/deps/akshare*

# 复制实际源码并构建
COPY src ./src
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
COPY --from=builder /app/target/release/akshare-backend /app/akshare-backend

# 设置环境变量
ENV RUST_LOG=info

EXPOSE 8080

CMD ["./akshare-backend"]
