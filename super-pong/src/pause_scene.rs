use crate::{scene::*, settings::*, title_scene::TitleScene};
use tetra::{Context, Result, input::{Key, self}, math::Vec2, graphics::text::{Text, Font}};

pub struct PauseScene { }

impl PauseScene {
    pub fn new() -> Result<PauseScene> { Ok(PauseScene {  }) }
}

impl Scene for PauseScene {
    fn update(&mut self, ctx: &mut Context) -> Result<Transition> {
        if input::is_key_pressed(ctx, Key::Enter) {
            Ok(Transition::Pop)
        } else if input::is_key_pressed(ctx, Key::Escape)  {
            let title_scene = TitleScene::new(ctx, String::from("SUPER PONG"))?;
            Ok(Transition::NewGame(Box::new(title_scene)))
        }
        else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<Transition> {

        let mut pause_menu = Text::new("Game Paused",
        Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE * 2.0)?);
        let center_x = center_text(ctx, &mut pause_menu);
        pause_menu.draw(ctx, Vec2::new(center_x, 40.0));

        let mut instructions = Text::new("Press enter again to continue.\nPress escape to quit.",
        Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE)?);
        let center_x = center_text(ctx, &mut instructions);
        instructions.draw(ctx, Vec2::new(center_x, WINDOW_HEIGHT / 1.5));
        
        Ok(Transition::None)
    }
}