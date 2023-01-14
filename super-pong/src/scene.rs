use tetra::{Context, Result};

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> Result<Transition>;
    fn draw(&mut self, ctx: &mut Context) -> Result<Transition>;
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    NewGame(Box<dyn Scene>),
}