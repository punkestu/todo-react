// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
mod app;
use app::{
    commands::{complete_todo, create_todo, get_archive, get_list, get_recent, uncomplete_todo},
    lib, repo_impl, service,
};
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let pool = lib::db_conn::gen_pool(&rt).unwrap();
    let r = repo_impl::mysql_impl::TodoImpl::new(pool, rt);
    let s = service::Todo::new(r);
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(s)
        .invoke_handler(tauri::generate_handler![
            complete_todo,
            create_todo,
            get_archive,
            get_list,
            get_recent,
            uncomplete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
