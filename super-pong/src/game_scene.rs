use crate::{entity::Entity, particle::Particle, pause_scene::PauseScene, scene::*, settings::*, title_scene::TitleScene};
use rand::{Rng, thread_rng, random};
use tetra::{graphics::{self, Color, Texture, DrawParams, mesh::{ShapeStyle, Mesh}, text::{Font, Text}}, audio::Sound};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, Result};

pub struct GameScene {
    player1: Entity,
    player2: Entity,
    ai_mode: bool,
    ball: Entity,
    particles: Vec<Particle>,
    scoreboard: Text,
    scores: Vec<i32>,
    hit_streak: i32,
}

impl GameScene {
    pub fn new(ctx: &mut Context, p1_color: usize, p2_color: usize, ai_mode: bool) -> Result<GameScene> {

        let player1_texture = Texture::new(ctx, TEXTURE_LIST[p1_color])?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, TEXTURE_LIST[p2_color])?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );

        let ball_texture = Texture::new(ctx, "./assets/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let mut rng = thread_rng();
        let init_direction: f32 = (rng.gen_bool(0.5) as i32 as f32) * 2.0 - 1.0;
        let init_ball_velocity = Vec2::new(-BALL_SPEED * init_direction as f32, 0.0);

        let scoreboard = Text::new(
            "0 - 0",
            Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE)?,
        );

        Ok(GameScene {
            player1: Entity::new(player1_texture, player1_position, Vec2::zero()),
            player2: Entity::new(player2_texture, player2_position, Vec2::zero()),
            ai_mode,
            ball: Entity::new(ball_texture, ball_position, init_ball_velocity),
            particles: Vec::new(),
            scores: vec![0, 0],
            scoreboard,
            hit_streak: 0,
        })
    }

    fn input(&mut self, ctx: &mut Context) -> &str {
        if input::is_key_down(ctx, Key::W) {
            self.player1.velocity.y = (self.player1.velocity.y - PADDLE_SPEED).clamp(-MAX_PADDLE_SPEED, 0.0);
        }
        if input::is_key_down(ctx, Key::S) {
            self.player1.velocity.y = (self.player1.velocity.y + PADDLE_SPEED).clamp(0.0, MAX_PADDLE_SPEED);
        }
        if input::is_key_down(ctx, Key::D) {
            if self.player1.can_boost() {

                self.player1.velocity.x = BOOST_SPEED; 
            }
        }
        if !self.ai_mode {
            if input::is_key_down(ctx, Key::Up) {
                self.player2.velocity.y = (self.player2.velocity.y - PADDLE_SPEED).clamp(-MAX_PADDLE_SPEED, 0.0);
            }
            if input::is_key_down(ctx, Key::Down) {
                self.player2.velocity.y = (self.player2.velocity.y + PADDLE_SPEED).clamp(0.0, MAX_PADDLE_SPEED);
            }
            if input::is_key_down(ctx, Key::Left) {
                if self.player2.can_boost() {

                    self.player2.velocity.x = -BOOST_SPEED;
                }
            }
        }
        if input::is_key_pressed(ctx, Key::Enter) {
            return "Pause";
        }

        "Continue"
    }

    fn ai_movement(&mut self) {
        let y_diff = (((self.ball.position.y - self.player2.centre().y) / self.player2.height() * 4.0) as i32).signum();
        let should_boost = random::<f32>() < (1.0 / 60.0) &&
                                self.player2.can_boost() &&
                                self.ball.position.x > CENTER.x &&
                                self.ball.velocity.x > 0.0;

        self.player2.velocity.y = (self.player2.velocity.y + (y_diff as f32 * 0.875 * PADDLE_SPEED)).clamp(-MAX_PADDLE_SPEED, MAX_PADDLE_SPEED);
        if should_boost {
            self.player2.velocity.x = -BOOST_SPEED;
        }
    }

    fn hit(&mut self, ctx: &mut Context) -> Result {
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) && !self.player1.last_hit {
            (self.player1.last_hit, self.player2.last_hit) = (true, false);
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) && !self.player2.last_hit {
            (self.player1.last_hit, self.player2.last_hit) = (false, true);
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.hit_streak += 1;
            for _n in 0..(self.hit_streak / 10 + 4)  {
                self.particles.push(Particle::new(
                    Texture::new(ctx, "./assets/particle.png")?, 
                    self.ball.position, 
                    -self.ball.velocity));
            }
            let mut boost_bonus = 1.0;
            if self.ball.velocity.x.signum() != paddle.velocity.x.signum() && paddle.velocity.x != 0.0 {
                Sound::new("assets/strong_hit.ogg")?.play_with(ctx, 1.0, 1.0)?;
                boost_bonus = 2.0
            }
            else { Sound::new("assets/paddle_hit.ogg")?.play_with(ctx, 0.5, 1.0)?; };
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * boost_bonus * self.ball.velocity.x.signum())).clamp(
                -MAX_BALL_SPEED, MAX_BALL_SPEED);
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        Ok(())
    }

    fn score(&mut self, ctx: &mut Context, winner: i32) -> Result<Transition> {
        Sound::new("assets/score.ogg")?.play_with(ctx, 0.75, 1.0)?;
        let index = (winner - 1) as usize;
        self.scores[index] += 1;
        self.scoreboard = Text::new(
            format!("{} - {}", self.scores[0], self.scores[1]),
            Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE)?,
        );
        self.hit_streak = 0;
        if self.scores[index] >= WINNING_SCORE {
            let win_screen = TitleScene::new(ctx, format!("Player {} wins!", winner))?;
            return Ok(Transition::NewGame(Box::new(win_screen)));
        }

        self.player1.reset_paddle();
        self.player2.reset_paddle();
        self.ball.reset_ball();
        self.particles.clear();

        Ok(Transition::None)
    }

}

impl Scene for GameScene {

    fn update(&mut self, ctx: &mut Context) -> Result<Transition> {
        // Input
        if self.input(ctx) == "Pause" { 
            return Ok(Transition::Push(Box::new(PauseScene::new()?)));
        }
        if self.ai_mode { self.ai_movement() }

        // Movement
        self.player1.move_paddle();
        self.player2.move_paddle();
        self.ball.position += self.ball.velocity;

        for particle in &mut self.particles { particle.update() }
        self.particles.retain(|particle| particle.lifetime > 0);

        // Collision
        self.hit(ctx)?;
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            Sound::new("assets/wall_hit.ogg")?.play_with(ctx, 
                (self.ball.velocity.y.abs() / 10.0).clamp(0.5, 1.0), 1.0)?;
            self.ball.velocity.y = -self.ball.velocity.y;
                        for _n in 0..(self.hit_streak / 10 + 4)  {
                self.particles.push(Particle::new(
                    Texture::new(ctx, "./assets/particle.png")?, 
                    self.ball.position, 
                    -self.ball.velocity));
            }

        }

        // Score
        if self.ball.position.x > WINDOW_WIDTH { return Ok(self.score(ctx, 1)?); }
        if self.ball.position.x < -self.ball.width() { return Ok(self.score(ctx, 2)?); }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        let center_circle = Mesh::circle(ctx, ShapeStyle::Stroke(2.0), Vec2::zero(), 32.0)?;
        center_circle.draw(ctx, CENTER);

        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);
        for particle in &self.particles {
            particle.texture.draw(ctx, DrawParams{ position: particle.position, scale: Vec2::one(), origin: Vec2::zero(), 
                                                        rotation: particle.rotation, color: Color::WHITE} );
        }
        let center_x = center_text(ctx, &mut self.scoreboard);
        self.scoreboard.draw(ctx, Vec2::new(center_x, 16.0));
        Ok(Transition::None)
    }
}