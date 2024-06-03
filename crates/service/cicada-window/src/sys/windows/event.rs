use std::{cell::RefCell, mem::MaybeUninit};

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        DefWindowProcW, DispatchMessageW, GetWindowLongPtrW, PeekMessageW, SetWindowLongPtrW,
        TranslateMessage, CREATESTRUCTW, GWLP_USERDATA, PM_REMOVE, WM_NCCREATE, WM_SIZE,
    },
};

use crate::event::EventHandler;

pub(crate) struct EventLoop<'a> {
    pub(super) handler: Option<Box<RefCell<dyn EventHandler + 'a>>>,
}

impl<'a> EventLoop<'a> {
    pub(crate) fn run<H: EventHandler + 'a>(mut self, handler: H) {
        if let None = self.handler {
            self.handler = Some(Box::new(RefCell::new(handler)));
            self.run_event_loop();
            self.handler = None;
        }
    }
}

impl<'a> EventLoop<'a> {
    fn run_event_loop(&self) {
        loop {
            self.process_events();
        }
    }

    fn process_events(&self) {
        let mut msg = MaybeUninit::uninit();
        while unsafe { PeekMessageW(msg.as_mut_ptr(), None, 0, 0, PM_REMOVE) }.as_bool() {
            let msg = unsafe { msg.assume_init() };
            let _ = unsafe { TranslateMessage(&msg) };
            let _ = unsafe { DispatchMessageW(&msg) };
        }
    }
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
                        event_loop: create_info.event_loop,
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
}

fn inner_common_window_callback(
    hwnd: HWND,
    data: &mut WindowData,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

struct CreateData<'a> {
    event_loop: &'a EventLoop<'a>,
}

struct WindowData<'a> {
    event_loop: &'a EventLoop<'a>,
}
