# Forza DualSense

<p align="center">
<img width="256" height="256" alt="icon" src="https://github.com/user-attachments/assets/408a0795-5182-40a2-b939-631545007050" />
</p>

<p align="center">
  <img width="1202" height="802" alt="Screenshot" src="https://github.com/user-attachments/assets/a831992c-0474-497a-bf23-7f85b4de5fe6" />
</p>

<p align="center">
  <i>Forza HorizonでDualSenseコントローラーを管理するための最新インターフェース</i>
</p>

---

## 説明

Forza DualSenseはForza HorizonでPlayStation 5 DualSenseコントローラーを管理するためのアプリケーションです。このプログラムはアダプティブトリガー、触覚フィードバック、その他の触覚フィードバック機能を含む拡張コントローラー設定機能を提供します。

---

## 依存関係

| 依存関係 | 説明 |
|---------|------|
| **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** | バックエンドスクリプトに必要 |

---

## インストール

### 要件
- **[Python 3.12](https://apps.microsoft.com/detail/9ncvdn91xzqp)** 以降
- DualSenseコントローラー (PS5)

### 起動
1. DualSenseコントローラーをコンピューターに接続
2. `forza-dualsense.exe` を起動
3. アプリケーションインターフェースでパラメーターを設定
4. ゲームの.exeへのパスを指定（オプション、自動起動に必要）

## ゲーム内設定

Forza Horizon → **設定 → HUDとゲームプレイ** を開き、下にスクロール:

| 設定 | 値 |
|------|-----|
| Data Out | **ON** |
| Data Out IP Address | **127.0.0.1** |
| Data Out IP Port | **5300** |

---

## 機能

### コントローラー管理
- アクセルとブレーキペダル用アダプティブトリガー設定
- 触覚フィードバック設定
- ジョイスティック用デッドゾーン設定
- ABS（アンチロックブレーキシステム）サポート
- 接続確認用スタートアップパルス

### インターフェース
- 直感的なコントロールを備えた最新ダークデザイン
- コントローラー接続ステータスのリアルタイム表示
- ゲーム通信用UDPパラメーター設定
- アプリケーションから直接Forza Horizonを起動可能（.exeパス指定時）

---

## 使用方法

### 初期設定
1. アプリケーションを起動
2. コントローラーが接続されていることを確認
3. ゲーム通信用UDPパラメーターを設定
4. トリガーとジョイスティックの感度を調整
5. アプリケーションボタンからForza Horizonを起動

---

## トラブルシューティング

### コントローラーが接続されない
- コントローラーがUSBまたはBluetoothで接続されていることを確認
- サードパーティードライバー（DS4Wなど）を使用する場合、Steam Inputを**<u>完全に</u>**無効化
- Pythonがインストールされ動作していることを確認
- アンチウイルスを無効化
- ファイアウォールを無効化
- デフォルトポート（5300）の可用性を確認、必要に応じて変更
- %LOCALAPPDATA%\Forza DualSense\backendフォルダーの存在を確認
- アプリケーションを再起動

---

## 設定

バックエンドファイルは自動的に以下に抽出されます:
```
%LOCALAPPDATA%\Forza DualSense\backend
```

アプリケーション設定は設定ファイルに保存されます。

---

## 開発者向け

### 開発依存関係

| 依存関係 | 説明 |
|---------|------|
| **[Node.js 18+](https://nodejs.org/)** | フロントエンド開発とビルドに必要 |
| **[Rust 1.70+](https://www.rust-lang.org/tools/install)** | Tauriバックエンドコンパイルに必要 |
| **[Python 3.12](https://www.python.org/downloads/)** | バックエンドスクリプトに必要 |

### 依存関係のインストール

#### Frontend (Tauri)
```powershell
cd tauri-app
npm install
```

#### Backend (Python)
```powershell
cd backend
pip install -r requirements.txt
# またはuvを使用（推奨）
uv sync
```

### 開発
```powershell
cd tauri-app
npm run tauri dev
```

### リリースビルド
```powershell
cd tauri-app
npm run tauri build
```

ビルド後、exeファイルは以下にあります:
```
src-tauri/target/release/forza-dualsense.exe
```

---

## プロジェクトアーキテクチャ

```
tauri-app/
├── src/                 # Tauri Frontend (TypeScript/HTML)
├── src-tauri/           # Tauri Rust Backend
│   ├── backend/         # Python IPC Backend (exeに埋め込み)
│   │   ├── modules/     # Pythonモジュール
│   │   ├── ipc_server.py
│   │   └── pyproject.toml
│   ├── src/             # Rustソースコード
│   │   └── main.rs      # IPCロジックを含むメインファイル
│   ├── build.rs         # ビルドスクリプト
│   └── tauri.conf.json  # Tauri設定
├── package.json         # Node.js依存関係
└── build.bat            # Windowsビルドスクリプト
```

### アプリケーションアーキテクチャ
- **Tauri Frontend** - TypeScript/HTML GUIインターフェース
- **Tauri Rust Backend** - プロセス管理とIPC通信
- **Python IPC Backend** - stdin/stdout経由のコントローラーコマンド処理
- **埋め込みリソース** - rust-embed経由でexeに埋め込まれたバックエンドファイル

---

## IPCプロトコル

Pythonバックエンドはstdin/stdout経由でJSONを使用してRustバックエンドと通信します:

### リクエスト (stdinへ)
```json
{"type": "start"}
{"type": "stop"}
{"type": "get_status"}
{"type": "update_setting", "key": "udp_port", "value": 5300}
```

### レスポンス (stdoutから)
```json
{"status": "started", "message": "Backend started successfully"}
{"type": "status", "dualsense_connected": true, "running": true, "settings": {...}}
```

---
