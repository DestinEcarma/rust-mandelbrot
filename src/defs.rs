pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub fn init_window() -> winit::window::WindowAttributes {
    winit::window::Window::default_attributes()
        .with_title("Fractals")
        .with_visible(false)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::NoUninit)]
pub struct Params {
    pub max_iter: u32,
    pub width: u32,
    pub height: u32,
}
