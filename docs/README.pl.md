# Forza DualSense

<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/a831992c-0474-497a-bf23-7f85b4de5fe6" />
</p>

<p align="center">
  <i>Nowoczesny interfejs do zarzadzania kontrolerem DualSense w Forza Horizon</i>
</p>

---

## Opis

Forza DualSense to aplikacja do zarzadzania kontrolerem PlayStation 5 DualSense w grze Forza Horizon. Program zapewnia rozszerzone mozliwosci konfiguracji kontrolera, w tym adaptacyjne spusty, haptyczne informacje zwrotne i inne funkcje haptyczne.

---

## Zaleznosci

| Zaleznosc | Opis |
|-----------|-----|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | Wymagany do skryptow backend |

---

## Instalacja

### Wymagania
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** lub nowszy
- Kontroler DualSense (PS5)

### Uruchomienie
1. Podlacz kontroler DualSense do komputera
2. Uruchom `forza-dualsense.exe`
3. Skonfiguruj parametry w interfejsie aplikacji
4. Okresl sciezke do .exe gry (opcjonalne, wymagane do automatycznego uruchomienia)

## Konfiguracja w grze

Otworz Forza Horizon → **Ustawienia → HUD i rozgrywka**, przewin w dol:

| Ustawienie | Wartosc |
|-----------|---------|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## Funkcje

### Sterowanie kontrolerem
- Konfiguracja adaptacyjnych spustow dla pedali gazu i hamulca
- Konfiguracja haptycznych informacji zwrotnych
- Ustawienia strefy martwej dla joystickow
- Obsluga ABS (system antyblokujacy)
- Impuls startowy do potwierdzenia polaczenia

### Interfejs
- Nowoczesny ciemny design z intuicyjnymi kontrolami
- Wyswietlanie w czasie rzeczywistym statusu polaczenia kontrolera
- Ustawienia parametrow UDP do komunikacji z gra
- Mozliwosc uruchomienia Forza Horizon bezposrednio z aplikacji (gdy okreslono sciezke do .exe)

---

## Uzycie

### Pierwsza konfiguracja
1. Uruchom aplikacje
2. Upewnij sie, ze kontroler jest podlaczony
3. Skonfiguruj parametry UDP do komunikacji z gra
4. Dostosuj czulosc spustow i joystickow
5. Uruchom Forza Horizon przez przycisk aplikacji

---

## Rozwiazywanie problemow

### Kontroler nie laczy sie
- Upewnij sie, ze kontroler jest podlaczony przez USB lub Bluetooth
- Podczas korzystania z sterownikow innych firm (jak DS4W) **<u>KOMPLETNIE</u>** wylacz Steam Input
- Sprawdz, czy Python jest zainstalowany i dziala
- Wylacz antywirus
- Wylacz firewall
- Sprawdz dostepnosc domyslnego portu (5300), zmien go, jesli to konieczne
- Sprawdz obecnie folderu %LOCALAPPDATA%\Forza DualSense\backend
- Uruchom ponownie aplikacje

---

## Konfiguracja

Pliki backend sa automatycznie ekstrahowane do:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

Ustawienia aplikacji sa zapisywane w pliku konfiguracyjnym.

---

## Dla deweloperow

### Zaleznosci deweloperskie

| Zaleznosc | Opis |
|-----------|-----|
| **[Node.js 18+](https://nodejs.org/)** | Wymagany do rozwoju i kompilacji frontend |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Wymagany do kompilacji backend Tauri |
| **[Python 3.12](https://www.python.org/downloads/)** | Wymagany do skryptow backend |

### Instalacja zaleznosci

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# lub przy uzyciu uv (zalecane)
uv sync
```

### Rozwoj
```powershell
cd tauri-app
npm run tauri dev
```

### Kompilacja wersji
```powershell
cd tauri-app
npm run tauri build
```

Po kompilacji plik exe znajdzie sie w:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## Architektura projektu

```
tauri-app/
├── src/                 # Frontend Tauri (TypeScript/HTML)
├── src-tauri/           # Backend Tauri Rust
│   ├── backend/         # Backend IPC Python (osadzony w exe)
│   │   ├── modules/     # Moduly Python
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Kod zrodlowy Rust
│   │   └── main.rs      # Glowny plik z logika IPC
│   ├── build.rs         # Script kompilacji
│   └── tauri.conf.json  # Konfiguracja Tauri
├── package.json         # Zaleznosci Node.js
└── build.bat            # Script kompilacji Windows
```

### Architektura aplikacji
- **Tauri Frontend** - Interfejs GUI w TypeScript/HTML
- **Tauri Rust Backend** - Zarzadzanie procesami i komunikacja IPC
- **Python IPC Backend** - Przetwarzanie komend kontrolera przez stdin/stdout
- **Osadzone zasoby** - Pliki backend osadzone w exe przez rust-embed

---

## Protokol IPC

Backend Python komunikuje sie z backend Rust przez JSON przez stdin/stdout:

### Zadania (do stdin)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### Odpowiedzi (z stdout)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---
