# Forza DualSense Tauri Interface

Tauri GUI для Forza Horizon DualSense Controller с Python backend.

## Структура проекта

- **src/** - Tauri frontend (TypeScript/HTML)
- **src-tauri/** - Tauri Rust backend
- **backend/** - Python IPC backend с существующей логикой
- **backend/modules/** - Скопированные модули из оригинального проекта

## Установка зависимостей

### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# или используя uv
uv sync
```

## Запуск

### Разработка
```powershell
cd tauri-app
npm run tauri dev
```

### Сборка
```powershell
cd tauri-app
npm run tauri build
```

## Архитектура

1. **Tauri Frontend** - GUI интерфейс на TypeScript/HTML
2. **Tauri Rust Backend** - Запускает Python subprocess
3. **Python IPC Backend** - Обрабатывает команды через stdin/stdout и использует существующую логику

## IPC Протокол

Python backend общается через JSON по stdin/stdout:

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

## Статус разработки

- [x] Структура Tauri проекта
- [x] Python IPC backend
- [x] Rust subprocess management
- [x] Frontend UI (базовый)
- [ ] Полная интеграция с существующей логикой
- [ ] Настройки в GUI
- [ ] Логи в GUI
- [ ] Тестирование

## TODO

1. Реализовать полноценную коммуникацию между Rust и Python через stdin/stdout
2. Добавить все настройки в GUI интерфейс
3. Добавить отображение логов в реальном времени
4. Интегрировать существующую логику UDP listener
5. Добавить иконки и стилизацию
