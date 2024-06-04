use crate::sys::imp::{self, WindowHandle};

pub trait EventHandler {
    fn starting(&mut self, event_loop: &ActiveEventLoop);

    fn events_arrived(&mut self, event_loop: &ActiveEventLoop, cause: StartCause);

    fn window_resize(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: WindowHandle,
        size: (u32, u32),
    ) {
        let _ = event_loop;
        let _ = window;
        let _ = size;
    }

    fn window_close(&mut self, event_loop: &ActiveEventLoop, window: WindowHandle) {
        let _ = event_loop;
        let _ = window;
    }

    fn events_finished(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
    }
}

#[derive(Default)]
pub struct EventLoop<'h> {
    pub(crate) inner: imp::EventLoop<'h>,
}

impl<'h> EventLoop<'h> {
    pub fn new() -> Self {
        Self {
            inner: imp::EventLoop::new(),
        }
    }

    pub fn run<H: EventHandler + 'h>(mut self, handler: H) {
        self.inner.run(handler)
    }

    pub fn target(&self) -> &ActiveEventLoop<'h> {
        &self.inner.target
    }
}

pub struct ActiveEventLoop<'h> {
    pub(crate) inner: imp::ActiveEventLoop<'h>,
}

impl<'h> ActiveEventLoop<'h> {
    pub fn set_control_flow(&self, control_flow: ControlFlow) {
        self.inner.set_control_flow(control_flow)
    }

    pub(crate) fn new() -> Self {
        Self {
            inner: imp::ActiveEventLoop::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartCause {
    Poll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    Poll,
    Exit,
}
