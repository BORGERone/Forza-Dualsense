// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Child, Stdio, ChildStdin};
use std::sync::Mutex;
use std::io::{Write, BufRead, BufReader};
use std::thread;
use std::fs;
use std::path::PathBuf;
use tauri::{State, Emitter};
use tauri_plugin_dialog::DialogExt;
use serde::{Deserialize, Serialize};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

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

fn ensure_backend_in_appdata() -> Result<PathBuf, String> {
    let app_data_dir = get_app_data_dir()?;

    // Create app data directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let backend_dir = app_data_dir.join("backend");
    let backend_script = backend_dir.join("ipc_server.py");

    // Check if backend already exists in appdata
    if backend_script.exists() {
        eprintln!("Backend found in AppData: {}", backend_dir.display());
        return Ok(backend_dir);
    }

    eprintln!("Backend not found in AppData, attempting to extract from embedded resources...");

    // Extract backend from embedded resources
    extract_embedded_backend(&backend_dir)?;

    eprintln!("Backend successfully extracted to AppData: {}", backend_dir.display());
    Ok(backend_dir)
}

fn extract_embedded_backend(target_dir: &PathBuf) -> Result<(), String> {
    // Get exe directory and look for backend folder
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?
        .parent()
        .ok_or("Failed to get exe parent dir")?
        .to_path_buf();

    let source_backend = exe_dir.join("backend");

    eprintln!("Looking for backend folder at: {}", source_backend.display());

    // Copy backend from exe directory to appdata
    if source_backend.exists() {
        eprintln!("Found backend folder, copying to AppData...");
        copy_dir(&source_backend, target_dir)?;
        Ok(())
    } else {
        Err(format!("Backend folder not found at: {}. Please ensure the 'backend' folder is in the same directory as the exe file.", source_backend.display()))
    }
}

fn copy_dir(source: &PathBuf, destination: &PathBuf) -> Result<(), String> {
    if destination.exists() {
        fs::remove_dir_all(destination)
            .map_err(|e| format!("Failed to remove existing directory: {}", e))?;
    }

    fs::create_dir_all(destination)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    for entry in fs::read_dir(source)
        .map_err(|e| format!("Failed to read source directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if entry.file_type().map_err(|e| format!("Failed to get file type: {}", e))?.is_dir() {
            copy_dir(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)
                .map_err(|e| format!("Failed to copy file: {}", e))?;
        }
    }

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
    // Save settings to file
    let settings_path = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?
        .join("settings.json");

    let settings_json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, settings_json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    Ok("Settings saved".to_string())
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
async fn start_python_backend(state: State<'_, PythonProcess>, app: tauri::AppHandle) -> Result<String, String> {
    let mut process_guard = state.0.lock().unwrap();

    if process_guard.is_some() {
        return Err("Python backend is already running".to_string());
    }

    let python_cmd = if cfg!(windows) {
        "python"
    } else {
        "python3"
    };

    // Ensure backend is in AppData and get path
    let backend_dir = ensure_backend_in_appdata()?;
    let backend_path = backend_dir.join("ipc_server.py");

    eprintln!("Starting Python backend with path: {}", backend_path.display());

    let mut child = Command::new(python_cmd)
        .arg(&backend_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(&backend_dir)
        .creation_flags(if cfg!(windows) {
            0x08000000 // CREATE_NO_WINDOW
        } else {
            0
        })
        .spawn()
        .map_err(|e| format!("Failed to start Python backend: {}", e))?;
    
    let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    
    let app_handle = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                let _ = app_handle.emit("python-response", line);
            }
        }
    });
    
    *process_guard = Some(PythonProcessInner { child, stdin });
    Ok("Python backend started".to_string())
}

#[tauri::command]
async fn stop_python_backend(state: State<'_, PythonProcess>) -> Result<String, String> {
    let mut process_guard = state.0.lock().unwrap();
    
    if let Some(mut inner) = process_guard.take() {
        inner.child.kill()
            .map_err(|e| format!("Failed to kill Python backend: {}", e))?;
        Ok("Python backend stopped".to_string())
    } else {
        Err("Python backend is not running".to_string())
    }
}

#[tauri::command]
async fn send_command_to_python(command: String, state: State<'_, PythonProcess>) -> Result<String, String> {
    let mut process_guard = state.0.lock().unwrap();
    
    if process_guard.is_none() {
        return Err("Python backend is not running".to_string());
    }
    
    if let Some(inner) = process_guard.as_mut() {
        writeln!(inner.stdin, "{}", command)
            .map_err(|e| format!("Failed to write to Python stdin: {}", e))?;
        inner.stdin.flush()
            .map_err(|e| format!("Failed to flush Python stdin: {}", e))?;
    }
    
    Ok(format!("Command sent: {}", command))
}

fn main() {
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
            minimize_window,
            maximize_window,
            close_window,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
