use vek::mat::Mat4;
use vek::vec::Vec3;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    // Movement parameters
    yaw: f32,
    pitch: f32,
    speed: f32,
    sensitivity: f32,
    zoom: f32,

    inverted_yaw: bool,

    //Movement Vectors
    position: Vec3<f32>,
    front: Vec3<f32>,
    up: Vec3<f32>,
    right: Vec3<f32>,
    world_up: Vec3<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            yaw: 90.0,
            pitch: 0.0,
            speed: 102.5,
            sensitivity: 0.1,
            zoom: 45.0,

            inverted_yaw: true,

            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

impl Camera {
    pub fn new(
        position: Vec3<f32>,
        up: Vec3<f32>,
        yaw: f32,
        pitch: f32,
        inverted_yaw: bool,
    ) -> Self {
        let mut camera = Self {
            position,
            up,
            yaw,
            pitch,
            inverted_yaw,
            ..Default::default()
        };

        camera.update_vectors();

        camera
    }

    pub fn get_position(&self) -> &Vec3<f32> {
        &self.position
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_view_matrix(&self) -> Mat4<f32> {
        Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard_inputs(&mut self, movement: CameraMovement, delta_time: f32) {
        let velocity = self.speed * delta_time;

        match movement {
            CameraMovement::FORWARD => {
                self.position += self.front * velocity;
            }
            CameraMovement::BACKWARD => {
                self.position -= self.front * velocity;
            }
            CameraMovement::LEFT => {
                self.position -= self.right * velocity;
            }
            CameraMovement::RIGHT => {
                self.position += self.right * velocity;
            }
        }
    }

    pub fn process_mouse_inputs(&mut self, xoffset: f32, yoffset: f32) {       
        let yaw_offset = xoffset * self.sensitivity;
        let pitch_offset = yoffset * self.sensitivity;

        if self.inverted_yaw {
            self.pitch += pitch_offset;
        } else {
            self.pitch -= pitch_offset;
        }
        self.yaw += yaw_offset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        } else if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        self.update_vectors();
    }

    fn update_vectors(&mut self) {
        self.front.x = f32::to_radians(self.yaw).cos() * f32::to_radians(self.pitch).cos();
        self.front.y = f32::to_radians(self.pitch).sin();
        self.front.z = f32::to_radians(self.yaw).sin() * f32::to_radians(self.pitch).cos();
        self.front.normalize();

        self.right = self.front.cross(self.world_up).normalized();
        self.up = self.right.cross(self.front).normalized();
    }
}
