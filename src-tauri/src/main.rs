#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod models;
mod services;
mod utils;

fn main() {
    // 初始化日志
    if let Err(e) = services::logger::init_logger() {
        println!("警告: 无法初始化日志系统: {}", e);
    }

    // 应用启动日志
    let _ = services::logger::log(
        services::logger::LogLevel::Info,
        "====== 应用程序启动 ======",
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            // 修改器相关API
            api::trainer::fetch_trainers,
            api::trainer::search_trainers,
            api::trainer::get_trainer_detail,
            api::trainer::download_trainer,
            api::trainer::delete_trainer,
            api::trainer::launch_trainer,
            // 更新相关API
            api::updater::check_update,
            api::updater::download_and_install_update,
            api::updater::get_app_version,
            // 管理员权限API
            api::admin::restart_as_admin,
            // 下载管理器API
            services::download_manager::get_active_downloads,
            services::download_manager::cancel_download,
            services::download_manager::clear_all_downloads,
            services::download_manager::get_download_directory,
            services::download_manager::open_download_directory,
            // 日志API
            services::logger::get_log_dir,
            services::logger::export_latest_log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
