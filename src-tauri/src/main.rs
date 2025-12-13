#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod models;
mod services;
mod utils;

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // 初始化 SQLite 存储，失败时仅打印警告以避免阻塞启动
            if let Err(e) = tauri::async_runtime::block_on(services::storage::init_db()) {
                println!("警告: 无法初始化本地数据库: {}", e);
            }

            // 创建系统托盘
            let show_item = MenuItemBuilder::with_id("show", "显示主窗口").build(app)?;
            let hide_item = MenuItemBuilder::with_id("hide", "隐藏主窗口").build(app)?;
            let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            
            let menu = MenuBuilder::new(app)
                .items(&[&show_item, &hide_item, &separator, &quit_item])
                .build()?;
            
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
            Ok(())
        })
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
            // 设置相关API
            api::settings::get_settings,
            api::settings::save_settings,
            api::settings::get_download_path,
            api::settings::set_download_path,
            api::settings::select_download_folder,
            api::settings::open_download_folder,
            // 存储相关API
            api::storage::get_installed_trainers,
            api::storage::save_installed_trainers,
            api::storage::get_downloaded_trainers,
            api::storage::save_downloaded_trainers,
            api::storage::cache_trainer_list,
            api::storage::get_cached_trainer_list,
            api::storage::cache_search_results,
            api::storage::get_cached_search_results,
            api::storage::clean_expired_cache,
            api::storage::migrate_from_local_storage,
            api::storage::get_storage_keys,
            api::storage::clear_all_storage,
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
