use tetra::{math::Vec2, Context, graphics::{text::Text, Color}};

// List of settings used throughout the program.

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 480.0;
pub const CENTER: Vec2<f32> = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
pub const PADDLE_SPEED: f32 = 5.0;
pub const MAX_PADDLE_SPEED: f32 = PADDLE_SPEED * PADDLE_SPEED;
pub const BOOST_SPEED: f32 = 15.0;
pub const BALL_SPEED: f32 = 5.0;
pub const MAX_BALL_SPEED: f32 = 40.0;
pub const PADDLE_SPIN: f32 = 7.5;
pub const BALL_ACC: f32 = 0.5;
pub const WINNING_SCORE: i32 = 10;
pub const FONT_SIZE: f32 = 36.0;
pub const PARTICLE_LIFETIME: i32 = 30;
pub const COLOR_LIST: [Color; 5] = [Color::rgb(0.5, 0.5, 1.0), Color::rgb(0.9, 0.0, 0.0), 
                                    Color::rgb(0.0, 0.75, 0.0), Color::rgb(1.0, 0.6, 0.0),
                                    Color::rgb(0.5, 0.5, 0.5)];
pub const TEXTURE_LIST: [&str; 5] = ["./assets/blue_player.png", "./assets/red_player.png", 
                                    "./assets/green_player.png", "./assets/orange_player.png",
                                    "./assets/grey_player.png"];

pub fn center_text(ctx: &mut Context, text: &mut Text) -> f32 {
    (WINDOW_WIDTH - text.get_bounds(ctx).unwrap().width) / 2.0
}