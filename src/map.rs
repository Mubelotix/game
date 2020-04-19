use wasm_game_lib::graphics::{drawable::*, canvas::Canvas, image::*};
use crate::{random::get_random, units::*, idx::HexIndex};
use arr_macro::arr;

fn idx_to_y(idx: usize) -> usize {
    if idx < 5 {
        0
    } else if idx < 11 {
        1
    } else if idx < 18 {
        2
    } else if idx < 26 {
        3
    } else if idx < 35 {
        4
    } else if idx < 43 {
        5
    } else if idx < 50 {
        6
    } else if idx < 56 {
        7
    } else {
        8
    }
}

fn idx_to_x(idx: usize) -> usize {
    if idx < 5 {
        idx
    } else if idx < 11 {
        idx - 5
    } else if idx < 18 {
        idx - 11
    } else if idx < 26 {
        idx - 18
    } else if idx < 35 {
        idx - 26
    } else if idx < 43 {
        idx - 35
    } else if idx < 50 {
        idx - 43
    } else if idx < 56 {
        idx - 50
    } else {
        idx - 56
    }
}

fn idx_to_coords(idx: usize) -> (usize, usize) {
    (idx_to_x(idx), idx_to_y(idx))
}

const TEXTURES_NUMBER: usize = 17;
const CANVAS_WIDTH: f64 = 9.0*253.0;
const CANVAS_HEIGHT: f64 = 8.0*256.0 + 10.0;


#[derive(Clone, Copy, Debug)]
pub enum Tile {
    GrassyPlain(u8),
    Forest(u8),
    Plain(u8),
}

impl Tile {
    fn get_texture_idx(self) -> u8 {
        match self {
            Tile::GrassyPlain(number) => number,
            Tile::Forest(number) => 4 + number,
            Tile::Plain(number) => 8 + number,
        }
    }
}

pub struct Map<'a> {
    pub coords: (isize, isize),
    tiles: [Tile; 61],
    pub units: [Option<Unit>; 61],
    textures: [&'a Image; TEXTURES_NUMBER],
    canvas: Canvas,
    pub dimensions: (usize, usize)
}

impl<'a> Map<'a> {
    #[allow(clippy::cognitive_complexity)]
    pub fn new(textures: [&'a Image; TEXTURES_NUMBER], dimensions: (usize, usize)) -> Map {
        let mut canvas = Canvas::new();
        canvas.set_width(CANVAS_WIDTH as u32);
        canvas.set_height(CANVAS_HEIGHT as u32);
        let tiles = arr!({
            let random = get_random(2);
            match random {
                0 => Tile::GrassyPlain(get_random(3)),
                1 => Tile::Forest(get_random(3)),
                _ => Tile::Plain(get_random(3)),
            }
        }; 61);

        let mut map = Map {
            coords: (0,0),
            tiles,
            textures,
            canvas,
            units: arr!(None; 61),
            dimensions
        };

        map.update_canvas();

        map
    }

    pub fn update_canvas(&mut self) {
        for (idx, tile) in self.tiles.iter().enumerate() {
            let coords = idx_to_coords(idx);
            let screen_coords = (coords.0 * 253, coords.1 * 193);
            let offset = match coords.1 {
                0 | 8 => 4,
                1 | 7 => 3,
                2 | 6 => 2,
                3 | 5 => 1,
                4 => 0,
                _ => panic!("can't happen")
            } * 128;
            
            self.canvas.draw_image(((offset + screen_coords.0) as f64, screen_coords.1 as f64), self.textures[tile.get_texture_idx() as usize]);

            if coords.1 == 8 || (coords.0 == 0 && coords.1 >= 4) || (coords.1 >= 4 && (idx_to_y(idx) != idx_to_y(idx+1))) {
                self.canvas.draw_image(((offset + screen_coords.0) as f64, screen_coords.1 as f64 + 318.45), self.textures[12]);
            }
        }

        let context = self.canvas.get_2d_canvas_rendering_context();
        for (idx, unit) in self.units.iter().enumerate().filter(|(_idx, u)| u.is_some()) {
            let unit = unit.as_ref().unwrap();
            
            let coords = idx_to_coords(idx);
            let screen_coords = (coords.0 * 253, coords.1 * 193);
            let offset = match coords.1 {
                0 | 8 => 4,
                1 | 7 => 3,
                2 | 6 => 2,
                3 | 5 => 1,
                4 => 0,
                _ => panic!("can't happen")
            } * 128;
            
            //self.canvas.draw_image(((offset + screen_coords.0) as f64, screen_coords.1 as f64 + 100.0), self.textures[unit.unit_type.get_texture_idx()]);
            context.draw_image_with_html_image_element_and_dw_and_dh(self.textures[unit.unit_type.get_texture_idx()].get_html_element(), (offset + screen_coords.0) as f64 + 50.0, screen_coords.1 as f64 + 160.0, 150.0, 150.0).unwrap();
        }
    }
}

impl<'a> Drawable for Map<'a> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        let factor_width: f64 = self.dimensions.0 as f64 / CANVAS_WIDTH;
        let factor_height = self.dimensions.1 as f64 / CANVAS_HEIGHT;
        let smaller_factor = if factor_width < factor_height {
            factor_width
        } else {
            factor_height
        };
        let fitting_width = CANVAS_WIDTH * smaller_factor;
        let fitting_height = CANVAS_HEIGHT * smaller_factor;
        let remaining_width = self.dimensions.0 as f64 - fitting_width;
        let remaining_height = self.dimensions.1 as f64 - fitting_height;

        let canvas_element = canvas.get_2d_canvas_rendering_context();
        canvas_element.draw_image_with_html_canvas_element_and_dw_and_dh(self.canvas.get_canvas_element(), self.coords.0 as f64 + remaining_width / 2.0, self.coords.1 as f64 + remaining_height / 2.0, fitting_width, fitting_height).unwrap();
    }
}

impl<'a> std::ops::Index<HexIndex> for Map<'a> {
    type Output = Tile;

    fn index(&self, index: HexIndex) -> &Self::Output {
        &self.tiles[index.get_index()]
    }
}

impl<'a> std::ops::IndexMut<HexIndex> for Map<'a> {
    fn index_mut(&mut self, index: HexIndex) -> &mut Self::Output {
        &mut self.tiles[index.get_index()]
    }
}