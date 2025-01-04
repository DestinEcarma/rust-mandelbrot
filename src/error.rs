use derive_more::derive::From;
use std::fmt;

#[derive(Debug, From)]
pub enum Error {
    /// An error that may be generated when no window is found
    NoWindow,
    /// An error that may be generated when no pixels are found
    NoPixels,
    /// An error that may be generated when no render pipeline is found
    NoRenderPipeline,
    /// An error that may be generated when no vertex buffer is found
    NoUniformBuffer,
    /// An error that may be generated when no diffuse bind group is found
    NoBindGroup,

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
    /// An error that may be generated when setting the logger
    SetLogger(log::SetLoggerError),
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
            Error::NoWindow => write!(f, "No window field found"),
            Error::NoPixels => write!(f, "No pixels field found"),
            Error::NoRenderPipeline => write!(f, "No render_pipeline field found"),
            Error::NoUniformBuffer => write!(f, "No uniform_buffer field found"),
            Error::NoBindGroup => write!(f, "No bind_group field found"),
            Error::Box(e) => write!(f, "{e:?}"),
            Error::Pixels(e) => e.fmt(f),
            Error::Winit(e) => e.fmt(f),
            Error::EventLoop(e) => e.fmt(f),
            Error::Texture(e) => e.fmt(f),
            Error::SetLogger(e) => e.fmt(f),
        }
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::error::Error for Error {}
