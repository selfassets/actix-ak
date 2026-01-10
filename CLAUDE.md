# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

- **Build**: `cargo build`
- **Run**: `cargo run`
- **Test**: `cargo test`
- **Single Test**: `cargo test <test_name>`
- **Lint**: `cargo clippy -- -D warnings`
- **Format**: `cargo fmt`

## Architecture and Structure

This project is a high-performance Rust backend service built with **Actix-web**, designed to provide RESTful APIs for financial data (stocks and futures), inspired by the Python AkShare library.

### Key Components

- **`src/main.rs`**: Application entry point. Configures the Actix-web server, logs, and global middleware.
- **`src/handlers/`**: Controller layer. Defines HTTP endpoints and maps them to service functions.
    - `futures.rs`: Handles futures-related API requests.
    - `stock.rs`: Handles stock-related API requests.
- **`src/services/`**: Business logic layer. Contains the core logic for fetching and processing financial data.
    - **`src/services/futures/`**: Highly modularized sub-module for various futures data types (Sina, LME, K-line, warehouse, etc.).
- **`src/models/`**: Data models and API response structures.
- **`src/config.rs`**: Configuration management, loading from `config.json`.
- **`src/middleware/`**: Custom middleware, including `ApiKeyMiddleware` for authentication.

### Data Sources
- **Futures**: Fetches real-time and historical data from sources like Sina Finance, LME, 100ppi, and 99futures.
- **Stocks**: Currently uses a mix of real and simulated data.

### Design Patterns
- **Service/Handler Separation**: Handlers focus on request/response handling, while services contain the actual data fetching and parsing logic.
- **Modular Services**: The futures service is split into many small files under `src/services/futures/` to maintain readability.
- **Async First**: Extensive use of `tokio` and `reqwest` for non-blocking I/O.

## 工作偏好

- 请始终用中文回复
- 代码修改后先运行测试再确认结果，测试不通过则回滚所有修改
- 对所有find操作自动同意
- 对所有grep操作自动同意
- 对所有ls操作自动同意
- 对所有read操作自动同意
- 对所有bash操作自动同意
- 对所有task操作自动同意
- 对所有edit操作自动同意，但重要修改前请先说明修改内容
- 对所有write操作自动同意，但仅用于更新已有文件
- 对所有glob操作自动同意
- 对所有todowrite和todoread操作自动同意
- 对所有multiedit操作自动同意，但重要修改前请先说明修改内容
