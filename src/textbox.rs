use crate::units::Attack;
use wasm_game_lib::graphics::text::*;
use wasm_game_lib::graphics::drawable::*;
use wasm_game_lib::graphics::canvas::*;
use wasm_game_lib::graphics::font::*;
use wasm_game_lib::graphics::color::*;
use wasm_bindgen::JsValue;
use wasm_game_lib::inputs::mouse::{get_mouse_position, is_mouse_pressed};
use std::cell::RefCell;

const BOX_STYLE: LineStyle = LineStyle {
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

const MARGIN: usize = 10;
const FONT_SIZE: usize = 20;

pub struct TextBox<'a> {
    coords: (f64, f64),
    width: RefCell<usize>,
    full_message: Vec<&'a str>,
    displayed_message: RefCell<(usize, Vec<usize>)>,
    displayed_text: RefCell<Text<'a>>,
}

impl<'a> TextBox<'a> {
    pub fn new(coords: (f64, f64), width: usize, font: &'a Font, text: &'a str) -> TextBox<'a> {
        let mut displayed_text = Text::new_with_text_and_coords(&font, String::new(), (coords.0 as usize + MARGIN, coords.1 as usize + MARGIN + FONT_SIZE / 2));
        displayed_text.style.color = Color::white();
        displayed_text.character_size = (FONT_SIZE, "px");
        
        TextBox {
            coords,
            width: RefCell::new(width),
            full_message: text.split(' ').collect(),
            displayed_message: RefCell::new((0, Vec::new())),
            displayed_text: RefCell::new(displayed_text)
        }
    }

    pub fn set_width(&mut self, width: usize) {
        self.displayed_text.borrow_mut().set_text(String::new());
        *self.displayed_message.borrow_mut() = (0, Vec::new());
        *self.width.borrow_mut() = width;
    }

    pub fn init(&self, mut canvas: &mut Canvas) {
        let width = *self.width.borrow() as f64;

        while self.displayed_message.borrow().0 < self.full_message.len() {
            if self.displayed_text.borrow().get_width(&mut canvas) <= width - MARGIN as f64 * 2.0 {
                let (words, end_line) = &mut *self.displayed_message.borrow_mut();
                *words += 1;
                let mut displayed_message = String::new();
                for (idx, word) in self.full_message.iter().enumerate() {
                    if idx < *words {
                        displayed_message.push_str(word);
                        if end_line.contains(&idx) {
                            displayed_message.push('\n');
                        } else {
                            displayed_message.push(' ');
                        }
                    }
                }
                self.displayed_text.borrow_mut().set_text(displayed_message);

                if *words == self.full_message.len() && self.displayed_text.borrow().get_width(&mut canvas) > width - MARGIN as f64 * 2.0 {
                    end_line.push(*words - 2);
                    let mut displayed_message = String::new();
                    for (idx, word) in self.full_message.iter().enumerate() {
                        if idx < *words {
                            displayed_message.push_str(word);
                            if end_line.contains(&idx) {
                                displayed_message.push('\n');
                            } else {
                                displayed_message.push(' ');
                            }
                        }
                    }
                    self.displayed_text.borrow_mut().set_text(displayed_message);
                }
            } else {
                let (words, end_line) = &mut *self.displayed_message.borrow_mut();
                end_line.push(*words - 2);
                *words += 1;
                let mut displayed_message = String::new();
                for (idx, word) in self.full_message.iter().enumerate() {
                    if idx < *words {
                        displayed_message.push_str(word);
                        if end_line.contains(&idx) {
                            displayed_message.push('\n');
                        } else {
                            displayed_message.push(' ');
                        }
                    }
                }
                self.displayed_text.borrow_mut().set_text(displayed_message);
            }
        }
    }

    pub fn get_height(&self) -> usize {
        self.displayed_text.borrow().get_height() + MARGIN * 2
    }

    pub fn set_y(&mut self, y: usize) {
        self.coords.1 = y as f64;
        self.displayed_text.borrow_mut().coords.1 = y + MARGIN + FONT_SIZE / 2;
    }

    pub fn is_hover(&self) -> bool {
        let mouse_position = get_mouse_position();
        let mouse_position = (mouse_position.0 as f64, mouse_position.1 as f64);
        let width = *self.width.borrow() as f64;

        mouse_position.0 > self.coords.0 && mouse_position.0 < self.coords.0 + width && mouse_position.1 > self.coords.1 && mouse_position.1 < self.coords.1 + self.get_height() as f64
    }

    pub fn is_pressed(&self) -> bool {
        self.is_hover() && is_mouse_pressed()
    }
}

impl<'a> Drawable for TextBox<'a> {
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
        self.init(&mut canvas);
        let width = *self.width.borrow() as f64;

        BOX_STYLE.apply_on_canvas(&mut canvas);
        let context = canvas.get_2d_canvas_rendering_context();
        
        context.begin_path();
        context.stroke_rect(self.coords.0, self.coords.1, width, self.displayed_text.borrow().get_height() as f64 + FONT_SIZE as f64 / 2.0);

        if self.is_hover() {
            if self.is_pressed() {
                context.set_fill_style(&JsValue::from_str("rgb(54, 58, 99)"));
            } else {
                context.set_fill_style(&JsValue::from_str("rgb(44, 48, 79)"));
            }
        } else {
            context.set_fill_style(&JsValue::from_str("rgb(24, 28, 39)"));
        }

        context.fill_rect(self.coords.0, self.coords.1, width, self.displayed_text.borrow().get_height() as f64 + FONT_SIZE as f64 / 2.0);
        context.stroke();
        canvas.draw(&*self.displayed_text.borrow());
    }
}