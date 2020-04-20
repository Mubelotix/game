#![allow(unused_imports)]

use wasm_bindgen::{prelude::*, JsCast};
use wasm_game_lib::graphics::image::Image;
use wasm_game_lib::graphics::sprite::Sprite;
use wasm_game_lib::inputs::{event::Event, keyboard::*, mouse::*};
use wasm_game_lib::graphics::{window::*, canvas::*, color::*};
use wasm_game_lib::inputs::event::types::*;
use wasm_game_lib::system::sleep;
use std::time::Duration;
use std::convert::TryInto;
use web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

mod loader;
mod progress_bar;
mod map;
mod random;
use loader::load_images;
use map::*;
mod units;
use units::*;
mod idx;
mod pathfinder;
use pathfinder::*;

#[allow(clippy::single_match)]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let (mut window, mut canvas) = Window::init_with_events(KEYBOARD_EVENT + RESIZE_EVENT + MOUSE_EVENT);
    let t = load_images(vec!["textures/plains/grassy_plain1", "textures/plains/grassy_plain2", "textures/plains/grassy_plain3", "textures/plains/grassy_plain4", "textures/forest/forest1", "textures/forest/forest2", "textures/forest/forest3", "textures/forest/forest4", "textures/plains/plain1", "textures/plains/plain2", "textures/plains/plain3", "textures/plains/plain4", "textures/underground/dirt", "units/archer.png", "units/knight.png", "units/scout.png", "units/barbarian.png"], &mut canvas).await;

    let mut arrows = (false, false, false, false);
    let (mut width, mut height) = (window.get_width(), window.get_height());
    let mut map = Map::new([&t[0], &t[1], &t[2], &t[3], &t[4], &t[5], &t[6], &t[7], &t[8], &t[9], &t[10], &t[11], &t[12], &t[13], &t[14], &t[15], &t[16]], (width as usize, height as usize));
    let mut path = Path::new();
    path.line_style = LineStyle {
        cap: LineCap::Round,
        color: Color::new(66, 135, 245),
        join: LineJoin::Round,
        size: 14.0,
    };
    map[3.try_into().unwrap()].1 = Some(Unit::new(UnitType::Archer));
    map[6.try_into().unwrap()].1 = Some(Unit::new(UnitType::Scout));
    map[34.try_into().unwrap()].1 = Some(Unit::new(UnitType::Knight));
    map[29.try_into().unwrap()].1 = Some(Unit::new(UnitType::Barbarian));
    map.update_canvas();

    loop {
        for event in window.poll_events() {
            match event {
                Event::KeyboardEvent(ke) => match ke {
                    KeyboardEvent::Down(key) => match key {
                        Key::UpArrow => arrows.0 = true,
                        Key::RightArrow => arrows.1 = true,
                        Key::DownArrow => arrows.2 = true,
                        Key::LeftArrow => arrows.3 = true,
                        _ => (),
                    },
                    KeyboardEvent::Up(key) => match key {
                        Key::UpArrow => arrows.0 = false,
                        Key::RightArrow => arrows.1 = false,
                        Key::DownArrow => arrows.2 = false,
                        Key::LeftArrow => arrows.3 = false,
                        _ => (),
                    }
                }
                Event::ResizeEvent(w, h) => {
                    width = w;
                    height = h;
                    canvas.set_width(width);
                    canvas.set_height(height);
                    map.dimensions = (width as usize, height as usize)
                }
                Event::MouseEvent(me) => match me {
                    MouseEvent::Move(x, y) => {
                        let mut coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
                        coords.1 -= 160;
                        coords.1 -= coords.1 % 193;
                        coords.1 /= 193;
                        coords.0 -= match coords.1 {
                            0 | 8 => 506,
                            1 | 7 => 380,
                            2 | 6 => 253,
                            3 | 5 => 127,
                            _ => 0
                        };
                        coords.0-= coords.0 % 253;
                        coords.0 /= 253;
                        if let Ok(index) = (coords.0 as usize, coords.1 as usize).try_into() {
                            if let Some(route) = find_route(&map, 0.try_into().unwrap(), index) {
                                path.route = Some(route);
                            } else {
                                path.route = None;
                            }
                        } else {
                            path.route = None;
                        }
                    }
                    _ => (),
                }
                event => log!("{:?}", event)
            }
        }

        if arrows.0 { map.coords.1 += 3; }
        if arrows.1 { map.coords.0 -= 3; }
        if arrows.2 { map.coords.1 -= 3; }
        if arrows.3 { map.coords.0 += 3; }

        canvas.clear_with_black();
        canvas.draw(&map);
        canvas.draw(&path);

        sleep(Duration::from_millis(16)).await;
    }
}