<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Modernes Interface zur Verwaltung des DualSense-Gamepads in Forza Horizon</i>
</p>

---

## Beschreibung

Forza DualSense ist eine Anwendung zur Verwaltung des PlayStation 5 DualSense-Gamepads in Forza Horizon. Das Programm bietet erweiterte Anpassungsmoglichkeiten fur den Controller, einschließlich adaptiver Trigger, haptisches Feedback und weitere haptische Feedback-Funktionen.

---

## Abhangigkeiten

| Abhangigkeit | Beschreibung |
|--------------|--------------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Erforderlich fur Backend-Skripte |

---

## Installation

### Anforderungen
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** oder neuer
- DualSense-Gamepad (PS5)

### Start
1. Verbinden Sie das DualSense-Gamepad mit Ihrem Computer
2. Starten Sie `forza-dualsense.exe`
3. Konfigurieren Sie die Parameter in der Anwendungsoberflache
4. Geben Sie den Pfad zur .exe des Spiels an (optional, erforderlich fur automatischen Start)

## Spiel-Einrichtung

Offnen Sie Forza Horizon → **Einstellungen → HUD & Gameplay**, scrollen Sie nach unten:

| Einstellung | Wert |
|-------------|------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Funktionen

### Gamepad-Steuerung
- Adaptive Trigger-Konfiguration fur Gas- und Bremspedale
- Haptisches Feedback-Konfiguration
- Deadzone-Einstellungen fur Joysticks
- ABS-Unterstutzung (Antiblockiersystem)
- Startimpuls fur Verbindungsbestatigung

### Benutzeroberflache
- Modernes dunkles Design mit intuitiven Steuerelementen
- Echtzeit-Anzeige des Gamepad-Verbindungsstatus
- UDP-Parameter-Einstellungen fur Spielkommunikation
- Moglichkeit, Forza Horizon direkt aus der Anwendung zu starten (wenn .exe-Pfad angegeben ist)

---

## Verwendung

### Ersteinrichtung
1. Starten Sie die Anwendung
2. Stellen Sie sicher, dass das Gamepad verbunden ist
3. Konfigurieren Sie die UDP-Parameter fur die Spielkommunikation
4. Passen Sie die Empfindlichkeit von Triggern und Joysticks an
5. Starten Sie Forza Horizon uber den Anwendungsknopf

---

## Fehlerbehebung

### Gamepad wird nicht verbunden
- Stellen Sie sicher, dass das Gamepad uber USB oder Bluetooth verbunden ist
- Bei Verwendung von Drittanbieter-Treibern (wie DS4W) **<u>VOLLSTANDIG</u>** Steam Input deaktivieren
- Uberprufen Sie, ob Python installiert ist und funktioniert
- Deaktivieren Sie Antivirensoftware
- Deaktivieren Sie Firewall
- Uberprufen Sie die Verfugbarkeit des Standardports (5300), andern Sie ihn bei Bedarf
- Uberprufen Sie das Vorhandensein des Ordners %LOCALAPPDATA%\Forza DualSense\backend
- Starten Sie die Anwendung neu

---

## Konfiguration

Backend-Dateien werden automatisch extrahiert nach:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

Anwendungseinstellungen werden in der Konfigurationsdatei gespeichert.

---

## Fur Entwickler

### Entwicklungsabhangigkeiten

| Abhangigkeit | Beschreibung |
|--------------|--------------|
| **[Node.js 18+](https://nodejs.org/)** | Erforderlich fur Frontend-Entwicklung und Build |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Erforderlich fur Tauri-Backend-Kompilierung |
| **[Python 3.12](https://www.python.org/downloads/)** | Erforderlich fur Backend-Skripte |

### Installation von Abhangigkeiten

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# oder mit uv (empfohlen)
uv sync
```

### Entwicklung
```powershell
cd tauri-app
npm run tauri dev
```

### Release-Build
```powershell
cd tauri-app
npm run tauri build
```

Nach dem Build befindet sich die exe-Datei unter:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Projektarchitektur

```
tauri-app/
├── src/                 # Tauri Frontend (TypeScript/HTML)
├── src-tauri/           # Tauri Rust Backend
│   ├── backend/         # Python IPC Backend (in exe eingebettet)
│   │   ├── modules/     # Python-Module
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rust-Quellcode
│   │   └── main.rs      # Hauptdatei mit IPC-Logik
│   ├── build.rs         # Build-Skript
│   └── tauri.conf.json  # Tauri-Konfiguration
├── package.json         # Node.js-Abhangigkeiten
└── build.bat            # Windows-Build-Skript
```

### Anwendungsarchitektur
- **Tauri Frontend** - GUI-Schnittstelle in TypeScript/HTML
- **Tauri Rust Backend** - Prozessverwaltung und IPC-Kommunikation
- **Python IPC Backend** - Gamepad-Befehlsverarbeitung uber stdin/stdout
- **Eingebettete Ressourcen** - Backend-Dateien uber rust-embed in exe eingebettet

---

## IPC-Protokoll

Python-Backend kommuniziert mit Rust-Backend uber JSON uber stdin/stdout:

### Anfragen (an stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Antworten (von stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---

## Danksagungen

Dieses Projekt verwendet Python-Backend-Code aus [Forza Horizon DualSense Python](https://github.com/HamzaYslmn/Forza-Horizon-DualSense-Python) von HamzaYslmn, der unter AGPL v3 lizenziert ist. Das ursprungliche Projekt bietet die Kernfunktionalitat fur die Kommunikation des DualSense-Gamepads mit Forza Horizon.

---
