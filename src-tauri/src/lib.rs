pub mod adobe;
pub mod commands;
pub mod models;
pub mod projects;
pub mod session;
pub mod startup;
pub mod system;
pub mod util;
pub mod fonts;
pub mod expressions;

use std::sync::Mutex;
use sysinfo::System;

pub struct AppState {
    pub sys: Mutex<System>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(System::new_all()),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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
            commands::stop_session_mode,
            commands::toggle_plugin,
            commands::get_render_status,
            commands::down_convert_aep,
            commands::install_ae_script,
            commands::purge_auto_saves,
            commands::audit_project_fonts,
            commands::get_expression_logs,
            commands::audit_project_expressions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
