use crate::settings::*;
use tetra::graphics::{Rectangle, Texture};
use tetra::math::Vec2;

// Defines the logic for game entities (Paddles and Balls).

pub struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
    original_position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub last_hit: bool,  
}

impl Entity {
    pub fn new(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity { texture, position, original_position: position, velocity, last_hit: false }
    }

    pub fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    pub fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    pub fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }

    pub fn can_boost(&self) -> bool {
        self.position.x == self.original_position.x
    }

    fn edge_face(& self) -> f32 {
        (self.original_position.x - CENTER.x).signum()
    }

    pub fn move_paddle(& mut self) {
        self.position.x = self.position.x + self.velocity.x;
        self.velocity.x = (self.velocity.x + self.edge_face()).clamp(-BOOST_SPEED, BOOST_SPEED);
        self.position.y = (self.position.y + self.velocity.y).clamp(0.0, WINDOW_HEIGHT - self.height());
        self.velocity.y = (self.velocity.y / 2.0) as i32 as f32;

        if self.edge_face() * (self.original_position.x - self.position.x) <= 0.0 {
            self.position.x = self.original_position.x;
            self.velocity.x = 0.0;
        } 
    }

    pub fn reset_paddle(& mut self) {
        self.position = self.original_position;
        self.velocity.x = 0.0;
        self.last_hit = false;
    }

    pub fn reset_ball(& mut self) {
        self.position = self.original_position;
        self.velocity = Vec2::new(BALL_SPEED * self.velocity.x.signum(), 0.0);
    }
}