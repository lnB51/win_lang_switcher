use crate::locales::keyboard_layouts::get_layouts;

pub fn window_setup(app: &mut tauri::App) {
    let window_builder =
        tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default())
            .decorations(false)
            .shadow(true)
            .skip_taskbar(true)
            .always_on_top(true)
            .visible(false)
            .resizable(false);
    let window = window_builder.build().unwrap();
    let monitor = app
        .primary_monitor()
        .expect("Primary monitor not found")
        .expect("No monitors found");
    let size = tauri::window::Monitor::size(&monitor);
    let _ = window.set_size(tauri::PhysicalSize::new(
        size.width as f64 * (get_layouts().len() as f64 * 0.07),
        size.height as f64 * 0.1,
    ));
    let _ = window.center();
}
