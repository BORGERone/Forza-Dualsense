<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Forza Horizon中管理DualSense手柄的现代界面</i>
</p>

---

## 描述

Forza DualSense是一个用于在Forza Horizon中管理PlayStation 5 DualSense手柄的应用程序。该程序提供扩展的手柄自定义功能，包括自适应扳机、触觉反馈和其他触觉反馈功能。

---

## 依赖项

| 依赖项 | 描述 |
|--------|------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | 后端脚本所需 |

---

## 安装

### 要求
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** 或更新版本
- DualSense手柄 (PS5)

### 启动
1. 将DualSense手柄连接到计算机
2. 启动 `forza-dualsense.exe`
3. 在应用程序界面中配置参数
4. 指定游戏.exe路径（可选，自动启动需要）

## 游戏内设置

打开Forza Horizon → **设置 → HUD和游戏玩法**，向下滚动:

| 设置 | 值 |
|------|-----|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## 功能

### 手柄控制
- 油门和刹车踏板的自适应扳机配置
- 触觉反馈配置
- 摇杆死区设置
- ABS（防抱死系统）支持
- 启动脉冲用于连接确认

### 界面
- 具有直观控制的现代暗色设计
- 实时显示手柄连接状态
- 游戏通信的UDP参数设置
- 可从应用程序直接启动Forza Horizon（指定.exe路径时）

---

## 使用

### 初始设置
1. 启动应用程序
2. 确保手柄已连接
3. 配置游戏通信的UDP参数
4. 调整扳机和摇杆灵敏度
5. 通过应用程序按钮启动Forza Horizon

---

## 故障排除

### 手柄未连接
- 确保手柄通过USB或蓝牙连接
- 使用第三方驱动程序（如DS4W）时，**<u>完全</u>**禁用Steam Input
- 检查Python是否已安装并正常工作
- 禁用杀毒软件
- 禁用防火墙
- 检查默认端口（5300）的可用性，必要时更改
- 检查%LOCALAPPDATA%\Forza DualSense\backend文件夹是否存在
- 重新启动应用程序

---

## 配置

后端文件自动提取到:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

应用程序设置保存在配置文件中。

---

## 开发者

### 开发依赖项

| 依赖项 | 描述 |
|--------|------|
| **[Node.js 18+](https://nodejs.org/)** | 前端开发和构建所需 |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Tauri后端编译所需 |
| **[Python 3.12](https://www.python.org/downloads/)** | 后端脚本所需 |

### 安装依赖项

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# 或使用uv（推荐）
uv sync
```

### 开发
```powershell
cd tauri-app
npm run tauri dev
```

### 发布构建
```powershell
cd tauri-app
npm run tauri build
```

构建后，exe文件位于:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## 项目架构

```
tauri-app/
├── src/                 # Tauri前端 (TypeScript/HTML)
├── src-tauri/           # Tauri Rust后端
│   ├── backend/         # Python IPC后端（嵌入exe）
│   │   ├── modules/     # Python模块
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rust源代码
│   │   └── main.rs      # 包含IPC逻辑的主文件
│   ├── build.rs         # 构建脚本
│   └── tauri.conf.json  # Tauri配置
├── package.json         # Node.js依赖项
└── build.bat            # Windows构建脚本
```

### 应用程序架构
- **Tauri前端** - TypeScript/HTML GUI界面
- **Tauri Rust后端** - 进程管理和IPC通信
- **Python IPC后端** - 通过stdin/stdout处理手柄命令
- **嵌入资源** - 通过rust-embed嵌入exe的后端文件

---

## IPC协议

Python后端通过stdin/stdout使用JSON与Rust后端通信:

### 请求（到stdin）
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### 响应（从stdout）
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---
