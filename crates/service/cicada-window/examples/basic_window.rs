use cicada_window::{
    error::OsError,
    event::{ActiveEventLoop, ControlFlow, EventHandler, EventLoop, StartCause},
    window::{ShowStyle, Window, WindowAttributes, WindowHandle},
};

fn main() -> Result<(), OsError> {
    let event_loop = EventLoop::new();
    let app = Application {
        window: Window::new(&event_loop, WindowAttributes::default())?,
    };
    event_loop.run(app);

    Ok(())
}

struct Application {
    window: Window,
}

impl Application {}

impl EventHandler for Application {
    fn starting(&mut self, event_loop: &ActiveEventLoop) {
        let _ = event_loop;
        self.window.show(ShowStyle::Visible);
    }

    fn events_arrived(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let _ = cause;
        let _ = event_loop;
    }

    fn window_close(&mut self, event_loop: &ActiveEventLoop, _: WindowHandle) {
        event_loop.set_control_flow(ControlFlow::Exit);
    }
}
