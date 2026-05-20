<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Interface moderno para gerenciar o controle DualSense no Forza Horizon</i>
</p>

---

## Descricao

Forza DualSense e um aplicativo para gerenciar o controle PlayStation 5 DualSense no Forza Horizon. O programa fornece recursos estendidos de personalizacao do controlador, incluindo gatilhos adaptativos, feedback haptico e outras funcoes de feedback haptico.

---

## Dependencias

| Dependencia | Descricao |
|------------|-----------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Necessario para scripts backend |

---

## Instalacao

### Requisitos
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** ou mais recente
- Controle DualSense (PS5)

### Inicio
1. Conecte o controle DualSense ao seu computador
2. Inicie `forza-dualsense.exe`
3. Configure os parametros na interface do aplicativo
4. Especifique o caminho para o .exe do jogo (opcional, necessario para inicio automatico)

## Configuracao no jogo

Abra Forza Horizon → **Configuracoes → HUD e Gameplay**, role para baixo:

| Configuracao | Valor |
|--------------|-------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Recursos

### Controle do gamepad
- Configuracao de gatilhos adaptativos para pedais de aceleracao e freio
- Configuracao de feedback haptico
- Ajustes de zona morta para joysticks
- Suporte ABS (sistema antibloqueio)
- Impulso de inicio para confirmacao de conexao

### Interface
- Design escuro moderno com controles intuitivos
- Exibicao em tempo real do status de conexao do controle
- Configuracoes de parametros UDP para comunicacao com o jogo
- Possibilidade de iniciar Forza Horizon diretamente do aplicativo (quando o caminho do .exe e especificado)

---

## Uso

### Configuracao inicial
1. Inicie o aplicativo
2. Certifique-se de que o controle esteja conectado
3. Configure os parametros UDP para comunicacao com o jogo
4. Ajuste a sensibilidade dos gatilhos e joysticks
5. Inicie Forza Horizon atraves do botao do aplicativo

---

## Solucao de problemas

### O controle nao conecta
- Certifique-se de que o controle esteja conectado via USB ou Bluetooth
- Ao usar drivers de terceiros (como DS4W), **<u>COMPLETAMENTE</u>** desative Steam Input
- Verifique se o Python esta instalado e funcionando
- Desative o antivirus
- Desative o Firewall
- Verifique a disponibilidade da porta padrao (5300), altere se necessario
- Verifique a presenca da pasta %LOCALAPPDATA%\Forza DualSense\backend
- Reinicie o aplicativo

---

## Configuracao

Os arquivos backend sao extraidos automaticamente para:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

As configuracoes do aplicativo sao salvas no arquivo de configuracao.

---

## Para desenvolvedores

### Dependencias de desenvolvimento

| Dependencia | Descricao |
|------------|-----------|
| **[Node.js 18+](https://nodejs.org/)** | Necessario para desenvolvimento e build frontend |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Necessario para compilacao do backend Tauri |
| **[Python 3.12](https://www.python.org/downloads/)** | Necessario para scripts backend |

### Instalacao de dependencias

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# ou usando uv (recomendado)
uv sync
```

### Desenvolvimento
```powershell
cd tauri-app
npm run tauri dev
```

### Build de release
```powershell
cd tauri-app
npm run tauri build
```

Apos o build, o arquivo exe estara localizado em:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Arquitetura do projeto

```
tauri-app/
├── src/                 # Frontend Tauri (TypeScript/HTML)
├── src-tauri/           # Backend Tauri Rust
│   ├── backend/         # Backend IPC Python (incorporado no exe)
│   │   ├── modules/     # Modulos Python
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Codigo fonte Rust
│   │   └── main.rs      # Arquivo principal com logica IPC
│   ├── build.rs         # Script de build
│   └── tauri.conf.json  # Configuracao Tauri
├── package.json         # Dependencias Node.js
└── build.bat            # Script de build Windows
```

### Arquitetura do aplicativo
- **Tauri Frontend** - Interface GUI em TypeScript/HTML
- **Tauri Rust Backend** - Gerenciamento de processos e comunicacao IPC
- **Python IPC Backend** - Processamento de comandos do controle via stdin/stdout
- **Recursos incorporados** - Arquivos backend incorporados no exe via rust-embed

---

## Protocolo IPC

O backend Python se comunica com o backend Rust via JSON sobre stdin/stdout:

### Solicitacoes (para stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Respostas (de stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---

## Agradecimentos

Este projeto usa o código backend Python de [Forza Horizon DualSense Python](https://github.com/HamzaYslmn/Forza-Horizon-DualSense-Python) por HamzaYslmn, que é licenciado sob AGPL v3. O projeto original fornece a funcionalidade principal para a comunicação do controle DualSense com Forza Horizon.

---
