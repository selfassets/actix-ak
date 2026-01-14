# 健康检查

检测服务是否正常运行。

## GET /health

健康检查接口，无需认证。

### 请求示例

```bash
curl -X GET "https://byteappua-actix-ak.zeabur.app/api/v1/health"
```

### 响应示例

```json
{
  "success": true,
  "data": "服务运行正常",
  "error": null
}
```

---

[返回首页](index.md)
