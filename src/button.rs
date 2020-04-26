use wasm_game_lib::graphics::text::*;
use wasm_game_lib::graphics::drawable::*;
use wasm_game_lib::graphics::canvas::*;
use wasm_game_lib::graphics::font::*;
use wasm_game_lib::graphics::color::*;
use wasm_bindgen::JsValue;
use wasm_game_lib::inputs::mouse::{get_mouse_position, is_mouse_pressed};
use std::cell::RefCell;

const BUTTON_STYLE: LineStyle = LineStyle {
    cap: LineCap::Round,
    join: LineJoin::Round,
    size: 4.0,
    color: Color {
        alpha: 255,
        red: 67,
        green: 75,
        blue: 96
    }
};

pub struct Button<'a> {
    pub coords: (f64, f64),
    pub width: RefCell<Option<f64>>,
    pub text: Text<'a>
}

impl<'a> Button<'a> {
    pub fn new(coords: (f64, f64), width: Option<f64>, font: &'a Font, text: String) -> Button<'a> {
        let mut text = Text::new_with_text_and_coords(&font, text, (coords.0 as usize + 7, coords.1 as usize + 35));
        text.style.color = Color::white();
        text.character_size = (2, "rem");
        
        Button {
            coords,
            width: RefCell::new(width),
            text
        }
    }

    pub fn is_pressed(&self) -> bool {
        let mouse_position = get_mouse_position();
        let mouse_position = (mouse_position.0 as f64, mouse_position.1 as f64);
        let width = self.width.borrow().unwrap();

        if mouse_position.0 > self.coords.0 && mouse_position.0 < self.coords.0 + width && mouse_position.1 > self.coords.1 && mouse_position.1 < self.coords.1 + 50.0 {
            if is_mouse_pressed() {
                return true;
            }
        }

        return false;
    }
}

impl<'a> Drawable for Button<'a> {
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
        if self.width.borrow().is_none() {
            *self.width.borrow_mut() = Some(self.text.get_width(&mut canvas) + 14.0);
        }
        let width = self.width.borrow().unwrap();

        BUTTON_STYLE.apply_on_canvas(&mut canvas);
        let context = canvas.get_2d_canvas_rendering_context();
        
        context.begin_path();
        context.stroke_rect(self.coords.0, self.coords.1, width, 50.0);
        let mouse_position = get_mouse_position();
        let mouse_position = (mouse_position.0 as f64, mouse_position.1 as f64);
        
        if mouse_position.0 > self.coords.0 && mouse_position.0 < self.coords.0 + width && mouse_position.1 > self.coords.1 && mouse_position.1 < self.coords.1 + 50.0 {
            if is_mouse_pressed() {
                context.set_fill_style(&JsValue::from_str("green"));
            } else {
                context.set_fill_style(&JsValue::from_str("cyan"));
            }
        } else {
            context.set_fill_style(&JsValue::from_str("rgb(24, 28, 39)"));
        }

        context.fill_rect(self.coords.0, self.coords.1, width, 50.0);
        context.stroke();
        canvas.draw(&self.text);
    }
}