use crate::{game_scene::GameScene, scene::*, settings::*};
use tetra::graphics::{Texture, DrawParams, Rectangle};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Text, Font};
use tetra::math::Vec2;
use tetra::{Context, graphics::{self, Color}, input::{self, Key, MouseButton}, Result};

trait Button {
    fn position(&self) -> Vec2<f32>;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn clicked(&self, mouse_position: Vec2<f32>) -> bool {
        mouse_position.x >= self.position().x && mouse_position.x <= self.position().x + self.width() as f32 &&
        mouse_position.y >= self.position().y && mouse_position.y <= self.position().y + self.height() as f32 
    }
}

struct SelectButton {
    position: Vec2<f32>,
    texture: Texture,
    text: Text,
}

impl SelectButton {
    fn new(position: Vec2<f32>, texture: Texture, text: Text) -> SelectButton { 
        SelectButton { position, texture, text} 
    }
}

impl Button for SelectButton {
    fn position(&self) -> Vec2<f32> { self.position }
    fn width(&self) -> f32 { self.texture.clone().width() as f32 }
    fn height(&self) -> f32 { self.texture.clone().height() as f32 }
}

struct ColorButton {
    position: Vec2<f32>,
    width: f32,
    height: f32,
    color: Color,
    selected: bool
}

impl ColorButton {
    fn new(position: Vec2<f32>, width: f32, height: f32, color: Color) -> ColorButton { 
        ColorButton { position, width, height, color, selected: false } 
    }
}

impl Button for ColorButton {
    fn position(&self) -> Vec2<f32> { self.position }
    fn width(&self) -> f32 { self.width }
    fn height(&self) -> f32 { self.height }
}

pub struct TitleScene {
    title_text: Text,
    instructions: Text,
    pvp_button: SelectButton,
    vs_comp_button: SelectButton,
    player1_colors: Vec<ColorButton>,
    player2_colors: Vec<ColorButton>,
}

impl TitleScene {
    pub fn new(ctx: &mut Context, title_text: String) -> Result<TitleScene> {
        let title_text = Text::new(
            title_text,
            Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE * 2.0)?,
        );
        let instructions = Text::new(
            "Press escape to exit.",
            Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE)?,
        );
        let pvp_button = SelectButton::new(Vec2::new(WINDOW_WIDTH * 0.2, WINDOW_HEIGHT / 2.0),
                                            Texture::new(ctx, "./assets/button.png")?,
                                            Text::new("2-Player",
                                        Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE * 0.6)?));
        let vs_comp_button = SelectButton::new(Vec2::new(WINDOW_WIDTH * 0.6, WINDOW_HEIGHT / 2.0),
                                                Texture::new(ctx, "./assets/button.png")?,
                                                Text::new("Vs. Comp",
                                            Font::vector(ctx, "./assets/OpenSans-Regular.ttf", FONT_SIZE * 0.6)?));

        let mut player1_colors = Vec::new();
        let mut player2_colors = Vec::new();
        let mut y_position = 48 as f32;
        let button_size = 64 as f32;
        let offset = 8 as f32;
        
        for color in COLOR_LIST {
            player1_colors.push(ColorButton::new(Vec2::new(offset, y_position),
                        button_size, button_size, color));
            player2_colors.push(ColorButton::new(Vec2::new(WINDOW_WIDTH - button_size - offset, y_position),
                       button_size, button_size, color));
            y_position += button_size + offset;
        }
        player1_colors[0].selected = true;
        player2_colors[1].selected = true;

        Ok(TitleScene { title_text, instructions, pvp_button, vs_comp_button, player1_colors, player2_colors })
    }

    fn update_color_selection(&mut self, player: i32, mouse_position: Vec2<f32>) {
        let selection = if player == 1 { &mut self.player1_colors } else { &mut self.player2_colors };
        let mut index = -1;
        for i in 0..selection.len() {
            if selection[i].clicked(mouse_position) {
                index = i as i32;
            }
        }
        if index >= 0 {
            selection.iter_mut().map(|x| x.selected = false).count();
            selection[index as usize].selected = true;
        }
    }

    fn draw_color_selection(&mut self, player: i32, ctx: &mut Context) -> Result {
        let selection = if player == 1 { &self.player1_colors } else { &self.player2_colors };
        for option in selection {
            let button = Mesh::rectangle(ctx, ShapeStyle::Fill,
                    Rectangle { x: option.position.x, y: option.position.y, width: option.width, height: option.width })?;
            let mut button_params = DrawParams::new();
            button_params.color = option.color;
            button.draw(ctx, button_params);

            if option.selected {
                let button = Mesh::rectangle(ctx, ShapeStyle::Stroke(2.0),
                    Rectangle { x: option.position.x, y: option.position.y, width: option.width, height: option.width })?;
                button.draw(ctx, DrawParams::new());
            }
        }
        Ok(())
    }

    fn start_game(&mut self, ctx: &mut Context, ai_mode: bool) -> Result<Transition> {
        self.player1_colors.retain(|option| option.selected);
        self.player2_colors.retain(|option| option.selected);
        let p1_color = COLOR_LIST.iter().position(|&x| x == self.player1_colors[0].color).unwrap();
        let p2_color = COLOR_LIST.iter().position(|&x| x == self.player2_colors[0].color).unwrap();
        return Ok(Transition::NewGame(Box::new(GameScene::new(ctx, p1_color, p2_color, ai_mode)?)))
    }

}

impl Scene for TitleScene {
    fn update(&mut self, ctx: &mut Context) -> Result<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let mouse_position = input::get_mouse_position(ctx);
            if self.pvp_button.clicked(mouse_position) {
                return self.start_game(ctx, false);
            }
            if self.vs_comp_button.clicked(mouse_position) {
                return self.start_game(ctx, true);
            }
            self.update_color_selection(1, mouse_position);
            self.update_color_selection(2, mouse_position);
        } 
        if input::is_key_pressed(ctx, Key::Escape) {
            return Ok(Transition::Pop);
        }
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        let center_x = center_text(ctx, &mut self.title_text);
        self.title_text.draw(ctx, Vec2::new(center_x, 32.0));

        let center_x = center_text(ctx, &mut self.instructions);
        self.instructions.draw(ctx, Vec2::new(center_x, WINDOW_HEIGHT * 0.75));

        self.pvp_button.texture.draw(ctx, self.pvp_button.position);
        self.pvp_button.text.draw(ctx, self.pvp_button.position + Vec2::new(20.0, 12.0));

        self.vs_comp_button.texture.draw(ctx, self.vs_comp_button.position);
        self.vs_comp_button.text.draw(ctx, self.vs_comp_button.position + Vec2::new(16.0, 12.0));

        self.draw_color_selection(1, ctx)?;
        self.draw_color_selection(2, ctx)?;

        Ok(Transition::None)
    }
}