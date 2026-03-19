mod adobe;
mod commands;
mod models;
mod projects;
mod session;
mod startup;
mod system;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_scan_snapshot,
            commands::get_everything_status,
            commands::get_project_index,
            commands::install_everything,
            commands::clear_directories,
            commands::set_gpu_preference,
            commands::set_performance_mode,
            commands::apply_power_profile,
            commands::disable_startup_item,
            commands::session_status,
            commands::start_session_mode,
            commands::stop_session_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
