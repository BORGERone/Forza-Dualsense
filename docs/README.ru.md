<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Снимок экрана 2026-05-18 212927" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Современный интерфейс для управления геймпадом DualSense в Forza Horizon</i>
</p>

---

## Описание

Forza DualSense - это приложение для управления геймпадом PlayStation 5 DualSense в игре Forza Horizon. Программа обеспечивает расширенные возможности настройки контроллера, включая адаптивный триггер, тактильную отдачу и другие функции haptic feedback.

---

## Зависимости

| Зависимость | Описание |
|------------|----------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Необходима для работы backend скриптов |

---

## Установка

### Требования
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** или новее
- Геймпад DualSense (PS5)

### Запуск
1. Подключите геймпад DualSense к компьютеру
2. Запустите `forza-dualsense.exe`
3. Настройте параметры в интерфейсе приложения
4. Укажите путь до .exe игры (по желанию, нужно для авто запуска игры)

## Настройка в игре

Откройте Forza Horizon → **Настройки → Приборная панель и игровой процесс**, прокрутите вниз:

| Настройка | Значение |
|-----------|----------|
| Пересылка данных | **Вкл** |
| Пересылка данных: IP-адрес | **127.0.0.1** |
| Пересылка данных: порт | **5300** |

---

## Основные возможности

### Управление геймпадом
- Настройка адаптивных триггеров для педалей газа и тормоза
- Конфигурация тактильной отдачи (haptic feedback)
- Настройка зон мертвой хода (deadzone) для джойстиков
- Поддержка функции ABS (антиблокировочной системы)
- Импульс при запуске для подтверждения подключения

### Интерфейс
- Современный темный дизайн с интуитивным управлением
- Отображение статуса подключения геймпада в реальном времени
- Настройки параметров UDP для связи с игрой
- Возможность запуска Forza Horizon прямо из приложения (при указании пути до .exe)

---

## Использование

### Первая настройка
1. Запустите приложение
2. Убедитесь, что геймпад подключен
3. Настройте параметры UDP для связи с игрой
4. Настройте чувствительность триггеров и джойстиков
5. Запустите Forza Horizon через кнопку в приложении

---

## Ошибки

### Геймпад не подключается
- Убедитесь, что геймпад подключен через USB или Bluetooth
- При использвании сторонних драйверов (вроде DS4W) **<u>ПОЛНОСТЬЮ</u>** отключите систему ввода Steam
- Проверьте, что Python установлен и работает
- Отключите антивирус
- Отключите Firewall
- Проверьте доступность порта по умолчанию (5300), смените его при необходимости
- Проверьте наличие папки %LOCALAPPDATA%\Forza DualSense\backend
- Перезапустите приложение

---

## Конфигурация

Backend файлы автоматически извлекаются в:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

Настройки приложения сохраняются в файле конфигурации.

---

## Для разработчиков

### Зависимости для разработки

| Зависимость | Описание |
|------------|----------|
| **[Node.js 18+](https://nodejs.org/)** | Необходима для разработки и сборки frontend |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Необходима для компиляции Tauri backend |
| **[Python 3.12](https://www.python.org/downloads/)** | Необходима для работы backend скриптов |

### Установка зависимостей

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# или используя uv (рекомендуется)
uv sync
```

### Разработка
```powershell
cd tauri-app
npm run tauri dev
```

### Сборка релиза
```powershell
cd tauri-app
npm run tauri build
```

После сборки exe файл будет находиться в:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Архитектура проекта

```
tauri-app/
├── src/                 # Tauri frontend (TypeScript/HTML)
├── src-tauri/           # Tauri Rust backend
│   ├── backend/         # Python IPC backend (встраивается в exe)
│   │   ├── modules/     # Модули Python
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rust исходный код
│   │   └── main.rs      # Основной файл с IPC логикой
│   ├── build.rs         # Build скрипт
│   └── tauri.conf.json  # Конфигурация Tauri
├── package.json         # Зависимости Node.js
└── build.bat            # Скрипт сборки для Windows
```

### Архитектура приложения
- **Tauri Frontend** - GUI интерфейс на TypeScript/HTML
- **Tauri Rust Backend** - Управление процессами и IPC коммуникация
- **Python IPC Backend** - Обработка команд геймпада через stdin/stdout
- **Встраиваемые ресурсы** - Backend файлы встроены в exe через rust-embed

---

## IPC Протокол

Python backend общается с Rust backend через JSON по stdin/stdout:

### Запросы (в stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Ответы (из stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---