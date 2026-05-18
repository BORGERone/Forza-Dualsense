// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Child, Stdio, ChildStdin};
use std::sync::Mutex;
use std::io::{BufRead, BufReader};
use std::thread;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use std::path::PathBuf;
use tauri::State;
use tauri_plugin_dialog::DialogExt;
use serde::{Deserialize, Serialize};
use rust_embed::RustEmbed;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn setup_logging() -> std::io::Result<std::fs::File> {
    let app_data_dir = get_app_data_dir()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    let log_file = app_data_dir.join(format!("rust_backend_{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S")));
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)?;
    
    // Don't print to stderr to avoid console window
    Ok(file)
}

fn log_to_file(message: &str) {
    if let Ok(mut file) = setup_logging() {
        let _ = writeln!(file, "[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), message);
    }
    // Don't print to stderr to avoid console window
}

#[derive(RustEmbed)]
#[folder = "backend/"]
struct BackendAssets;

struct PythonProcess(Mutex<Option<PythonProcessInner>>);

struct PythonProcessInner {
    child: Child,
    stdin: ChildStdin,
}

fn get_app_data_dir() -> Result<PathBuf, String> {
    let local_app_data = std::env::var("LOCALAPPDATA")
        .map_err(|e| format!("Failed to get LOCALAPPDATA: {}", e))?;
    let app_data_dir = PathBuf::from(local_app_data).join("Forza DualSense");
    Ok(app_data_dir)
}

fn find_python() -> Result<String, String> {
    log_to_file("find_python: called");
    
    // Try to find python in PATH
    if let Ok(output) = std::process::Command::new("python").arg("--version").output() {
        if output.status.success() {
            log_to_file("Found 'python' in PATH");
            return Ok("python".to_string());
        }
    }
    
    if let Ok(output) = std::process::Command::new("python3").arg("--version").output() {
        if output.status.success() {
            log_to_file("Found 'python3' in PATH");
            return Ok("python3".to_string());
        }
    }

    // Try common installation paths on Windows
    if cfg!(windows) {
        log_to_file("Checking common Python installation paths on Windows");
        let common_paths = vec![
            r"C:\Python313\python.exe",
            r"C:\Python312\python.exe",
            r"C:\Python311\python.exe",
            r"C:\Python310\python.exe",
            r"C:\Program Files\Python313\python.exe",
            r"C:\Program Files\Python312\python.exe",
            r"C:\Program Files\Python311\python.exe",
            r"C:\Program Files\Python310\python.exe",
            r"C:\Users\%USERNAME%\AppData\Local\Programs\Python\Python313\python.exe",
            r"C:\Users\%USERNAME%\AppData\Local\Programs\Python\Python312\python.exe",
            r"C:\Users\%USERNAME%\AppData\Local\Programs\Python\Python311\python.exe",
            r"C:\Users\%USERNAME%\AppData\Local\Programs\Python\Python310\python.exe",
        ];

        for path in common_paths {
            let expanded_path = path.replace("%USERNAME%", &std::env::var("USERNAME").unwrap_or_default());
            if PathBuf::from(&expanded_path).exists() {
                log_to_file(&format!("Found Python at: {}", expanded_path));
                return Ok(expanded_path);
            }
        }
    }

    let err = "Python not found. Please install Python 3.10 or later.".to_string();
    log_to_file(&err);
    Err(err)
}

fn install_python_dependencies(python_exe: &str, backend_dir: &PathBuf) -> Result<(), String> {
    log_to_file("Checking Python dependencies...");
    
    // Check if dependencies are installed by trying to import them
    let check_script = r#"
import sys
try:
    import hid
    import psutil
    import textual
    print("DEPENDENCIES_OK")
except ImportError as e:
    print(f"MISSING: {e}")
    sys.exit(1)
"#;
    
    let check_result = Command::new(python_exe)
        .arg("-c")
        .arg(check_script)
        .current_dir(backend_dir)
        .output()
        .map_err(|e| format!("Failed to check dependencies: {}", e))?;
    
    if String::from_utf8_lossy(&check_result.stdout).contains("DEPENDENCIES_OK") {
        log_to_file("All dependencies are already installed");
        return Ok(());
    }
    
    log_to_file("Dependencies not found, installing via pip...");
    
    // Install dependencies using pip
    let install_result = Command::new(python_exe)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("hidapi>=0.15.0")
        .arg("psutil>=7.2.2")
        .arg("textual>=8.2.5")
        .current_dir(backend_dir)
        .output()
        .map_err(|e| format!("Failed to install dependencies: {}", e))?;
    
    if !install_result.status.success() {
        let error = String::from_utf8_lossy(&install_result.stderr);
        let err_msg = format!("Failed to install dependencies: {}", error);
        log_to_file(&err_msg);
        return Err(err_msg);
    }
    
    log_to_file("Dependencies installed successfully");
    Ok(())
}

fn ensure_backend_in_appdata() -> Result<PathBuf, String> {
    log_to_file("ensure_backend_in_appdata: called");
    let app_data_dir = get_app_data_dir()?;
    log_to_file(&format!("AppData directory: {}", app_data_dir.display()));

    // Create app data directory if it doesn't exist
    if !app_data_dir.exists() {
        log_to_file("Creating AppData directory...");
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| {
                let err = format!("Failed to create app data directory: {}", e);
                log_to_file(&err);
                err
            })?;
    }

    let backend_dir = app_data_dir.join("backend");
    let backend_script = backend_dir.join("ipc_server.py");

    // Check if backend already exists in appdata
    if backend_script.exists() {
        log_to_file(&format!("Backend found in AppData: {}", backend_dir.display()));
        return Ok(backend_dir);
    }

    log_to_file("Backend not found in AppData, attempting to extract from embedded resources...");

    // Extract backend from embedded resources
    extract_embedded_backend(&backend_dir)?;

    log_to_file(&format!("Backend successfully extracted to AppData: {}", backend_dir.display()));
    Ok(backend_dir)
}

fn extract_embedded_backend(target_dir: &PathBuf) -> Result<(), String> {
    log_to_file("=== BACKEND EXTRACTION DEBUG ===");
    log_to_file(&format!("Target directory: {}", target_dir.display()));

    // Try to extract from embedded rust-embed resources first
    log_to_file("Attempting to extract backend from embedded resources...");

    // Check if we have any embedded files
    let has_embedded = BackendAssets::iter().next().is_some();
    log_to_file(&format!("Has embedded backend files: {}", has_embedded));

    if has_embedded {
        log_to_file("Extracting embedded backend files to AppData...");
        extract_embedded_files(target_dir)?;
        log_to_file("Backend extracted successfully from embedded resources");
        return Ok(());
    }

    // Fallback for dev mode: Look for backend folder next to exe
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?
        .parent()
        .ok_or("Failed to get exe parent dir")?
        .to_path_buf();
    log_to_file(&format!("No embedded files found (dev mode), looking for backend folder at: {}", exe_dir.display()));

    let source_backend = exe_dir.join("backend");
    log_to_file(&format!("Backend folder exists: {}", source_backend.exists()));

    if source_backend.exists() {
        log_to_file("Found backend folder next to exe, copying to AppData...");
        copy_dir(&source_backend, target_dir)?;
        log_to_file("Backend copied successfully from exe directory");
        Ok(())
    } else {
        let error_msg = format!(
            "Backend folder not found at: {}. Also not found in embedded resources. Please ensure the 'backend' folder is in the same directory as the exe file.",
            source_backend.display()
        );
        log_to_file(&error_msg);
        Err(error_msg)
    }
}

fn extract_embedded_files(target_dir: &PathBuf) -> Result<(), String> {
    for file_path in BackendAssets::iter() {
        let file_path = file_path.to_string();
        let target_path = target_dir.join(&file_path);

        // Create parent directories if needed
        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| {
                        let err = format!("Failed to create directory {}: {}", parent.display(), e);
                        log_to_file(&err);
                        err
                    })?;
            }
        }

        // Extract file content
        if let Some(content) = BackendAssets::get(&file_path) {
            let data = content.data;
            fs::write(&target_path, data)
                .map_err(|e| {
                    let err = format!("Failed to write file {}: {}", target_path.display(), e);
                    log_to_file(&err);
                    err
                })?;
            log_to_file(&format!("Extracted: {}", file_path));
        }
    }
    Ok(())
}

fn copy_dir(source: &PathBuf, destination: &PathBuf) -> Result<(), String> {
    log_to_file(&format!("copy_dir: source={}, destination={}", source.display(), destination.display()));
    
    if destination.exists() {
        log_to_file("Removing existing directory...");
        fs::remove_dir_all(destination)
            .map_err(|e| {
                let err = format!("Failed to remove existing directory: {}", e);
                log_to_file(&err);
                err
            })?;
    }

    log_to_file("Creating destination directory...");
    fs::create_dir_all(destination)
        .map_err(|e| {
            let err = format!("Failed to create directory: {}", e);
            log_to_file(&err);
            err
        })?;

    log_to_file("Copying files...");
    for entry in fs::read_dir(source)
        .map_err(|e| {
            let err = format!("Failed to read source directory: {}", e);
            log_to_file(&err);
            err
        })?
    {
        let entry = entry.map_err(|e| {
            let err = format!("Failed to read entry: {}", e);
            log_to_file(&err);
            err
        })?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if entry.file_type().map_err(|e| {
            let err = format!("Failed to get file type: {}", e);
            log_to_file(&err);
            err
        })?.is_dir() {
            copy_dir(&source_path, &destination_path)?;
        } else {
            log_to_file(&format!("Copying file: {} -> {}", source_path.display(), destination_path.display()));
            fs::copy(&source_path, &destination_path)
                .map_err(|e| {
                    let err = format!("Failed to copy file: {}", e);
                    log_to_file(&err);
                    err
                })?;
        }
    }

    log_to_file("Directory copied successfully");
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Settings {
    udp_host: String,
    udp_port: u32,
    brake_deadzone: u32,
    brake_baseline_force: u32,
    brake_max_force: u32,
    enable_brake_resistance: bool,
    enable_throttle_resistance: bool,
    enable_abs: bool,
    enable_startup_pulse: bool,
}

#[tauri::command]
async fn open_file_dialog(app: tauri::AppHandle) -> Result<String, String> {
    let file_path = app.dialog()
        .file()
        .add_filter("Executable", &["exe"])
        .blocking_pick_file()
        .ok_or_else(|| "No file selected".to_string())?;
    
    Ok(file_path.to_string())
}

#[tauri::command]
async fn launch_exe(path: String) -> Result<String, String> {
    Command::new(&path)
        .spawn()
        .map_err(|e| format!("Failed to launch exe: {}", e))?;
    Ok(format!("Launched: {}", path))
}

#[tauri::command]
async fn save_settings(settings: Settings) -> Result<String, String> {
    // Save settings to AppData
    let app_data_dir = get_app_data_dir()?;

    // Create app data directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let settings_path = app_data_dir.join("settings.json");

    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, settings_json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    Ok("Settings saved".to_string())
}

#[tauri::command]
async fn load_settings() -> Result<Settings, String> {
    // Load settings from AppData
    let app_data_dir = get_app_data_dir()?;
    let settings_path = app_data_dir.join("settings.json");

    if !settings_path.exists() {
        // Return default settings if file doesn't exist
        return Ok(Settings {
            udp_host: "127.0.0.1".to_string(),
            udp_port: 5300,
            brake_deadzone: 50,
            brake_baseline_force: 15,
            brake_max_force: 60,
            enable_brake_resistance: true,
            enable_throttle_resistance: true,
            enable_abs: true,
            enable_startup_pulse: true,
        });
    }

    let settings_json = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: Settings = serde_json::from_str(&settings_json)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    Ok(settings)
}

#[tauri::command]
async fn minimize_window(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| format!("Failed to minimize window: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn maximize_window(window: tauri::Window) -> Result<(), String> {
    window.maximize().map_err(|e| format!("Failed to maximize window: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn close_window(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| format!("Failed to close window: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn open_url(url: String, app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_url(&url, None::<String>)
        .map_err(|e| format!("Failed to open URL: {}", e))
}

#[tauri::command]
async fn start_python_backend(state: State<'_, PythonProcess>) -> Result<String, String> {
    log_to_file("start_python_backend: called");
    let mut process_guard = state.0.lock().unwrap();

    if process_guard.is_some() {
        let err = "Python backend is already running".to_string();
        log_to_file(&err);
        return Err(err);
    }

    let backend_dir = ensure_backend_in_appdata()?;
    let python_exe = find_python()?;

    log_to_file(&format!("Starting Python backend..."));
    log_to_file(&format!("Python executable: {}", python_exe));
    log_to_file(&format!("Backend directory: {}", backend_dir.display()));

    // Install dependencies if needed
    install_python_dependencies(&python_exe, &backend_dir)?;

    let ipc_server_path = backend_dir.join("ipc_server.py");

    if !ipc_server_path.exists() {
        let error_msg = format!("ipc_server.py not found at: {}", ipc_server_path.display());
        log_to_file(&error_msg);
        return Err(error_msg);
    }

    let mut cmd = Command::new(&python_exe);
    cmd.arg(&ipc_server_path)
        .current_dir(&backend_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    // On Windows, create process without console window
    #[cfg(windows)]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    
    let mut child = cmd.spawn()
        .map_err(|e| {
            let err = format!("Failed to spawn Python process: {}", e);
            log_to_file(&err);
            err
        })?;

    log_to_file(&format!("Python process started with PID: {}", child.id()));

    let stdin = child.stdin.take().ok_or("Failed to open stdin")?;

    // Spawn a thread to read stderr and log it
    let stderr = child.stderr.take().ok_or("Failed to open stderr")?;
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                log_to_file(&format!("[Python stderr] {}", line));
            }
        }
    });

    *process_guard = Some(PythonProcessInner { child, stdin });
    Ok("Python backend started".to_string())
}

#[tauri::command]
async fn stop_python_backend(state: State<'_, PythonProcess>) -> Result<String, String> {
    log_to_file("stop_python_backend: called");
    let mut process_guard = state.0.lock().unwrap();
    
    if let Some(mut inner) = process_guard.take() {
        log_to_file("Stopping Python backend...");
        inner.child.kill()
            .map_err(|e| {
                let err = format!("Failed to kill Python backend: {}", e);
                log_to_file(&err);
                err
            })?;
        log_to_file("Python backend stopped successfully");
        Ok("Python backend stopped".to_string())
    } else {
        let err = "Python backend is not running".to_string();
        log_to_file(&err);
        Err(err)
    }
}

#[tauri::command]
async fn send_command_to_python(command: String, state: State<'_, PythonProcess>) -> Result<String, String> {
    log_to_file(&format!("send_command_to_python: command = {}", command));
    let mut process_guard = state.0.lock().unwrap();
    
    if process_guard.is_none() {
        let err = "Python backend is not running".to_string();
        log_to_file(&err);
        return Err(err);
    }
    
    if let Some(inner) = process_guard.as_mut() {
        writeln!(inner.stdin, "{}", command)
            .map_err(|e| {
                let err = format!("Failed to write to Python stdin: {}", e);
                log_to_file(&err);
                err
            })?;
        inner.stdin.flush()
            .map_err(|e| {
                let err = format!("Failed to flush Python stdin: {}", e);
                log_to_file(&err);
                err
            })?;
    }
    
    let msg = format!("Command sent: {}", command);
    log_to_file(&msg);
    Ok(msg)
}

fn main() {
    log_to_file("=== APPLICATION START ===");
    log_to_file(&format!("OS: {}", std::env::consts::OS));
    log_to_file(&format!("Arch: {}", std::env::consts::ARCH));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(PythonProcess(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            start_python_backend,
            stop_python_backend,
            send_command_to_python,
            open_file_dialog,
            launch_exe,
            save_settings,
            load_settings,
            minimize_window,
            maximize_window,
            close_window,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
