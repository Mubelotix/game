use wasm_bindgen::{prelude::*, JsCast};
use wasm_game_lib::graphics::image::Image;
use wasm_game_lib::graphics::sprite::Sprite;
use wasm_game_lib::inputs::{event::Event, keyboard::*};
use wasm_game_lib::graphics::window::Window;
use wasm_game_lib::system::log;
use wasm_game_lib::inputs::event::types::*;
use wasm_game_lib::system::sleep;
use std::time::Duration;
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

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let (mut window, mut canvas) = Window::init_with_events(KEYBOARD_EVENT + RESIZE_EVENT);
    let t = load_images(vec!["textures/plains/grassy_plain1", "textures/plains/grassy_plain2", "textures/plains/grassy_plain3", "textures/plains/grassy_plain4", "textures/forest/forest1", "textures/forest/forest2", "textures/forest/forest3", "textures/forest/forest4", "textures/plains/plain1", "textures/plains/plain2", "textures/plains/plain3", "textures/plains/plain4", "textures/underground/dirt", "units/archer.png", "units/knight.png", "units/scout.png", "units/barbarian.png"], &mut canvas).await;

    let mut arrows = (false, false, false, false);
    let (mut width, mut height) = (window.get_width(), window.get_height());
    let mut map = Map::new([&t[0], &t[1], &t[2], &t[3], &t[4], &t[5], &t[6], &t[7], &t[8], &t[9], &t[10], &t[11], &t[12], &t[13], &t[14], &t[15], &t[16]], (width as usize, height as usize));
    map[3.into()].1 = Some(Unit::new(UnitType::Archer));
    map[13.into()].1 = Some(Unit::new(UnitType::Archer));
    map[16.into()].1 = Some(Unit::new(UnitType::Archer));
    map[5.into()].1 = Some(Unit::new(UnitType::Archer));
    map[6.into()].1 = Some(Unit::new(UnitType::Scout));
    map[26.into()].1 = Some(Unit::new(UnitType::Scout));
    map[32.into()].1 = Some(Unit::new(UnitType::Scout));
    map[34.into()].1 = Some(Unit::new(UnitType::Knight));
    map[52.into()].1 = Some(Unit::new(UnitType::Knight));
    map[29.into()].1 = Some(Unit::new(UnitType::Knight));
    map[29.into()].1 = Some(Unit::new(UnitType::Barbarian));
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
                event => log!("{:?}", event)
            }
        }

        if arrows.0 { map.coords.1 += 3; }
        if arrows.1 { map.coords.0 -= 3; }
        if arrows.2 { map.coords.1 -= 3; }
        if arrows.3 { map.coords.0 += 3; }

        canvas.clear_with_black();
        canvas.draw(&map);

        sleep(Duration::from_millis(16)).await;
    }

    Ok(())
}