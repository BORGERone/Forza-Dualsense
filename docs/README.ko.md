<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

# Forza DualSense
<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/4968f25e-8b51-499b-b968-292f5c191fb9" />

  <i>Forza Horizon에서 DualSense 컨트롤러를 관리하기 위한 최신 인터페이스</i>
</p>

---

## 설명

Forza DualSense는 Forza Horizon에서 PlayStation 5 DualSense 컨트롤러를 관리하는 응용 프로그램입니다. 이 프로그램은 적응형 트리거, 햅틱 피드백 및 기타 햅틱 피드백 기능을 포함한 확장 컨트롤러 사용자 지정 기능을 제공합니다.

---

## 의존성

| 의존성 | 설명 |
|-------|------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | 백엔드 스크립트에 필요 |

---

## 설치

### 요구 사항
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** 이상
- DualSense 컨트롤러 (PS5)

### 시작
1. DualSense 컨트롤러를 컴퓨터에 연결
2. `forza-dualsense.exe` 실행
3. 응용 프로그램 인터페이스에서 매개 변수 구성
4. 게임 .exe 경로 지정 (선택 사항, 자동 시작에 필요)

## 게임 내 설정

Forza Horizon → **설정 → HUD 및 게임 플레이** 열기, 아래로 스크롤:

| 설정 | 값 |
|------|-----|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## 기능

### 컨트롤러 관리
- 액셀 및 브레이크 페달용 적응형 트리거 구성
- 햅틱 피드백 구성
- 조이스틱용 데드존 설정
- ABS (안티 락 브레이크 시스템) 지원
- 연결 확인용 시작 펄스

### 인터페이스
- 직관적인 컨트롤이 있는 최신 다크 디자인
- 컨트롤러 연결 상태 실시간 표시
- 게임 통신용 UDP 매개 변수 설정
- 응용 프로그램에서 직접 Forza Horizon 시작 가능 (.exe 경로 지정 시)

---

## 사용법

### 초기 설정
1. 응용 프로그램 시작
2. 컨트롤러가 연결되어 있는지 확인
3. 게임 통신용 UDP 매개 변수 구성
4. 트리거 및 조이스틱 민감도 조정
5. 응용 프로그램 버튼으로 Forza Horizon 시작

---

## 문제 해결

### 컨트롤러가 연결되지 않음
- 컨트롤러가 USB 또는 블루투스로 연결되어 있는지 확인
- 타사 드라이버(DS4W 등) 사용 시 Steam Input을 **<u>완전히</u>** 비활성화
- Python이 설치되어 작동하는지 확인
- 안티바이러스 비활성화
- 방화벽 비활성화
- 기본 포트(5300) 사용 가능성 확인, 필요 시 변경
- %LOCALAPPDATA%\Forza DualSense\backend 폴더 존재 확인
- 응용 프로그램 다시 시작

---

## 구성

백엔드 파일은 자동으로 다음에 추출됩니다:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

응용 프로그램 설정은 구성 파일에 저장됩니다.

---

## 개발자용

### 개발 의존성

| 의존성 | 설명 |
|-------|------|
| **[Node.js 18+](https://nodejs.org/)** | 프론트엔드 개발 및 빌드에 필요 |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Tauri 백엔드 컴파일에 필요 |
| **[Python 3.12](https://www.python.org/downloads/)** | 백엔드 스크립트에 필요 |

### 의존성 설치

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# 또는 uv 사용 (권장)
uv sync
```

### 개발
```powershell
cd tauri-app
npm run tauri dev
```

### 릴리스 빌드
```powershell
cd tauri-app
npm run tauri build
```

빌드 후 exe 파일은 다음에 있습니다:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## 프로젝트 아키텍처

```
tauri-app/
├── src/                 # Tauri Frontend (TypeScript/HTML)
├── src-tauri/           # Tauri Rust Backend
│   ├── backend/         # Python IPC Backend (exe에 포함)
│   │   ├── modules/     # Python 모듈
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rust 소스 코드
│   │   └── main.rs      # IPC 로직이 포함된 메인 파일
│   ├── build.rs         # 빌드 스크립트
│   └── tauri.conf.json  # Tauri 구성
├── package.json         # Node.js 의존성
└── build.bat            # Windows 빌드 스크립트
```

### 응용 프로그램 아키텍처
- **Tauri Frontend** - TypeScript/HTML GUI 인터페이스
- **Tauri Rust Backend** - 프로세스 관리 및 IPC 통신
- **Python IPC Backend** - stdin/stdout를 통한 컨트롤러 명령 처리
- **임베드된 리소스** - rust-embed를 통해 exe에 포함된 백엔드 파일

---

## IPC 프로토콜

Python 백엔드는 stdin/stdout을 통해 JSON을 사용하여 Rust 백엔드와 통신합니다:

### 요청 (stdin으로)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### 응답 (stdout에서)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---

## 감사의 말

이 프로젝트는 [Forza Horizon DualSense Python](https://github.com/HamzaYslmn/Forza-Horizon-DualSense-Python) by HamzaYslmn 의 Python 백엔드 코드를 사용하며, AGPL v3 하에 라이선스됩니다. 원본 프로젝트는 Forza Horizon 과의 DualSense 컨트롤러 통신을 위한 핵심 기능을 제공합니다.

---
