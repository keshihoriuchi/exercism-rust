#![no_main]
#[cfg(windows)]
extern crate winapi;
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef;
use winapi::shared::windef;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

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
        WM_PAINT => {
            hdc = BeginPaint(hwnd, &mut ps);
            GetClientRect(hwnd, &mut rect);
            DrawTextW(
                hdc,
                "Sample Text".to_unicode().as_ptr(),
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

// #[cfg(windows)]
// fn print_message(msg: &str) -> Result<i32, Error> {
//     let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
//     let ret = unsafe { MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK) };
//     if ret == 0 {
//         Err(Error::last_os_error())
//     } else {
//         Ok(ret)
//     }
// }

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(
    _hInstance: *const c_void,
    _hPrevInstance: *const c_void,
    _lpCmdLine: *const c_char,
    _nCmdShow: c_int,
) -> c_int {
    unsafe {
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

        let hwnd = CreateWindowExW(
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

        let mut msg = mem::uninitialized::<MSG>();
        loop {
            if GetMessageW(&mut msg, null_mut(), 0, 0) == 0 {
                return 0;
            }
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
    // print_message("Hello, world!").unwrap();
}

trait Helper {
    fn to_unicode(&self) -> Vec<u16>;
}

impl Helper for str {
    fn to_unicode(&self) -> Vec<u16> {
        self.encode_utf16().chain(Some(0)).collect()
    }
}
