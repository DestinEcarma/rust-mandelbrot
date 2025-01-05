use crate::defs;

#[derive(Debug)]
pub struct Camera {
    /// The zoom level of the camera.
    pub scale: f32,
    /// The size of the window.
    pub size: (f32, f32),
    /// The current position of the camera.
    pub world_position: (f32, f32),
    /// The current position of the mouse.
    pub mouse_position: (f32, f32),
}

impl Camera {
    /// Create a new camera with the given window size.
    pub fn new(size: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            scale: defs::START_SCALE,
            size: (size.width as f32, size.height as f32),
            world_position: Default::default(),
            mouse_position: Default::default(),
        }
    }
}

impl Camera {
    /// Get the position of the mouse in world coordinates.
    pub fn mouse_world_position(&self) -> (f32, f32) {
        let x = self.mouse_position.0 / self.size.0 - 0.5;
        let y = self.mouse_position.1 / self.size.1 - 0.5;

        (
            self.world_position.0 + x * self.scale * self.size.0 / self.size.1,
            self.world_position.1 + y * self.scale,
        )
    }

    /// Move the camera by the given delta.
    pub fn zoom(&mut self, delta: f32) {
        let (world_x, world_y) = self.mouse_world_position();

        self.scale *= defs::ZOOM_FACTOR.powf(-delta * defs::ZOOM_SENSITIVITY);

        let (new_world_x, new_world_y) = self.mouse_world_position();

        self.world_position.0 += world_x - new_world_x;
        self.world_position.1 += world_y - new_world_y;
    }
}
