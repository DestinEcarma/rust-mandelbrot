pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub const MAX_ITER: u32 = 256;
pub const START_SCALE: f32 = 4.0;
pub const ZOOM_FACTOR: f32 = 1.1;
pub const ZOOM_SENSITIVITY: f32 = 1.0;

pub fn init_window() -> winit::window::WindowAttributes {
    winit::window::Window::default_attributes()
        .with_title("Fractals")
        .with_visible(false)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::NoUninit)]
pub struct Params {
    max_iter: u32,
    scale: f32,
    size: [f32; 2],
    center: [f32; 2],
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

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_size(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.size = [size.width as f32, size.height as f32];
    }

    pub fn set_center(&mut self, center: (f32, f32)) {
        self.center = [center.0, center.1];
    }
}
