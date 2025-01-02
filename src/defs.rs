pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub const BOUNDS: (f64, f64, f64, f64) = (-2.0, 1.0, -1.5, 1.5);

pub fn init_window() -> winit::window::WindowAttributes {
    winit::window::Window::default_attributes()
        .with_title("Fractals")
        .with_visible(false)
}
