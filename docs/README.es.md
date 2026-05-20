<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Interfaz moderna para gestionar el mando DualSense en Forza Horizon</i>
</p>

---

## Descripcion

Forza DualSense es una aplicacion para gestionar el mando PlayStation 5 DualSense en Forza Horizon. El programa proporciona capacidades de personalizacion extendidas del controlador, incluyendo gatillos adaptativos, retroalimentacion haptica y otras caracteristicas de retroalimentacion haptica.

---

## Dependencias

| Dependencia | Descripcion |
|------------|-------------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Requerido para scripts backend |

---

## Instalacion

### Requisitos
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** o mas reciente
- Mando DualSense (PS5)

### Inicio
1. Conecte el mando DualSense a su computadora
2. Inicie `forza-dualsense.exe`
3. Configure los parametros en la interfaz de la aplicacion
4. Especifique la ruta al .exe del juego (opcional, necesario para inicio automatico)

## Configuracion en el juego

Abra Forza Horizon → **Configuracion → HUD y Gameplay**, desplacese hacia abajo:

| Configuracion | Valor |
|--------------|-------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Caracteristicas

### Control del mando
- Configuracion de gatillos adaptativos para pedales de aceleracion y freno
- Configuracion de retroalimentacion haptica
- Ajustes de zona muerta para joysticks
- Soporte de ABS (sistema antibloqueo)
- Impulso de inicio para confirmacion de conexion

### Interfaz
- Designo oscuro moderno con controles intuitivos
- Visualizacion en tiempo real del estado de conexion del mando
- Parametros UDP para comunicacion con el juego
- Posibilidad de iniciar Forza Horizon directamente desde la aplicacion (cuando se especifica la ruta del .exe)

---

## Uso

### Configuracion inicial
1. Inicie la aplicacion
2. Asegurese de que el mando este conectado
3. Configure los parametros UDP para comunicacion con el juego
4. Ajuste la sensibilidad de gatillos y joysticks
5. Inicie Forza Horizon a traves del boton de la aplicacion

---

## Solucion de problemas

### El mando no se conecta
- Asegurese de que el mando este conectado via USB o Bluetooth
- Al usar controladores de terceros (como DS4W), **<u>COMPLETAMENTE</u>** desactive Steam Input
- Verifique que Python este instalado y funcionando
- Desactive el antivirus
- Desactive el Firewall
- Verifique la disponibilidad del puerto predeterminado (5300), cambielo si es necesario
- Verifique la presencia de la carpeta %LOCALAPPDATA%\Forza DualSense\backend
- Reinicie la aplicacion

---

## Configuracion

Los archivos backend se extraen automaticamente a:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

La configuracion de la aplicacion se guarda en el archivo de configuracion.

---

## Para desarrolladores

### Dependencias de desarrollo

| Dependencia | Descripcion |
|------------|-------------|
| **[Node.js 18+](https://nodejs.org/)** | Requerido para desarrollo y construccion frontend |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Requerido para compilacion del backend Tauri |
| **[Python 3.12](https://www.python.org/downloads/)** | Requerido para scripts backend |

### Instalacion de dependencias

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# o usando uv (recomendado)
uv sync
```

### Desarrollo
```powershell
cd tauri-app
npm run tauri dev
```

### Construccion de version
```powershell
cd tauri-app
npm run tauri build
```

Despues de la construccion, el archivo exe se encontrara en:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Arquitectura del proyecto

```
tauri-app/
├── src/                 # Frontend Tauri (TypeScript/HTML)
├── src-tauri/           # Backend Tauri Rust
│   ├── backend/         # Backend IPC Python (incrustado en exe)
│   │   ├── modules/     # Modulos Python
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Codigo fuente Rust
│   │   └── main.rs      # Archivo principal con logica IPC
│   ├── build.rs         # Script de construccion
│   └── tauri.conf.json  # Configuracion Tauri
├── package.json         # Dependencias Node.js
└── build.bat            # Script de construccion Windows
```

### Arquitectura de la aplicacion
- **Tauri Frontend** - Interfaz GUI en TypeScript/HTML
- **Tauri Rust Backend** - Gestion de procesos y comunicacion IPC
- **Python IPC Backend** - Procesamiento de comandos del mando via stdin/stdout
- **Recursos incrustados** - Archivos backend incrustados en exe via rust-embed

---

## Protocolo IPC

El backend Python se comunica con el backend Rust via JSON sobre stdin/stdout:

### Solicitudes (a stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Respuestas (de stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---

## Agradecimientos

Este proyecto utiliza el codigo backend Python de [Forza Horizon DualSense Python](https://github.com/HamzaYslmn/Forza-Horizon-DualSense-Python) por HamzaYslmn, el cual esta licenciado bajo AGPL v3. El proyecto original proporciona la funcionalidad principal para la comunicacion del mando DualSense con Forza Horizon.

---
