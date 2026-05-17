fn main() {
    tauri_build::build();

    // Copy backend files to build output for NSIS installer
    let backend_dir = std::path::PathBuf::from("backend");
    let target_dir = std::env::var("OUT_DIR").unwrap();
    let install_dir = std::path::PathBuf::from(&target_dir).join("../../../backend");

    if backend_dir.exists() {
        println!("cargo:rerun-if-changed=backend");
        // Copy backend directory to build output
        if let Err(e) = fs_extra::dir::copy(&backend_dir, &install_dir, &fs_extra::dir::CopyOptions::new()) {
            eprintln!("Warning: Failed to copy backend files: {}", e);
        }
    }
}
