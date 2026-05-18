# Forza DualSense

![](https://github.com/BORGERone/ShadowLibrary/blob/main/docs/logo.png)

<p align="center">
  <i>Interface moderne pour la gestion de la manette DualSense dans Forza Horizon</i>
</p>

---

## Description

Forza DualSense est une application pour gérer la manette PlayStation 5 DualSense dans Forza Horizon. Le programme offre des capacites de personnalisation etendues du controleur, y compris les gachettes adaptatives, le retour haptique et d'autres fonctionnalites de retour haptique.

---

## Dependances

| Dependance | Description |
|------------|-------------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Requis pour les scripts backend |

---

## Installation

### Exigences
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** ou plus recent
- Manette DualSense (PS5)

### Lancement
1. Connectez la manette DualSense a votre ordinateur
2. Lancez `forza-dualsense.exe`
3. Configurez les parametres dans l'interface de l'application
4. Specifiez le chemin vers le .exe du jeu (optionnel, necessaire pour le lancement automatique)

## Configuration dans le jeu

Ouvrez Forza Horizon → **Parametres → HUD et Gameplay**, faites defiler vers le bas:

| Parametre | Valeur |
|-----------|--------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Fonctionnalites

### Controle de la manette
- Configuration des gachettes adaptatives pour les pedales d'acceleration et de frein
- Configuration du retour haptique
- Parametres de zone morte pour les joysticks
- Support de l'ABS (systeme antiblocage)
- Impulsion au demarrage pour confirmation de connexion

### Interface
- Design sombre moderne avec commandes intuitives
- Affichage en temps reel du statut de connexion de la manette
- Parametres UDP pour la communication avec le jeu
- Possibilite de lancer Forza Horizon directement depuis l'application (lorsque le chemin du .exe est specifie)

---

## Utilisation

### Configuration initiale
1. Lancez l'application
2. Assurez-vous que la manette est connectee
3. Configurez les parametres UDP pour la communication avec le jeu
4. Ajustez la sensibilite des gachettes et des joysticks
5. Lancez Forza Horizon via le bouton de l'application

---

## Depannage

### La manette ne se connecte pas
- Assurez-vous que la manette est connectee via USB ou Bluetooth
- Lors de l'utilisation de pilotes tiers (comme DS4W), **<u>ENTIEREMENT</u>** desactivez l'entree Steam
- Verifiez que Python est installe et fonctionne
- Desactivez l'antivirus
- Desactivez le pare-feu
- Verifiez la disponibilite du port par defaut (5300), changez-le si necessaire
- Verifiez la presence du dossier %LOCALAPPDATA%\Forza DualSense\backend
- Redemarrez l'application

---

## Configuration

Les fichiers backend sont automatiquement extraits vers:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

Les parametres de l'application sont enregistres dans le fichier de configuration.

---

## Pour les developpeurs

### Dependances de developpement

| Dependance | Description |
|------------|-------------|
| **[Node.js 18+](https://nodejs.org/)** | Requis pour le developpement et la construction frontend |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Requis pour la compilation du backend Tauri |
| **[Python 3.12](https://www.python.org/downloads/)** | Requis pour les scripts backend |

### Installation des dependances

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# ou en utilisant uv (recommande)
uv sync
```

### Developpement
```powershell
cd tauri-app
npm run tauri dev
```

### Construction de version
```powershell
cd tauri-app
npm run tauri build
```

Apres construction, le fichier exe se trouvera a:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Architecture du projet

```
tauri-app/
├── src/                 # Frontend Tauri (TypeScript/HTML)
├── src-tauri/           # Backend Tauri Rust
│   ├── backend/         # Backend IPC Python (integre dans exe)
│   │   ├── modules/     # Modules Python
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Code source Rust
│   │   └── main.rs      # Fichier principal avec logique IPC
│   ├── build.rs         # Script de construction
│   └── tauri.conf.json  # Configuration Tauri
├── package.json         # Dependances Node.js
└── build.bat            # Script de construction Windows
```

### Architecture de l'application
- **Tauri Frontend** - Interface GUI en TypeScript/HTML
- **Tauri Rust Backend** - Gestion des processus et communication IPC
- **Python IPC Backend** - Traitement des commandes de manette via stdin/stdout
- **Ressources integrees** - Fichiers backend integres dans exe via rust-embed

---

## Protocole IPC

Le backend Python communique avec le backend Rust via JSON sur stdin/stdout:

### Requetes (vers stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Reponses (de stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---
