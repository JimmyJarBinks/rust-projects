use crate::settings::*;
use rand::{thread_rng, Rng};
use tetra::graphics::Texture;
use tetra::math::Vec2;

// Particles are spawned when the ball collides with paddles and walls.

pub struct Particle {
    pub texture: Texture,
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub rotation: f32,
    pub lifetime: i32, 
}

impl Particle {
    pub fn new(texture: Texture, position: Vec2<f32>, ball_velocity: Vec2<f32>) -> Particle {
        Particle { texture, position, velocity: generate_velocity(ball_velocity),
                   rotation: starting_rotation(), lifetime: PARTICLE_LIFETIME }
    }

    pub fn update(& mut self) {
        self.position += self.velocity;
        self.rotation += 0.02;
        self.lifetime -= 1; 
    }
}

fn generate_velocity(ball_velocity: Vec2<f32>) -> Vec2<f32> {
    let mut rng = thread_rng();
    let x_vel: f32 = rng.gen_range(0.05, 0.1);
    let y_vel: f32 = rng.gen_range(-2.0, 2.0);
    Vec2::new(ball_velocity.x * x_vel, y_vel)
}

fn starting_rotation() -> f32 {
    let mut rng = thread_rng();
    let rotation: f32 = rng.gen_range(0.0, 360.0);
    rotation
}