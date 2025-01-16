pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub const MAX_ITER: u32 = 1000;
pub const START_SCALE: f64 = 4.0;
pub const ZOOM_FACTOR: f64 = 1.1;
pub const ZOOM_SENSITIVITY: f64 = 1.0;

pub fn init_window() -> winit::window::WindowAttributes {
    winit::window::Window::default_attributes()
        .with_title("Fractals")
        .with_visible(false)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::NoUninit)]
pub struct Params {
    max_iter: u32,
    _padding: [u32; 3],
    scale: f64,
    size: [u32; 2],
    center: [f64; 2],
}

impl Params {
    pub fn new() -> Self {
        Self {
            max_iter: MAX_ITER,
            scale: START_SCALE,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn set_max_iter(&mut self, max_iter: u32) {
        self.max_iter = max_iter;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn set_size(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.size = [size.width, size.height];
    }

    pub fn set_center(&mut self, center: (f64, f64)) {
        self.center = [center.0, center.1];
    }
}
