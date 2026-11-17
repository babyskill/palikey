//! PaliKey Windows Shell — System Tray Application.
//!
//! Entry point, Win32 message pump, and system tray menu.
//! Links `palikey-engine` via its clean Rust API.

#![cfg_attr(not(windows), allow(dead_code, unused_imports))]

mod config;
mod hook;
mod win32;

#[cfg(windows)]
use std::sync::Mutex;

#[cfg(windows)]
use crate::config::{Config, NiggahitaConfig, SchemeConfig};
#[cfg(windows)]
use crate::win32::*;

#[cfg(windows)]
const WM_TRAYICON: u32 = WM_USER + 1;
#[cfg(windows)]
const TRAY_UID: u32 = 1;

#[cfg(windows)]
const IDM_TOGGLE_ENABLED: usize = 1001;
#[cfg(windows)]
const IDM_SCHEME_VELTHUIS: usize = 1002;
#[cfg(windows)]
const IDM_SCHEME_PALITELEX: usize = 1003;
#[cfg(windows)]
const IDM_SCHEME_SIL: usize = 1004;
#[cfg(windows)]
const IDM_NIGGAHITA_DOT_BELOW: usize = 1005;
#[cfg(windows)]
const IDM_NIGGAHITA_DOT_ABOVE: usize = 1006;
#[cfg(windows)]
const IDM_TOGGLE_EXTENDED: usize = 1007;
#[cfg(windows)]
const IDM_EXIT: usize = 1099;

#[cfg(windows)]
static GLOBAL_CONFIG: Mutex<Option<Config>> = Mutex::new(None);

#[cfg(windows)]
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_TRAYICON => {
            let event = l_param as u32;
            if event == WM_RBUTTONUP || event == WM_LBUTTONUP {
                show_tray_menu(hwnd);
            }
            0
        }
        WM_COMMAND => {
            handle_menu_command(hwnd, w_param as usize);
            0
        }
        WM_DESTROY => {
            hook::uninstall();
            let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = TRAY_UID;
            Shell_NotifyIconW(NIM_DELETE, &nid);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

#[cfg(windows)]
unsafe fn show_tray_menu(hwnd: HWND) {
    let hmenu = CreatePopupMenu();
    if hmenu.is_null() {
        return;
    }

    let guard = GLOBAL_CONFIG.lock().unwrap_or_else(|e| e.into_inner());
    let cfg = guard.clone().unwrap_or_default();
    let enabled = hook::is_enabled();

    let append_item = |flags: u32, id: usize, text: &str| {
        let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        AppendMenuW(hmenu, flags, id, wide.as_ptr());
    };

    let append_separator = || {
        AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());
    };

    // 1. Enable / Disable toggle
    let enabled_flag = if enabled { MF_CHECKED } else { MF_UNCHECKED };
    append_item(
        enabled_flag | MF_STRING,
        IDM_TOGGLE_ENABLED,
        "Enabled (Pāḷi Input)",
    );

    append_separator();

    // 2. Scheme selection
    let vel_flag = if cfg.scheme == SchemeConfig::Velthuis {
        MF_CHECKED
    } else {
        MF_UNCHECKED
    };
    append_item(
        vel_flag | MF_STRING,
        IDM_SCHEME_VELTHUIS,
        "Velthuis (aa->ā, .t->ṭ, .m->ṃ)",
    );

    let telex_flag = if cfg.scheme == SchemeConfig::Palitelex {
        MF_CHECKED
    } else {
        MF_UNCHECKED
    };
    append_item(
        telex_flag | MF_STRING,
        IDM_SCHEME_PALITELEX,
        "PaliTelex (aa->ā, tx->ṭ, ngw->ṅ)",
    );

    let sil_flag = if cfg.scheme == SchemeConfig::Sil {
        MF_CHECKED
    } else {
        MF_UNCHECKED
    };
    append_item(
        sil_flag | MF_STRING,
        IDM_SCHEME_SIL,
        "SIL / Keyman (-a->ā, .t->ṭ, >n->ṅ)",
    );

    append_separator();

    // 3. Niggahita variants
    let nig_below_flag = if cfg.niggahita == NiggahitaConfig::DotBelow {
        MF_CHECKED
    } else {
        MF_UNCHECKED
    };
    append_item(
        nig_below_flag | MF_STRING,
        IDM_NIGGAHITA_DOT_BELOW,
        "Niggahīta: ṃ (U+1E43 Dot Below - PTS)",
    );

    let nig_above_flag = if cfg.niggahita == NiggahitaConfig::DotAbove {
        MF_CHECKED
    } else {
        MF_UNCHECKED
    };
    append_item(
        nig_above_flag | MF_STRING,
        IDM_NIGGAHITA_DOT_ABOVE,
        "Niggahīta: ṁ (U+1E41 Dot Above - VRI)",
    );

    append_separator();

    // 4. Extended Sanskrit / IAST toggle
    let ext_flag = if cfg.extended_set {
        MF_CHECKED
    } else {
        MF_UNCHECKED
    };
    append_item(
        ext_flag | MF_STRING,
        IDM_TOGGLE_EXTENDED,
        "Extended Sanskrit / IAST (ṛ, ṝ, ḥ, ś, ṣ)",
    );

    append_separator();

    // 5. Exit
    append_item(MF_STRING, IDM_EXIT, "Exit PaliKey");

    let mut pt = POINT { x: 0, y: 0 };
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);
    TrackPopupMenu(
        hmenu,
        TPM_RIGHTBUTTON | TPM_BOTTOMALIGN,
        pt.x,
        pt.y,
        0,
        hwnd,
        std::ptr::null(),
    );
    PostMessageW(hwnd, WM_NULL, 0, 0);
    DestroyMenu(hmenu);
}

#[cfg(windows)]
unsafe fn handle_menu_command(hwnd: HWND, cmd_id: usize) {
    let mut guard = GLOBAL_CONFIG.lock().unwrap_or_else(|e| e.into_inner());
    let mut cfg = guard.clone().unwrap_or_default();

    match cmd_id {
        IDM_TOGGLE_ENABLED => {
            let next_state = !hook::is_enabled();
            hook::set_enabled(next_state);
        }
        IDM_SCHEME_VELTHUIS => {
            cfg.scheme = SchemeConfig::Velthuis;
            hook::update_config(&cfg);
            let _ = cfg.save();
        }
        IDM_SCHEME_PALITELEX => {
            cfg.scheme = SchemeConfig::Palitelex;
            hook::update_config(&cfg);
            let _ = cfg.save();
        }
        IDM_SCHEME_SIL => {
            cfg.scheme = SchemeConfig::Sil;
            hook::update_config(&cfg);
            let _ = cfg.save();
        }
        IDM_NIGGAHITA_DOT_BELOW => {
            cfg.niggahita = NiggahitaConfig::DotBelow;
            hook::update_config(&cfg);
            let _ = cfg.save();
        }
        IDM_NIGGAHITA_DOT_ABOVE => {
            cfg.niggahita = NiggahitaConfig::DotAbove;
            hook::update_config(&cfg);
            let _ = cfg.save();
        }
        IDM_TOGGLE_EXTENDED => {
            cfg.extended_set = !cfg.extended_set;
            hook::update_config(&cfg);
            let _ = cfg.save();
        }
        IDM_EXIT => {
            DestroyWindow(hwnd);
            return;
        }
        _ => {}
    }

    *guard = Some(cfg);
}

#[cfg(windows)]
fn windows_main() {
    let config = Config::load();
    {
        let mut guard = GLOBAL_CONFIG.lock().unwrap_or_else(|e| e.into_inner());
        *guard = Some(config.clone());
    }

    // Install WH_KEYBOARD_LL hook
    if !hook::install(&config) {
        eprintln!("Failed to install low-level keyboard hook.");
        return;
    }

    unsafe {
        let h_instance = GetModuleHandleW(std::ptr::null());
        let class_name: Vec<u16> = "PaliKeyMessageWindowClass\0".encode_utf16().collect();
        let window_title: Vec<u16> = "PaliKey\0".encode_utf16().collect();

        let mut wc: WNDCLASSEXW = std::mem::zeroed();
        wc.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wc.lpfnWndProc = Some(window_proc);
        wc.hInstance = h_instance;
        wc.lpszClassName = class_name.as_ptr();

        RegisterClassExW(&wc);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_title.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            eprintln!("Failed to create PaliKey message window.");
            hook::uninstall();
            return;
        }

        // Setup notification tray icon
        let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = TRAY_UID;
        nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = LoadIconW(std::ptr::null_mut(), IDI_APPLICATION);

        let tip = "PaliKey — Pāḷi Input Method";
        let tip_utf16: Vec<u16> = tip.encode_utf16().collect();
        let copy_len = tip_utf16.len().min(127);
        nid.szTip[..copy_len].copy_from_slice(&tip_utf16[..copy_len]);

        Shell_NotifyIconW(NIM_ADD, &nid);

        // Win32 message pump
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn main() {
    #[cfg(windows)]
    {
        windows_main();
    }

    #[cfg(not(windows))]
    {
        println!("PaliKey Windows Shell (Cross-Compilation Check)");
        println!("This crate is designed for Windows and requires Win32 APIs.");
        println!("To typecheck on macOS/Linux: cargo check --target x86_64-pc-windows-msvc");
    }
}
