use crate::sys::imp;

pub trait EventHandler {
    fn startup(&mut self, event_loop: &EventLoop);

    fn new_events(&mut self, event_loop: &EventLoop, cause: StartCause);

    fn finished_events(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }

    fn shutdown(&mut self, event_loop: &EventLoop) {
        let _ = event_loop;
    }
}

pub enum StartCause {
    Poll,
}

pub struct EventLoop<'a> {
    inner: imp::EventLoop<'a>,
}

impl<'a> EventLoop<'a> {
    pub fn run<H: EventHandler + 'a>(self, handler: H) {
        self.inner.run(handler)
    }
}
