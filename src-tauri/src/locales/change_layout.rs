use std::thread;
// use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::{AppHandle, Emitter as _};

#[tauri::command]
pub fn change_layout(app: AppHandle){
    let device_state = DeviceState::new();

    let _handle = thread::spawn(move || {
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            if (keys.contains(&Keycode::LMeta) && keys.contains(&Keycode::Space))
                || (keys.contains(&Keycode::LShift) && keys.contains(&Keycode::LAlt))
            {
                if !(keys.contains(&Keycode::LMeta) && keys.contains(&Keycode::Space))
                || !(keys.contains(&Keycode::LShift) && keys.contains(&Keycode::LAlt)) {
                    // Key combination pressed for the first time
                    thread::sleep(std::time::Duration::from_millis(300));
                    if let Err(e) = app.emit_to(tauri::EventTarget::labeled("main"), "change_locale", crate::locales::keyboard_layouts::get_current_layout()) {
                        eprintln!("Error emitting event: {:?}", e);
                    }
                }
            }
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}