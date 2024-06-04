use crate::{error::OsError, event::EventLoop, sys::imp};

pub type WindowHandle = imp::WindowHandle;

pub struct Window {
    inner: imp::Window,
}

impl Window {
    pub fn new(event_loop: &EventLoop, attributes: WindowAttributes) -> Result<Self, OsError> {
        Ok(Self {
            inner: imp::Window::new(event_loop, attributes)?,
        })
    }

    pub fn show(&self, show_style: ShowStyle) {
        self.inner.show(show_style)
    }
}

#[derive(Debug, Clone)]
pub struct WindowAttributes {
    pub size: Option<Size>,
    pub pos: Option<Position>,
    pub title: Option<String>,
    pub show_style: ShowStyle,
    pub display_style: DisplayStyle,
    pub platform_specific: WindowPlatformSpecificAttributes,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            size: Default::default(),
            pos: Default::default(),
            title: Default::default(),
            show_style: ShowStyle::Hidden,
            display_style: DisplayStyle::Windowed { resizable: true },
            platform_specific: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowStyle {
    Hidden,
    Visible,
    Minimized,
    Maximized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayStyle {
    Windowed { resizable: bool },
    ExclusiveFullscreen,
    BorderlessWindow,
}

pub type WindowPlatformSpecificAttributes = imp::WindowPlatformSpecificAttributes;
