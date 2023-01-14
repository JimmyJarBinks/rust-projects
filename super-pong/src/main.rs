#![windows_subsystem = "windows"]

mod entity;
mod game_scene;
mod particle;
mod pause_scene;
mod scene;
mod settings;
mod title_scene;

use self::title_scene::TitleScene;
use self::scene::*;
use self::settings::*;
use tetra::{ContextBuilder, Context, State, Result, graphics, window};

// enhancement of tetra pong tutorial (https://tetra.seventeencups.net/tutorial)
// scenes inspired by tetras.rs (https://github.com/17cupsofcoffee/tetra/blob/main/examples/tetras.rs)

struct GameState {
    scenes: Vec<Box<dyn Scene>>,
}

impl GameState {
    fn new(ctx: &mut Context) -> Result<GameState> {
        let starting_scene = TitleScene::new(ctx, String::from("SUPER PONG"))?;

        Ok(GameState {
            scenes: vec![Box::new(starting_scene)],
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result {
        match self.scenes.last_mut() {
            Some(current) => match current.update(ctx)? {
                Transition::None => {}
                Transition::Push(s) => { self.scenes.push(s); }
                Transition::Pop => { self.scenes.pop(); }
                Transition::NewGame(s) => {
                    self.scenes.clear();
                    self.scenes.push(s);
                }
            },
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.draw(ctx)? {
                Transition::None => {}
                Transition::Push(s) => { self.scenes.push(s); }
                Transition::Pop => { self.scenes.pop(); }
                Transition::NewGame(s) => {
                    self.scenes.pop();
                    self.scenes.push(s); 
                }
            },
            None => window::quit(ctx),
        }
        graphics::reset_canvas(ctx);

        Ok(())
    }
}

fn main() -> Result {
    ContextBuilder::new("Super Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}