use windows::Win32::UI::Input::KeyboardAndMouse::{
    ActivateKeyboardLayout, GetKeyboardLayoutList, GetKeyboardLayoutNameA, KLF_SETFORPROCESS
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId,
};

use winapi::um::winuser::GetKeyboardLayout;
use winapi::shared::minwindef::DWORD;
use windows::Win32::UI::TextServices::HKL;

use crate::locales;

#[tauri::command(rename_all = "snake_case")]
pub fn get_layouts() -> Vec<String> {
    let mut hkls = [HKL::default(); 16];
    let len = unsafe { GetKeyboardLayoutList(Some(&mut hkls)) } as usize;
    let hkls = &hkls[..len];
    let mut locales = Vec::new();
    for hkl in hkls.iter().cloned() {
        let id = unsafe {
            let mut buf = [0; 9];
            ActivateKeyboardLayout(hkl, KLF_SETFORPROCESS).ok().unwrap();
            GetKeyboardLayoutNameA(&mut buf).ok().unwrap();
            u32::from_str_radix(std::str::from_utf8_unchecked(&buf[..8]), 16)
                .ok()
                .unwrap()
        };
        locales.push(locales::keyboard_codes::locale_from_u16(format!(
            "{id:08X?}"
        )));
    }
    locales
}

pub fn get_current_layout() -> String {
    let locale_id = unsafe { 
        let fgw = GetForegroundWindow();
        let lpdwprocessid: u32 = 0;
        let psid = GetWindowThreadProcessId(fgw, Some((&lpdwprocessid as *const u32) as *mut u32));
        let hkl = GetKeyboardLayout(psid);
        let locale_id = (hkl as DWORD) & 0xFFFF;
        locale_id
    };

    locales::keyboard_codes::locale_from_u16(format!(
        "{locale_id:08X?}"
    ))
}
