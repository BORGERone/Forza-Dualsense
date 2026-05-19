<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Modern interface for managing DualSense gamepad in Forza Horizon</i>
</p>

---

## Description

Forza DualSense is an application for managing the PlayStation 5 DualSense gamepad in Forza Horizon. The program provides extended controller customization capabilities, including adaptive triggers, haptic feedback, and other haptic feedback features.

---

## Dependencies

| Dependency | Description |
|------------|-------------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Required for backend scripts |

---

## Installation

### Requirements
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** or newer
- DualSense gamepad (PS5)

### Launch
1. Connect DualSense gamepad to your computer
2. Launch `forza-dualsense.exe`
3. Configure parameters in the application interface
4. Specify path to game .exe (optional, needed for auto-launch)

## In-Game Setup

Open Forza Horizon → **Settings → HUD & Gameplay**, scroll to the bottom:

| Setting | Value |
|---------|-------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Features

### Gamepad Control
- Adaptive trigger configuration for gas and brake pedals
- Haptic feedback configuration
- Deadzone settings for joysticks
- ABS (anti-lock braking system) support
- Startup pulse for connection confirmation

### Interface
- Modern dark design with intuitive controls
- Real-time gamepad connection status display
- UDP parameter settings for game communication
- Ability to launch Forza Horizon directly from the application (when .exe path is specified)

---

## Usage

### Initial Setup
1. Launch the application
2. Ensure the gamepad is connected
3. Configure UDP parameters for game communication
4. Adjust trigger and joystick sensitivity
5. Launch Forza Horizon via the application button

---

## Troubleshooting

### Gamepad not connecting
- Ensure the gamepad is connected via USB or Bluetooth
- When using third-party drivers (like DS4W), **<u>FULLY</u>** disable Steam Input
- Check that Python is installed and working
- Disable antivirus
- Disable Firewall
- Check default port availability (5300), change if necessary
- Check for folder %LOCALAPPDATA%\Forza DualSense\backend
- Restart the application

---

## Configuration

Backend files are automatically extracted to:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

Application settings are saved in the configuration file.

---

## For Developers

### Development Dependencies

| Dependency | Description |
|------------|-------------|
| **[Node.js 18+](https://nodejs.org/)** | Required for frontend development and build |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Required for Tauri backend compilation |
| **[Python 3.12](https://www.python.org/downloads/)** | Required for backend scripts |

### Installing Dependencies

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# or using uv (recommended)
uv sync
```

### Development
```powershell
cd tauri-app
npm run tauri dev
```

### Release Build
```powershell
cd tauri-app
npm run tauri build
```

After build, the exe file will be located at:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Project Architecture

```
tauri-app/
├── src/                 # Tauri frontend (TypeScript/HTML)
├── src-tauri/           # Tauri Rust backend
│   ├── backend/         # Python IPC backend (embedded in exe)
│   │   ├── modules/     # Python modules
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rust source code
│   │   └── main.rs      # Main file with IPC logic
│   ├── build.rs         # Build script
│   └── tauri.conf.json  # Tauri configuration
├── package.json         # Node.js dependencies
└── build.bat            # Windows build script
```

### Application Architecture
- **Tauri Frontend** - GUI interface in TypeScript/HTML
- **Tauri Rust Backend** - Process management and IPC communication
- **Python IPC Backend** - Gamepad command processing via stdin/stdout
- **Embedded Resources** - Backend files embedded in exe via rust-embed

---

## IPC Protocol

Python backend communicates with Rust backend via JSON over stdin/stdout:

### Requests (to stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Responses (from stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---
