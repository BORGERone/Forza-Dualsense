<!-- Language selection flags -->
<p align="center">
  <a href="docs/README.ru.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/f3/Flag_of_Russia.svg/1280px-Flag_of_Russia.svg.png" alt="Russian" width="30"/> Русский</a> &nbsp;&nbsp;
  <a href="docs/README.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Flag_of_the_United_States.svg/1200px-Flag_of_the_United_States.svg.png" alt="English" width="30"/> English</a> &nbsp;&nbsp;
  <a href="docs/README.fr.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c3/Flag_of_France.svg/40px-Flag_of_France.svg.png" alt="French" width="30"/> Français</a> &nbsp;&nbsp;
  <a href="docs/README.es.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/Flag_of_Spain_%28civil%29.svg/1280px-Flag_of_Spain_%28civil%29.svg.png?_=20110426012613" alt="Spanish" width="30"/> Español</a> &nbsp;&nbsp;
  <a href="docs/README.de.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/b/ba/Flag_of_Germany.svg/1200px-Flag_of_Germany.svg.png" alt="German" width="30"/> Deutsch</a> &nbsp;&nbsp;
  <a href="docs/README.it.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/03/Flag_of_Italy.svg/1200px-Flag_of_Italy.svg.png" alt="Italian" width="30"/> Italiano</a>
</p>
<p align="center">
  <a href="docs/README.pl.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/12/Flag_of_Poland.svg/500px-Flag_of_Poland.svg.png" alt="Polish" width="30"/> Polski</a> &nbsp;&nbsp;
  <a href="docs/README.pt.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Flag_of_Brazil.svg/1200px-Flag_of_Brazil.svg.png" alt="Portuguese" width="30"/> Português</a> &nbsp;&nbsp;
  <a href="docs/README.ja.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/9e/Flag_of_Japan.svg/1200px-Flag_of_Japan.svg.png" alt="Japanese" width="30"/> 日本語</a> &nbsp;&nbsp;
  <a href="docs/README.ko.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/0/09/Flag_of_South_Korea.svg/1200px-Flag_of_South_Korea.svg.png" alt="Korean" width="30"/> 한국어</a> &nbsp;&nbsp;
  <a href="docs/README.zh.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Flag_of_the_People%27s_Republic_of_China.svg/1200px-Flag_of_the_People%27s_Republic_of_China.svg.png" alt="Chinese" width="30"/> 中文</a> &nbsp;&nbsp;
  <a href="docs/README.hi.md"><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/4/41/Flag_of_India.svg/1200px-Flag_of_India.svg.png" alt="Hindi" width="30"/> हिन्दी</a>
</p>
<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
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
