# Forza DualSense

<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/a831992c-0474-497a-bf23-7f85b4de5fe6" />
</p>
<p align="center">
  <i>Interfaccia moderna per gestire il gamepad DualSense in Forza Horizon</i>
</p>

---

## Descrizione

Forza DualSense e un'applicazione per gestire il gamepad PlayStation 5 DualSense in Forza Horizon. Il programma fornisce capacit di personalizzazione estese per il controller, inclusi trigger adattivi, feedback aptico e altre funzionalit di feedback aptico.

---

## Dipendenze

| Dipendenza | Descrizione |
|------------|-------------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Richiesto per gli script backend |

---

## Installazione

### Requisiti
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** o piu recente
- Gamepad DualSense (PS5)

### Avvio
1. Collega il gamepad DualSense al tuo computer
2. Avvia `forza-dualsense.exe`
3. Configura i parametri nell'interfaccia dell'applicazione
4. Specifica il percorso del .exe del gioco (opzionale, necessario per l'avvio automatico)

## Configurazione nel gioco

Apri Forza Horizon → **Impostazioni → HUD e Gameplay**, scorri verso il basso:

| Impostazione | Valore |
|--------------|-------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Funzionalit

### Controllo del gamepad
- Configurazione trigger adattivi per pedali acceleratore e freno
- Configurazione feedback aptico
- Impostazioni deadzone per joystick
- Supporto ABS (sistema antibloccaggio)
- Impulso di avvio per conferma connessione

### Interfaccia
- Design scuro moderno con controlli intuitivi
- Visualizzazione in tempo reale dello stato di connessione del gamepad
- Impostazioni parametri UDP per comunicazione con il gioco
- Possibilit di avviare Forza Horizon direttamente dall'applicazione (quando specificato il percorso del .exe)

---

## Utilizzo

### Configurazione iniziale
1. Avvia l'applicazione
2. Assicurati che il gamepad sia collegato
3. Configura i parametri UDP per la comunicazione con il gioco
4. Regola la sensibilit dei trigger e joystick
5. Avvia Forza Horizon tramite il pulsante dell'applicazione

---

## Risoluzione problemi

### Il gamepad non si collega
- Assicurati che il gamepad sia collegato via USB o Bluetooth
- Quando usi driver di terze parti (come DS4W), **<u>COMPLETAMENTE</u>** disabilita Steam Input
- Verifica che Python sia installato e funzionante
- Disabilita l'antivirus
- Disabilita il Firewall
- Verifica la disponibilit del porta predefinita (5300), cambiala se necessario
- Verifica la presenza della cartella %LOCALAPPDATA%\Forza DualSense\backend
- Riavvia l'applicazione

---

## Configurazione

I file backend vengono estratti automaticamente in:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

Le impostazioni dell'applicazione vengono salvate nel file di configurazione.

---

## Per sviluppatori

### Dipendenze di sviluppo

| Dipendenza | Descrizione |
|------------|-------------|
| **[Node.js 18+](https://nodejs.org/)** | Richiesto per sviluppo e build frontend |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Richiesto per compilazione backend Tauri |
| **[Python 3.12](https://www.python.org/downloads/)** | Richiesto per gli script backend |

### Installazione dipendenze

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# o usando uv (raccomandato)
uv sync
```

### Sviluppo
```powershell
cd tauri-app
npm run tauri dev
```

### Build release
```powershell
cd tauri-app
npm run tauri build
```

Dopo la build, il file exe si trover in:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Architettura progetto

```
tauri-app/
├── src/                 # Frontend Tauri (TypeScript/HTML)
├── src-tauri/           # Backend Tauri Rust
│   ├── backend/         # Backend IPC Python (incorporato in exe)
│   │   ├── modules/     # Moduli Python
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Codice sorgente Rust
│   │   └── main.rs      # File principale con logica IPC
│   ├── build.rs         # Script build
│   └── tauri.conf.json  # Configurazione Tauri
├── package.json         # Dipendenze Node.js
└── build.bat            # Script build Windows
```

### Architettura applicazione
- **Tauri Frontend** - Interfaccia GUI in TypeScript/HTML
- **Tauri Rust Backend** - Gestione processi e comunicazione IPC
- **Python IPC Backend** - Elaborazione comandi gamepad via stdin/stdout
- **Risorse incorporate** - File backend incorporati in exe via rust-embed

---

## Protocollo IPC

Il backend Python comunica con il backend Rust tramite JSON su stdin/stdout:

### Richieste (a stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Risposte (da stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---
