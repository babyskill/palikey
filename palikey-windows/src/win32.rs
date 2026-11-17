//! Raw Win32 FFI bindings for PaliKey Windows shell.
//!
//! Defined via std + direct extern declarations without third-party crate
//! dependencies, keeping compilation fast and the dependency tree minimal.

#![allow(non_snake_case, non_camel_case_types, dead_code)]

pub type HWND = *mut std::ffi::c_void;
pub type HHOOK = *mut std::ffi::c_void;
pub type HINSTANCE = *mut std::ffi::c_void;
pub type HMENU = *mut std::ffi::c_void;
pub type HICON = *mut std::ffi::c_void;
pub type HCURSOR = *mut std::ffi::c_void;
pub type HBRUSH = *mut std::ffi::c_void;
pub type HKL = *mut std::ffi::c_void;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type LRESULT = isize;
pub type BOOL = i32;

pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

// Hook constants
pub const WH_KEYBOARD_LL: i32 = 13;
pub const HC_ACTION: i32 = 0;

// Window and system messages
pub const WM_NULL: u32 = 0x0000;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_COMMAND: u32 = 0x0111;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_USER: u32 = 0x0400;

// Virtual keys
pub const VK_BACK: u32 = 0x08;
pub const VK_TAB: u32 = 0x09;
pub const VK_RETURN: u32 = 0x0D;
pub const VK_SHIFT: u32 = 0x10;
pub const VK_CONTROL: u32 = 0x11;
pub const VK_MENU: u32 = 0x12; // Alt key
pub const VK_CAPITAL: u32 = 0x14; // Caps Lock
pub const VK_ESCAPE: u32 = 0x1B;
pub const VK_SPACE: u32 = 0x20;
pub const VK_PRIOR: u32 = 0x21; // Page Up
pub const VK_NEXT: u32 = 0x22;  // Page Down
pub const VK_END: u32 = 0x23;
pub const VK_HOME: u32 = 0x24;
pub const VK_LEFT: u32 = 0x25;
pub const VK_UP: u32 = 0x26;
pub const VK_RIGHT: u32 = 0x27;
pub const VK_DOWN: u32 = 0x28;
pub const VK_DELETE: u32 = 0x2E;
pub const VK_LWIN: u32 = 0x5B;
pub const VK_RWIN: u32 = 0x5C;

// Low-level keyboard hook flags
pub const LLKHF_INJECTED: u32 = 0x00000010;

/// Magic extra-info value attached to synthetic SendInput keystrokes
/// to distinguish them from user hardware input and prevent hook recursion.
pub const PALIKEY_MAGIC_EXTRA_INFO: usize = 0x50414C49; // ASCII "PALI"

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct KBDLLHOOKSTRUCT {
    pub vkCode: u32,
    pub scanCode: u32,
    pub flags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

// SendInput constants and structures
pub const INPUT_KEYBOARD: u32 = 1;
pub const KEYEVENTF_KEYUP: u32 = 0x0002;
pub const KEYEVENTF_UNICODE: u32 = 0x0004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KEYBDINPUT {
    pub wVk: u16,
    pub wScan: u16,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union INPUT_UNION {
    pub ki: KEYBDINPUT,
    pub mi: MOUSEINPUT,
    pub hi: HARDWAREINPUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INPUT {
    pub r#type: u32,
    pub u: INPUT_UNION,
}

// Tray icon constants and structures
pub const NIM_ADD: u32 = 0;
pub const NIM_MODIFY: u32 = 1;
pub const NIM_DELETE: u32 = 2;

pub const NIF_MESSAGE: u32 = 0x00000001;
pub const NIF_ICON: u32 = 0x00000002;
pub const NIF_TIP: u32 = 0x00000004;

#[repr(C)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub uTimeoutOrVersion: u32,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: [u8; 16],
    pub hBalloonIcon: HICON,
}

// Menu flags
pub const MF_STRING: u32 = 0x00000000;
pub const MF_SEPARATOR: u32 = 0x00000800;
pub const MF_CHECKED: u32 = 0x00000008;
pub const MF_UNCHECKED: u32 = 0x00000000;
pub const MF_GRAYED: u32 = 0x00000001;
pub const TPM_RIGHTBUTTON: u32 = 0x0002;
pub const TPM_BOTTOMALIGN: u32 = 0x0020;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

pub type WNDPROC = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;
pub type HOOKPROC = unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT;

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cbSize: u32,
    pub style: u32,
    pub lpfnWndProc: Option<WNDPROC>,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: *const u16,
    pub lpszClassName: *const u16,
    pub hIconSm: HICON,
}

pub const IDI_APPLICATION: *const u16 = 32512 as *const u16;

#[cfg(windows)]
#[link(name = "user32")]
#[link(name = "shell32")]
#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: *const u16) -> HINSTANCE;
    pub fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
    pub fn CreateWindowExW(
        dwExStyle: u32,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: u32,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: *mut std::ffi::c_void,
    ) -> HWND;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn DefWindowProcW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: i32) -> ();
    pub fn PostMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: u32, wMsgFilterMax: u32) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: *const u16) -> HICON;

    pub fn SetWindowsHookExW(idHook: i32, lpfn: HOOKPROC, hmod: HINSTANCE, dwThreadId: u32) -> HHOOK;
    pub fn UnhookWindowsHookEx(hhk: HHOOK) -> BOOL;
    pub fn CallNextHookEx(hhk: HHOOK, nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

    pub fn GetForegroundWindow() -> HWND;
    pub fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: *mut u32) -> u32;
    pub fn GetKeyboardLayout(idThread: u32) -> HKL;
    pub fn GetKeyboardState(lpKeyState: *mut u8) -> BOOL;
    pub fn GetKeyState(nVirtKey: i32) -> i16;
    pub fn ToUnicodeEx(
        wVirtKey: u32,
        wScanCode: u32,
        lpKeyState: *const u8,
        pwszBuff: *mut u16,
        cchBuff: i32,
        wFlags: u32,
        dwhkl: HKL,
    ) -> i32;

    pub fn SendInput(cInputs: u32, pInputs: *mut INPUT, cbSize: i32) -> u32;

    pub fn Shell_NotifyIconW(dwMessage: u32, lpData: *const NOTIFYICONDATAW) -> BOOL;

    pub fn CreatePopupMenu() -> HMENU;
    pub fn AppendMenuW(hMenu: HMENU, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> BOOL;
    pub fn DestroyMenu(hMenu: HMENU) -> BOOL;
    pub fn TrackPopupMenu(
        hMenu: HMENU,
        uFlags: u32,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: HWND,
        prcRect: *const std::ffi::c_void,
    ) -> BOOL;
    pub fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    pub fn GetCursorPos(lpPoint: *mut POINT) -> BOOL;
}
