---
layout: default
title: 首页
nav_order: 1
---

# AkShare Backend API 文档

基于 Rust Actix-web 实现的期货和股票数据 RESTful API 服务。

## 基础信息

- **Base URL**: `https://byteappua-actix-ak.zeabur.app/api/v1`
- **认证方式**: Bearer Token
- **请求头**: `Authorization: Bearer <token>`

## 目录

- [健康检查](health)
- [期货接口](futures)
- [股票接口](stocks)

---

## 快速开始

### 认证示例

```bash
curl -X GET "https://byteappua-actix-ak.zeabur.app/api/v1/health" \
  -H "Authorization: Bearer 12345678"
```

### 响应格式

所有接口返回统一的 JSON 格式：

```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

错误响应：

```json
{
  "success": false,
  "data": null,
  "error": "错误信息"
}
```
