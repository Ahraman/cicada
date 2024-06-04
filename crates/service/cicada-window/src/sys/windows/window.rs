use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{
            GetLastError, ERROR_CLASS_ALREADY_EXISTS, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM,
        },
        Graphics::Gdi::{GetStockObject, BLACK_BRUSH, HBRUSH},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, GetWindowLongPtrW, LoadCursorW,
            LoadIconW, RegisterClassExW, SetWindowLongPtrW, ShowWindow, CREATESTRUCTW, CS_HREDRAW,
            CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, IDI_APPLICATION, SW_HIDE,
            SW_MAXIMIZE, SW_MINIMIZE, SW_SHOW, WINDOW_STYLE, WM_DESTROY, WM_NCCREATE, WM_SIZE,
            WNDCLASSEXW, WS_MAXIMIZE, WS_MINIMIZE, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

use crate::{
    event::{ActiveEventLoop as RootActiveEventLoop, EventLoop as RootEventLoop},
    window::{DisplayStyle, Position, ShowStyle, Size, WindowAttributes},
};

use super::{
    util::{hiword, loword, WideStr},
    OsError,
};

pub type WindowHandle = HWND;

pub(crate) struct Window {
    hwnd: WindowHandle,
}

impl Window {
    pub(crate) fn new(
        event_loop: &RootEventLoop,
        attributes: WindowAttributes,
    ) -> Result<Self, OsError> {
        Ok(unsafe { Self::create_window(event_loop, attributes) }?)
    }

    pub(crate) fn show(&self, show_style: ShowStyle) {
        unsafe { Self::show_window(self.hwnd, show_style) }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { Self::destroy_window(self.hwnd) };
        self.hwnd = HWND::default();
    }
}

impl Window {
    pub(super) unsafe fn create_window(
        event_loop: &RootEventLoop,
        attributes: WindowAttributes,
    ) -> Result<Self, OsError> {
        let hinstance = unsafe { GetModuleHandleW(None) }?;

        let class_name = unsafe {
            Self::register_window_class(
                hinstance.into(),
                &attributes
                    .platform_specific
                    .class_name
                    .unwrap_or("common_window".into()),
            )
        }?;

        let window_name = WideStr::from_str(attributes.title.unwrap_or("Main Window".into()));
        let Position { x, y } = attributes.pos.unwrap_or(Position {
            x: CW_USEDEFAULT,
            y: CW_USEDEFAULT,
        });
        let Size { width, height } = attributes.size.unwrap_or(Size {
            width: CW_USEDEFAULT as u32,
            height: CW_USEDEFAULT as u32,
        });

        let mut create_data = CreateData { event_loop };

        let window_style = {
            let window_display_style = match attributes.display_style {
                DisplayStyle::Windowed { resizable: true } => WS_OVERLAPPEDWINDOW,
                DisplayStyle::Windowed { resizable: false } => WS_OVERLAPPEDWINDOW,
                DisplayStyle::ExclusiveFullscreen => todo!(),
                DisplayStyle::BorderlessWindow => todo!(),
            };

            let window_show_style = match attributes.show_style {
                ShowStyle::Hidden => WINDOW_STYLE::default(),
                ShowStyle::Visible => WS_VISIBLE,
                ShowStyle::Minimized => WS_MINIMIZE,
                ShowStyle::Maximized => WS_MAXIMIZE,
            };

            window_display_style | window_show_style
        };

        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                class_name.as_pcwstr(),
                window_name.as_pcwstr(),
                window_style,
                x,
                y,
                width as i32,
                height as i32,
                None,
                None,
                hinstance,
                Some(&mut create_data as *mut _ as *mut _),
            )
        };

        Ok(Self { hwnd })
    }

    pub(super) unsafe fn register_window_class(
        hinstance: HINSTANCE,
        name: &str,
    ) -> Result<WideStr, OsError> {
        let class_name = WideStr::from_str(name);

        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(common_window_callback),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: unsafe { LoadIconW(None, IDI_APPLICATION) }?,
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }?,
            hbrBackground: HBRUSH(unsafe { GetStockObject(BLACK_BRUSH) }.0),
            lpszMenuName: PCWSTR::null(),
            lpszClassName: class_name.as_pcwstr(),
            hIconSm: unsafe { LoadIconW(None, IDI_APPLICATION) }?,
        };

        if unsafe { RegisterClassExW(&wnd_class) } == 0 {
            let error = unsafe { GetLastError() };
            if error.is_err() && error != ERROR_CLASS_ALREADY_EXISTS {
                return Err(error.into());
            }
        }

        Ok(class_name)
    }

    pub(super) unsafe fn show_window(hwnd: HWND, show_style: ShowStyle) {
        let cmd_show = match show_style {
            ShowStyle::Hidden => SW_HIDE,
            ShowStyle::Visible => SW_SHOW,
            ShowStyle::Minimized => SW_MINIMIZE,
            ShowStyle::Maximized => SW_MAXIMIZE,
        };
        let _ = unsafe { ShowWindow(hwnd, cmd_show) };
    }

    pub(super) unsafe fn destroy_window(hwnd: HWND) {
        let _ = unsafe { DestroyWindow(hwnd) };
    }
}

#[derive(Debug, Clone, Default)]
pub struct WindowPlatformSpecificAttributes {
    pub class_name: Option<String>,
}

pub(super) struct CreateData<'r, 'h: 'r> {
    pub(super) event_loop: &'r RootEventLoop<'h>,
}

pub(super) struct WindowData<'r, 'h: 'r> {
    pub(super) event_loop: &'r RootActiveEventLoop<'h>,
}

pub(super) extern "system" fn common_window_callback(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let userdata = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
    let data = match (msg, userdata) {
        (WM_NCCREATE, 0) => {
            if let Some(create_struct) = unsafe { (lparam.0 as *mut CREATESTRUCTW).as_mut() } {
                if let Some(create_info) =
                    unsafe { (create_struct.lpCreateParams as *mut CreateData).as_mut() }
                {
                    let data = Box::new(WindowData {
                        event_loop: &create_info.event_loop.target(),
                    });
                    let data_ptr = Box::into_raw(data);
                    let _ = unsafe {
                        SetWindowLongPtrW(hwnd, GWLP_USERDATA, data_ptr as _);
                    };
                }
            }

            return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
        }
        (_, userdata) => {
            let data_ptr = userdata as *mut WindowData;
            if let Some(data) = unsafe { data_ptr.as_mut() } {
                data
            } else {
                return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
            }
        }
    };

    inner_common_window_callback(hwnd, data, msg, wparam, lparam)
        .unwrap_or_else(|| unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) })
}

pub(super) fn inner_common_window_callback(
    hwnd: HWND,
    data: &mut WindowData,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> Option<LRESULT> {
    let mut handler = data
        .event_loop
        .inner
        .handler
        .as_ref()?
        .try_borrow_mut()
        .ok()?;

    Some(match msg {
        WM_SIZE => {
            let (width, height) = (
                loword(lparam.0 as u32) as u32,
                hiword(lparam.0 as u32) as u32,
            );
            handler.window_resize(data.event_loop, hwnd, (width, height));
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
        WM_DESTROY => {
            handler.window_close(data.event_loop, hwnd);
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
        _ => return None,
    })
}
