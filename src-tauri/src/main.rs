#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use <your_app_corename>::Shared;
use std::sync::Arc;
use tauri::Manager;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    let client = Arc::new(<your_app_corename>::db::migrator::new_client().await.unwrap());
    let router = <your_app_corename>::routes::init_router();
    pretty_env_logger::init();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router, move || Shared {
            client: Arc::clone(&client),
        }))
        .invoke_handler(tauri::generate_handler![])
        .setup(
            #[allow(unused_variables)]
            |app| {
                #[cfg(not(linux))]
                {
                    let window = app.get_window("main").unwrap();

                    window_shadows::set_shadow(&window, true).unwrap();

                    #[cfg(windows)]
                    window.set_decorations(false).unwrap();
                }

                Ok(())
            },
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
