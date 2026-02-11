//! 注册中心仪表板页面

/// 返回仪表板 HTML 页面
pub fn dashboard_html() -> &'static str {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>注册中心 · 服务仪表板</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
    <style>
        *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --bg: #0f1117;
            --surface: #1a1d27;
            --surface-hover: #22263a;
            --border: #2a2e3e;
            --text: #e4e6ef;
            --text-dim: #8b8fa3;
            --accent: #6c63ff;
            --accent-glow: rgba(108, 99, 255, 0.25);
            --green: #22c55e;
            --green-bg: rgba(34, 197, 94, 0.12);
            --red: #ef4444;
            --red-bg: rgba(239, 68, 68, 0.12);
            --yellow: #f59e0b;
            --yellow-bg: rgba(245, 158, 11, 0.12);
            --radius: 12px;
        }
        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            background: var(--bg);
            color: var(--text);
            min-height: 100vh;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem 1.5rem;
        }

        /* Header */
        .header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-bottom: 2rem;
        }
        .header-left { display: flex; align-items: center; gap: 1rem; }
        .logo {
            width: 42px; height: 42px;
            background: linear-gradient(135deg, var(--accent), #a855f7);
            border-radius: 10px;
            display: flex; align-items: center; justify-content: center;
            font-size: 1.2rem; font-weight: 700; color: #fff;
            box-shadow: 0 4px 20px var(--accent-glow);
        }
        .header h1 { font-size: 1.5rem; font-weight: 700; letter-spacing: -0.02em; }
        .header h1 span { color: var(--text-dim); font-weight: 400; font-size: 0.9rem; margin-left: .5rem; }
        .refresh-info {
            display: flex; align-items: center; gap: .5rem;
            color: var(--text-dim); font-size: 0.8rem;
        }
        .pulse-dot {
            width: 8px; height: 8px; border-radius: 50%; background: var(--green);
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; box-shadow: 0 0 0 0 rgba(34, 197, 94, 0.5); }
            50% { opacity: .7; box-shadow: 0 0 0 6px rgba(34, 197, 94, 0); }
        }

        /* Stats Cards */
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }
        .stat-card {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 1.25rem 1.5rem;
            transition: border-color .2s;
        }
        .stat-card:hover { border-color: var(--accent); }
        .stat-label { font-size: 0.8rem; color: var(--text-dim); text-transform: uppercase; letter-spacing: .05em; margin-bottom: .5rem; }
        .stat-value { font-size: 2rem; font-weight: 700; }
        .stat-value.green { color: var(--green); }
        .stat-value.red { color: var(--red); }
        .stat-value.yellow { color: var(--yellow); }

        /* Table */
        .table-wrap {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: var(--radius);
            overflow: hidden;
        }
        .table-header {
            padding: 1rem 1.5rem;
            border-bottom: 1px solid var(--border);
            display: flex; align-items: center; justify-content: space-between;
        }
        .table-header h2 { font-size: 1rem; font-weight: 600; }
        table { width: 100%; border-collapse: collapse; }
        thead th {
            text-align: left;
            padding: .75rem 1.5rem;
            font-size: 0.75rem;
            font-weight: 600;
            color: var(--text-dim);
            text-transform: uppercase;
            letter-spacing: .05em;
            border-bottom: 1px solid var(--border);
            background: rgba(255,255,255,.02);
        }
        tbody tr {
            border-bottom: 1px solid var(--border);
            transition: background .15s;
        }
        tbody tr:last-child { border-bottom: none; }
        tbody tr:hover { background: var(--surface-hover); }
        tbody td {
            padding: .875rem 1.5rem;
            font-size: 0.875rem;
        }
        .mono { font-family: 'SF Mono', 'Fira Code', monospace; font-size: 0.8rem; color: var(--text-dim); }

        /* Status Badge */
        .badge {
            display: inline-flex; align-items: center; gap: .375rem;
            padding: .25rem .75rem;
            border-radius: 20px;
            font-size: 0.75rem; font-weight: 600;
        }
        .badge-up { background: var(--green-bg); color: var(--green); }
        .badge-down { background: var(--red-bg); color: var(--red); }
        .badge-dot { width: 6px; height: 6px; border-radius: 50%; }
        .badge-up .badge-dot { background: var(--green); }
        .badge-down .badge-dot { background: var(--red); }

        .service-name { font-weight: 600; }
        .endpoint { color: var(--accent); }

        /* Empty state */
        .empty-state {
            text-align: center;
            padding: 4rem 2rem;
            color: var(--text-dim);
        }
        .empty-icon { font-size: 3rem; margin-bottom: 1rem; opacity: .5; }
        .empty-state p { font-size: 0.9rem; }

        /* Responsive */
        @media (max-width: 768px) {
            .stats { grid-template-columns: 1fr 1fr; }
            .header { flex-direction: column; gap: 1rem; align-items: flex-start; }
            table { font-size: .8rem; }
            tbody td, thead th { padding: .625rem 1rem; }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="header-left">
                <div class="logo">R</div>
                <h1>服务注册中心<span>Dashboard</span></h1>
            </div>
            <div class="refresh-info">
                <div class="pulse-dot"></div>
                <span>每 5 秒自动刷新 · 更新于 <span id="update-time">--:--:--</span></span>
            </div>
        </div>

        <div class="stats">
            <div class="stat-card">
                <div class="stat-label">已注册服务</div>
                <div class="stat-value" id="total-count">0</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">在线</div>
                <div class="stat-value green" id="up-count">0</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">离线</div>
                <div class="stat-value red" id="down-count">0</div>
            </div>
            <div class="stat-card">
                <div class="stat-label">服务类型</div>
                <div class="stat-value yellow" id="type-count">0</div>
            </div>
        </div>

        <div class="table-wrap">
            <div class="table-header">
                <h2>服务实例列表</h2>
            </div>
            <div id="table-body">
                <div class="empty-state">
                    <div class="empty-icon">📡</div>
                    <p>暂无注册服务</p>
                </div>
            </div>
        </div>
    </div>

    <script>
        function formatTime(isoStr) {
            const d = new Date(isoStr);
            return d.toLocaleString('zh-CN', { hour12: false });
        }

        function timeSince(isoStr) {
            const secs = Math.floor((Date.now() - new Date(isoStr).getTime()) / 1000);
            if (secs < 60) return secs + '秒前';
            if (secs < 3600) return Math.floor(secs / 60) + '分钟前';
            return Math.floor(secs / 3600) + '小时前';
        }

        async function refresh() {
            try {
                const resp = await fetch('/api/v1/registry/instances');
                const json = await resp.json();
                const instances = json.data || [];

                // Stats
                const upCount = instances.filter(i => i.status === 'Up').length;
                const downCount = instances.filter(i => i.status === 'Down').length;
                const types = new Set(instances.map(i => i.service_name));

                document.getElementById('total-count').textContent = instances.length;
                document.getElementById('up-count').textContent = upCount;
                document.getElementById('down-count').textContent = downCount;
                document.getElementById('type-count').textContent = types.size;
                document.getElementById('update-time').textContent =
                    new Date().toLocaleTimeString('zh-CN', { hour12: false });

                // Table
                const container = document.getElementById('table-body');
                if (instances.length === 0) {
                    container.innerHTML = `
                        <div class="empty-state">
                            <div class="empty-icon">📡</div>
                            <p>暂无注册服务，等待服务注册…</p>
                        </div>`;
                    return;
                }

                let html = `<table><thead><tr>
                    <th>服务名称</th><th>地址</th><th>状态</th>
                    <th>最近心跳</th><th>注册时间</th><th>实例 ID</th>
                </tr></thead><tbody>`;

                for (const inst of instances) {
                    const isUp = inst.status === 'Up';
                    const badgeClass = isUp ? 'badge-up' : 'badge-down';
                    const statusText = isUp ? '在线' : '离线';
                    html += `<tr>
                        <td class="service-name">${inst.service_name}</td>
                        <td class="endpoint">${inst.host}:${inst.port}</td>
                        <td><span class="badge ${badgeClass}"><span class="badge-dot"></span>${statusText}</span></td>
                        <td>${timeSince(inst.last_heartbeat)}</td>
                        <td>${formatTime(inst.registered_at)}</td>
                        <td class="mono">${inst.instance_id.substring(0, 8)}…</td>
                    </tr>`;
                }
                html += '</tbody></table>';
                container.innerHTML = html;
            } catch (e) {
                console.error('刷新失败:', e);
            }
        }

        refresh();
        setInterval(refresh, 5000);
    </script>
</body>
</html>"#
}
