use derive_more::derive::From;
use std::fmt;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    /// An error that may be generated when requesting Winit state
    Pixels(pixels::Error),
    #[from]
    /// A general error that may occur while running the Winit event loop
    Winit(winit::error::OsError),
    #[from]
    /// An error that may be generated when setting the Texture width and height
    Texture(pixels::TextureError),
    #[from]
    /// A general error that may occur while running the Winit event loop
    EventLoop(winit::error::EventLoopError),
    #[from]
    /// An error that may be generated when requesting form any type
    Box(Box<dyn std::any::Any + Send + 'static>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Box(e) => write!(f, "{e:?}"),
            Error::Pixels(e) => e.fmt(f),
            Error::Winit(e) => e.fmt(f),
            Error::EventLoop(e) => e.fmt(f),
            Error::Texture(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
