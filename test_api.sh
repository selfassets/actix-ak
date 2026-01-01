#!/bin/bash

echo "=== 测试 AkShare Backend API ==="
echo ""

echo "1. 健康检查"
curl -s "http://127.0.0.1:8080/api/v1/health" | jq .
echo ""

echo "2. 获取交易所列表"
curl -s "http://127.0.0.1:8080/api/v1/futures/exchanges" | jq .
echo ""

echo "3. 获取大商所期货列表"
curl -s "http://127.0.0.1:8080/api/v1/futures?exchange=DCE&limit=3" | jq .
echo ""

echo "4. 获取单个期货合约信息 (CU2405)"
curl -s "http://127.0.0.1:8080/api/v1/futures/CU2405" | jq .
echo ""

echo "5. 批量获取期货数据"
curl -s -X POST "http://127.0.0.1:8080/api/v1/futures/batch" \
  -H "Content-Type: application/json" \
  -d '["CU2405", "AL2405", "ZN2405"]' | jq .
echo ""

echo "6. 获取股票列表"
curl -s "http://127.0.0.1:8080/api/v1/stocks?limit=3" | jq .
echo ""
