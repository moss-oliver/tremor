//Util includes
extern crate lib_foundry_common;
use lib_foundry_common::Platform;
use lib_foundry_common::PlatformEvent;
use lib_foundry_common::Window;
use lib_foundry_common::KeyboardKey;
use lib_foundry_common::PlatformError;
use lib_foundry_common::PlatformType;

//Windows includes
extern crate winapi;
extern crate user32;
extern crate gdi32;
extern crate kernel32;

use winapi::windef::HWND;
use winapi::windef::HMENU;
use winapi::windef::HBRUSH;
use winapi::minwindef::HINSTANCE;

use winapi::minwindef::UINT;
use winapi::minwindef::DWORD;
use winapi::minwindef::WPARAM;
use winapi::minwindef::LPARAM;
use winapi::minwindef::LRESULT;
use winapi::winnt::LPCWSTR;

use winapi::winuser::WS_OVERLAPPEDWINDOW;
use winapi::winuser::CW_USEDEFAULT;
use winapi::winuser::WS_VISIBLE;
use winapi::winuser::WNDCLASSW;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use std::ffi::{OsString};
use std::os::windows::ffi::{OsStringExt};
use std::path::{PathBuf};

use std::cell::RefCell;

pub struct PlatformWin32 {
    window_class_name: Vec<u16>,
    window_class: RefCell<Option<WNDCLASSW>>
}
impl PlatformWin32 {
    pub fn new() -> PlatformWin32 {
        PlatformWin32 {
            window_class_name: "foundry_window".to_wide_null(),
            window_class: RefCell::new(Option::None)
        }
    }
}
impl Platform for PlatformWin32 {
    type T_WIN = Win32Window;

    fn get_platform_type(&self) -> PlatformType {return PlatformType::Win32}
    fn get_platform_name(&self) -> &'static str {return "Windows"}

    fn create_window(&self, window_name:&str) -> Option<Win32Window> {
        //Create window class

        {
            let mut wnd_class = self.window_class.borrow_mut();
            if wnd_class.is_none()
            {
                //Load default icon and cursor
                let h_icon : *mut winapi::HICON__;
                let h_cursor : *mut winapi::HICON__;
                unsafe {
                    h_icon = user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION);
                    h_cursor = user32::LoadCursorW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION);
                }
                let wnd = WNDCLASSW {
                    style: 0,
                    lpfnWndProc: Some(window_proc), 
                    cbClsExtra: 0,
                    cbWndExtra: 0,
                    hInstance: 0 as HINSTANCE,
                    hIcon: h_icon,
                    hCursor: h_cursor,
                    hbrBackground: 16 as HBRUSH,
                    lpszMenuName: 0 as LPCWSTR,
                    lpszClassName: self.window_class_name.as_ptr() as *const u16,
                };
                unsafe {
                    // We register our class -
                    if 0 == user32::RegisterClassW(&wnd)
                    {
                        println!("Window error: {0}", "Failed to register window");
                        return Option::None;
                    }
                }
                *wnd_class = Option::Some(wnd);
            }
        }

        let win = Win32Window::new(self.window_class.borrow().unwrap(),window_name,200,200);
        match win {
            Ok(expr) => {return Option::Some(expr)},
            Err(err) => {println!("Window error: {0}", err.text); return Option::None}
        };
    }
}

#[derive(Debug)]
pub enum WindowCreateErrorCode {
    RegisterError,
    CreateError,
    UpdateError
}

pub struct Win32Window {
    window_data: *mut winapi::HWND__
}
impl Window for Win32Window {

}

impl Win32Window {
    fn new(window_class:WNDCLASSW,window_name:&str, w:i32,h:i32) -> Result<Win32Window,PlatformError<WindowCreateErrorCode>> {
        let hwnd : *mut winapi::HWND__;

        unsafe {
            hwnd = user32::CreateWindowExW(0, window_class.lpszClassName, 
                            window_name.to_wide_null().as_ptr() as *mut _, WS_OVERLAPPEDWINDOW | WS_VISIBLE, 
                            CW_USEDEFAULT, CW_USEDEFAULT, w, h,
                            0 as HWND, 0 as HMENU, 0 as HINSTANCE, std::ptr::null_mut());

            if hwnd == std::ptr::null_mut()
            {
                let errorcode = kernel32::GetLastError();
                println!("Failed to create window: {:?}",errorcode);
                return Result::Err(PlatformError::new(WindowCreateErrorCode::CreateError, "Failed to create window"));
            }

            user32::ShowWindow(hwnd,
                winapi::winuser::SW_RESTORE);
            if user32::UpdateWindow(hwnd) == 0
            {
                return Result::Err(PlatformError::new(WindowCreateErrorCode::UpdateError, "Failed to update window"));
            }

            user32::InvalidateRect(hwnd, std::ptr::null_mut(),0);
            
        };
        let win32_window = Win32Window {window_data:hwnd};
        return Result::Ok(win32_window);
    }
}

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}
impl<T> ToWide for T where T: AsRef<OsStr> {
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}
pub trait FromWide where Self: Sized {
    fn from_wide(wide: &[u16]) -> Self;
    fn from_wide_null(wide: &[u16]) -> Self {
        let len = wide.iter().take_while(|&&c| c != 0).count();
        Self::from_wide(&wide[..len])
    }
}
impl FromWide for OsString {
    fn from_wide(wide: &[u16]) -> OsString {
        OsStringExt::from_wide(wide)
    }
}
impl FromWide for PathBuf {
    fn from_wide(wide: &[u16]) -> PathBuf {
        <OsString as OsStringExt>::from_wide(wide).into()
    }
}

unsafe extern "system" fn window_proc(h_wnd :HWND, 
	msg :UINT, w_param :WPARAM, l_param :LPARAM) -> LRESULT
{
    if msg == winapi::winuser::WM_DESTROY
    || msg == winapi::winuser::WM_CLOSE
    || msg == winapi::winuser::WM_QUIT {
        user32::PostQuitMessage(0);
    }
    return user32::DefWindowProcW(h_wnd, msg, w_param, l_param);
}

pub fn get_win32_event(window:&Win32Window) -> Option<PlatformEvent> {

    let is_visible : bool;
    unsafe {
        is_visible = user32::IsWindowVisible(window.window_data) != 0;
    }
    if !is_visible
    {
        return Option::Some(PlatformEvent::WindowClose);
    }

    let mut msg = winapi::winuser::MSG {
        hwnd : 0 as HWND,
        message : 0 as UINT,
        wParam : 0 as WPARAM,
        lParam : 0 as LPARAM,
        time : 0 as DWORD,
        pt : winapi::windef::POINT { x: 0, y: 0, },
    };

    let msg_id : UINT;
    unsafe {
        user32::PeekMessageW(&mut msg,window.window_data,0,0,winapi::winuser::PM_REMOVE);
        user32::TranslateMessage(&mut msg);
        user32::DispatchMessageW(&mut msg);
        msg_id = msg.message;
    }
    if msg_id == 0
    {
        return Option::None;
    }
    //println!("Got Event: {0}",msg_id);
    if msg_id == winapi::winuser::WM_DESTROY
    || msg_id == winapi::winuser::WM_CLOSE
    || msg_id == winapi::winuser::WM_QUIT
    {
        return Option::Some(PlatformEvent::WindowClose);
    }

    //keydown
    if msg_id == 256 || msg_id == 257 {
        let keyboard : KeyboardKey;
        match msg.wParam {
            9 => {keyboard = KeyboardKey::Tab},
            13 => {keyboard = KeyboardKey::Enter},

            37 => {keyboard = KeyboardKey::Left},
            38 => {keyboard = KeyboardKey::Up},
            39 => {keyboard = KeyboardKey::Right},
            40 => {keyboard = KeyboardKey::Down},

            65 => {keyboard = KeyboardKey::A},
            66 => {keyboard = KeyboardKey::B},
            67 => {keyboard = KeyboardKey::C},
            68 => {keyboard = KeyboardKey::D},
            69 => {keyboard = KeyboardKey::E},
            70 => {keyboard = KeyboardKey::F},
            71 => {keyboard = KeyboardKey::G},
            72 => {keyboard = KeyboardKey::H},
            73 => {keyboard = KeyboardKey::I},
            74 => {keyboard = KeyboardKey::J},
            75 => {keyboard = KeyboardKey::K},
            76 => {keyboard = KeyboardKey::L},
            77 => {keyboard = KeyboardKey::M},
            78 => {keyboard = KeyboardKey::N},
            79 => {keyboard = KeyboardKey::O},
            80 => {keyboard = KeyboardKey::P},
            81 => {keyboard = KeyboardKey::Q},
            82 => {keyboard = KeyboardKey::R},
            83 => {keyboard = KeyboardKey::S},
            84 => {keyboard = KeyboardKey::T},
            85 => {keyboard = KeyboardKey::U},
            86 => {keyboard = KeyboardKey::V},
            87 => {keyboard = KeyboardKey::W},
            88 => {keyboard = KeyboardKey::X},
            89 => {keyboard = KeyboardKey::Y},
            90 => {keyboard = KeyboardKey::Z},
            _ => {keyboard = KeyboardKey::Unknown}
        }
        
        if msg_id == 256 {
            return Option::Some(PlatformEvent::KeyboardKeydown{key:keyboard});
        } else {
            return Option::Some(PlatformEvent::KeyboardKeyup{key:keyboard});
        }
    }

    return Option::None;
}

pub fn win32_draw_bmp
(window: &Win32Window, width:u32, height:u32, bmp: *const u8) {
    unsafe {

        let bitmapinfo = winapi::BITMAPINFO {
            bmiHeader: winapi::BITMAPINFOHEADER {
                biSize:std::mem::size_of::<winapi::BITMAPINFOHEADER>() as u32,
                biWidth:width as i32,
                biHeight:-(height as i32),
                biPlanes:1,
                biBitCount:32,
                biCompression:winapi::BI_RGB,
                biSizeImage:0,
                biXPelsPerMeter:0,
                biYPelsPerMeter:0,
                biClrUsed:0,
                biClrImportant:0
            },
            bmiColors:[winapi::RGBQUAD{rgbRed:0,rgbBlue:0,rgbGreen:0,rgbReserved:0}; 0]
        };
        let mut paintstr = winapi::PAINTSTRUCT{
            rcPaint:winapi::RECT{left:0,right:256,top:0,bottom:256},
            fRestore:0,
            hdc:std::ptr::null_mut(),
            fErase:0,
            rgbReserved:[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            fIncUpdate:0
        };
        let paint = &mut paintstr as *mut winapi::PAINTSTRUCT;

        let mut rect = winapi::windef::RECT{top:0,left:0,right:0,bottom:0};
        user32::GetClientRect(window.window_data, &mut rect as winapi::windef::LPRECT);

        user32::InvalidateRect(window.window_data, std::ptr::null_mut(),0);

        let device = user32::BeginPaint(window.window_data,paint);
        gdi32::StretchDIBits(device, 0, 0, rect.right, rect.bottom, 0, 0, width as i32, height as i32
            , bmp as *const winapi::c_void
            , &bitmapinfo,winapi::DIB_RGB_COLORS,winapi::SRCCOPY);

        user32::EndPaint(window.window_data, paint);
    }
}
