mod locales;

mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            window::window_setup::window_setup(app);
            locales::change_layout::change_layout(app.handle().clone());

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            locales::keyboard_layouts::get_layouts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
