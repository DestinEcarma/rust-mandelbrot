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
    /// Whether the mouse is currently pressed.
    pub mouse_pressed: bool,
}

impl Camera {
    /// Create a new camera with the given window size.
    pub fn new(size: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            scale: defs::START_SCALE,
            size: (size.width as f32, size.height as f32),
            world_position: Default::default(),
            mouse_position: Default::default(),
            mouse_pressed: false,
        }
    }
}

impl Camera {
    /// Move the camera by the given delta.
    pub fn zoom(&mut self, delta: f32) {
        let (world_x, world_y) = self.mouse_world_position();

        self.scale *= defs::ZOOM_FACTOR.powf(-delta * defs::ZOOM_SENSITIVITY);

        let (new_world_x, new_world_y) = self.mouse_world_position();

        self.world_position.0 += world_x - new_world_x;
        self.world_position.1 += world_y - new_world_y;
    }

    /// Pan the camera to the given delta.
    pub fn pan(&mut self, delta: (f32, f32)) {
        let normalized_offset_x = delta.0 / self.size.0;
        let normalized_offset_y = delta.1 / self.size.1;

        let (x, y) = self.position_to_local((normalized_offset_x, normalized_offset_y));

        self.world_position.0 -= x;
        self.world_position.1 -= y;
    }
}

impl Camera {
    /// Convert a position in normalized coordinates to local coordinates.
    fn position_to_local(&self, position: (f32, f32)) -> (f32, f32) {
        let x = position.0 * self.scale * self.size.0 / self.size.1;
        let y = position.1 * self.scale;

        (x, y)
    }

    /// Get the position of the mouse in world coordinates.
    fn mouse_world_position(&self) -> (f32, f32) {
        let normalized_x = self.mouse_position.0 / self.size.0 - 0.5;
        let normalized_y = self.mouse_position.1 / self.size.1 - 0.5;

        let (x, y) = self.position_to_local((normalized_x, normalized_y));

        (self.world_position.0 + x, self.world_position.1 + y)
    }
}
