# 前端项目

基于 Next.js 15 + Shadcn UI 构建的现代 Web 前端项目，提供期货和股票数据的可视化展示。

## 技术栈

- **框架**: [Next.js 15](https://nextjs.org/) (App Router)
- **UI 组件库**: [Shadcn UI](https://ui.shadcn.com/) (基于 Radix UI + Tailwind CSS)
- **图表库**: [Lightweight Charts](https://tradingview.github.io/lightweight-charts/) (高性能金融图表)
- **样式**: Tailwind CSS 4
- **包管理器**: pnpm

## 目录结构

```
web/
├── app/                # Next.js App Router 页面
│   ├── futures/        # 期货相关页面
│   │   ├── [symbol]/   # 详情页
│   │   ├── realtime/   # 实时行情
│   │   └── symbols/    # 品种列表
│   └── page.tsx        # 首页
├── components/         # UI 组件
│   ├── ui/             # Shadcn 基础组件
│   └── ...             # 业务组件
├── lib/                # 工具函数
│   ├── api.ts          # API 接口封装
│   └── utils.ts        # 通用工具
└── public/             # 静态资源
```

## 快速开始

### 1. 安装依赖

```bash
cd web
pnpm install
```

### 2. 配置环境变量

复制 `.env.example` 到 `.env.local` 并修改配置：

```bash
cp .env.example .env.local
```

`.env.local` 内容示例：

```env
NEXT_PUBLIC_API_BASE_URL=http://localhost:8080/api/v1
NEXT_PUBLIC_API_TOKEN=12345678
```

### 3. 启动开发服务器

```bash
pnpm dev
```

访问 <http://localhost:3000> 查看效果。

## 主要功能

### 期货行情

- **实时看板**: 展示主力合约的实时价格、涨跌幅。
- **K 线图表**: 支持日 K、分钟 K 线，集成 TradingView 轻量级图表。
- **持仓分析**: 可视化展示多空持仓排名。
- **仓单日报**: 查看各交易所仓单数据。

### 股票行情

- **列表展示**: A 股实时行情列表。
- **个股详情**: 实时盘口数据和历史走势。

## 开发指南

### 添加新组件

使用 Shadcn CLI 添加组件：

```bash
pnpm dlx shadcn@latest add [component-name]
```

例如添加按钮组件：

```bash
pnpm dlx shadcn@latest add button
```

### 接口调用

所有后端 API 调用封装在 `lib/api.ts` 中，使用 fetch 进行请求。

示例：

```typescript
import { getFuturesInfo } from "@/lib/api";

const data = await getFuturesInfo("RB2510");
```
