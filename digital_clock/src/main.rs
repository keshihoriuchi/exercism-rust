// #![no_main]
#[cfg(windows)]
extern crate winapi;
extern crate chrono;

use std::mem;
// use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null_mut;
use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use chrono::prelude::*;

unsafe extern "system" fn win_proc(
    hwnd: windef::HWND,
    msg: minwindef::UINT,
    w_param: minwindef::WPARAM,
    l_param: minwindef::LPARAM,
) -> minwindef::LRESULT {
    let hdc: windef::HDC;
    let mut ps = mem::zeroed::<PAINTSTRUCT>();
    let mut rect = mem::zeroed::<windef::RECT>();

    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_CREATE => {
            SetTimer(hwnd, 1, 100, None);
            0
        }
        WM_TIMER => {
            InvalidateRect(hwnd, null_mut(), minwindef::TRUE);
            0
        }
        WM_PAINT => {
            hdc = BeginPaint(hwnd, &mut ps);
            GetClientRect(hwnd, &mut rect);
            let local: DateTime<Local> = Local::now();
            DrawTextW(
                hdc,
                local.format("%H:%M:%S").to_string().to_unicode().as_ptr(),
                -1,
                &mut rect,
                DT_WORDBREAK | DT_CENTER,
            );
            EndPaint(hwnd, &mut ps);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

// #[allow(non_snake_case)]
// #[no_mangle]
// pub extern "system" fn WinMain(
//     _hInstance: *const c_void,
//     _hPrevInstance: *const c_void,
//     _lpCmdLine: *const c_char,
//     _nCmdShow: c_int,
// ) -> c_int {
pub fn main() {
    unsafe {
        println!("main");
        let class_name = "DIGITALCLOCKCLASS".to_unicode();
        let mut wc = mem::zeroed::<WNDCLASSW>();
        wc.style = CS_HREDRAW | CS_VREDRAW;
        wc.lpfnWndProc = Some(win_proc);
        // wc.cbClsExtra = 0;
        // wc.cbWndExtra = 0;
        // wc.hInstance = hInstance;
        wc.hIcon = LoadIconW(null_mut(), IDI_APPLICATION);
        wc.hCursor = LoadCursorW(null_mut(), IDC_ARROW);
        wc.hbrBackground = GetStockObject(WHITE_BRUSH as i32) as windef::HBRUSH;
        wc.lpszClassName = class_name.as_ptr();
        RegisterClassW(&wc);

        CreateWindowExW(
            0,
            class_name.as_ptr(),
            "デジタル時計".to_unicode().as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );

        let mut msg = mem::zeroed::<MSG>();
        loop {
            if GetMessageW(&mut msg, null_mut(), 0, 0) == 0 {
                return;
            }
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
}

trait Helper {
    fn to_unicode(&self) -> Vec<u16>;
}

impl Helper for str {
    fn to_unicode(&self) -> Vec<u16> {
        self.encode_utf16().chain(Some(0)).collect()
    }
}
