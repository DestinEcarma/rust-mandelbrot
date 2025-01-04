pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub const MAX_ITER: u32 = 256;

pub fn init_window() -> winit::window::WindowAttributes {
    winit::window::Window::default_attributes()
        .with_title("Fractals")
        .with_visible(false)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::NoUninit)]
pub struct Params {
    max_iter: u32,
    _padding: u32,
    size: [u32; 2],
}

impl Params {
    pub fn new() -> Self {
        Self {
            max_iter: MAX_ITER,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn with_max_iter(mut self, max_iter: u32) -> Self {
        self.max_iter = max_iter;
        self
    }

    pub fn with_size(mut self, size: winit::dpi::PhysicalSize<u32>) -> Self {
        self.size = [size.width, size.height];
        self
    }
}
