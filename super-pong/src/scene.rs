use tetra::{Context, Result};

/*
All scenes must implement the update and draw functions that are called each frame.
Transitions set up the scenes like a stack where the top scene is active
and the scenes below are paused.
*/

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