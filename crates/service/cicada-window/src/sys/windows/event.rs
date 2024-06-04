use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell, RefMut},
    mem::MaybeUninit,
};

use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, PeekMessageW, TranslateMessage, PM_REMOVE,
};

use crate::event::{ActiveEventLoop as RootActiveEventLoop, ControlFlow, EventHandler};

pub(crate) struct EventLoop<'h> {
    pub(crate) target: Box<RootActiveEventLoop<'h>>,
}

impl<'h> Default for EventLoop<'h> {
    fn default() -> Self {
        Self {
            target: Box::new(RootActiveEventLoop::new()),
        }
    }
}

impl<'h> EventLoop<'h> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn run<H: EventHandler + 'h>(&mut self, handler: H) {
        if let Some(_) = self.handler() {
            return;
        }

        self.set_handler(handler);
        self.run_event_loop();
        self.unset_handler();
        println!("Finished");
    }

    pub(crate) fn target(&self) -> &ActiveEventLoop<'h> {
        &self.target.inner
    }

    pub(crate) fn target_mut(&mut self) -> &mut ActiveEventLoop<'h> {
        &mut self.target.inner
    }
}

impl<'h> EventLoop<'h> {
    fn set_handler<H: EventHandler + 'h>(&mut self, handler: H) {
        self.target_mut().handler = Some(Box::new(RefCell::new(handler)));
    }

    fn unset_handler(&mut self) {
        self.target_mut().handler = None;
    }

    fn handler(&self) -> Option<Ref<dyn EventHandler + 'h>> {
        self.target()
            .handler
            .as_ref()
            .map(|h| h.try_borrow().ok())?
    }

    fn handler_mut(&self) -> Option<RefMut<dyn EventHandler + 'h>> {
        self.target()
            .handler
            .as_ref()
            .map(|h| h.try_borrow_mut().ok())?
    }

    fn run_event_loop(&self) {
        self.startup();

        loop {
            if let Ok(borrow) = self.target().control_flow.try_borrow() {
                if let ControlFlow::Exit = *borrow {
                    break;
                }
            }

            self.process_events();
        }

        self.shutdown();
    }

    fn startup(&self) {
        if let Some(mut handler) = self.handler_mut() {
            handler.borrow_mut().starting(&self.target);
        }
    }

    fn shutdown(&self) {
        if let Some(mut handler) = self.handler_mut() {
            handler.borrow_mut().exiting(&self.target);
        }
    }
}

impl<'h> EventLoop<'h> {
    fn process_events(&self) {
        let mut msg = MaybeUninit::uninit();
        while unsafe { PeekMessageW(msg.as_mut_ptr(), None, 0, 0, PM_REMOVE) }.as_bool() {
            let msg = unsafe { msg.assume_init() };
            let _ = unsafe { TranslateMessage(&msg) };
            let _ = unsafe { DispatchMessageW(&msg) };
        }
    }
}

pub(crate) struct ActiveEventLoop<'h> {
    pub(super) handler: Option<Box<RefCell<dyn EventHandler + 'h>>>,
    pub(super) control_flow: RefCell<ControlFlow>,
}

impl<'h> ActiveEventLoop<'h> {
    pub(crate) fn new() -> Self {
        Self {
            handler: Default::default(),
            control_flow: RefCell::new(ControlFlow::Poll),
        }
    }

    pub(crate) fn set_control_flow(&self, control_flow: ControlFlow) {
        if let Ok(mut borrow) = self.control_flow.try_borrow_mut() {
            *borrow = control_flow;
        }
    }
}
