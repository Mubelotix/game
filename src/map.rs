use wasm_game_lib::graphics::{drawable::*, canvas::Canvas, image::*, color::*};
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
pub const CANVAS_WIDTH: f64 = 9.0*253.0;
pub const CANVAS_HEIGHT: f64 = 8.0*256.0 + 10.0;


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
    textures: [&'a Image; TEXTURES_NUMBER],
    canvas: Canvas,
    pub dimensions: (usize, usize),
    pub margin: usize,
}

impl<'a> Map<'a> {
    #[allow(clippy::cognitive_complexity)]
    pub fn new(textures: [&'a Image; TEXTURES_NUMBER], dimensions: (usize, usize), margin: usize) -> Map {
        let mut canvas = Canvas::new();
        canvas.set_width(CANVAS_WIDTH as u32);
        canvas.set_height(CANVAS_HEIGHT as u32);
        let tiles = arr!((
            {
                let random = get_random(2);
                match random {
                    0 => Tile::GrassyPlain(get_random(3)),
                    1 => Tile::Forest(get_random(3)),
                    _ => Tile::Plain(get_random(3)),
                }
            }); 61);

        let mut map = Map {
            coords: (0,0),
            tiles,
            textures,
            canvas,
            dimensions,
            margin,
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
    }

    pub fn screen_coords_to_internal_canvas_coords(&self, x: usize, y: usize) -> (isize, isize) {
        let x = x as isize - self.margin as isize;
        let factor_width: f64 = (self.dimensions.0 - self.margin) as f64 / CANVAS_WIDTH;
        let factor_height = self.dimensions.1 as f64 / CANVAS_HEIGHT;
        let smaller_factor = if factor_width < factor_height {
            factor_width
        } else {
            factor_height
        };
        let fitting_width = CANVAS_WIDTH * smaller_factor;
        let fitting_height = CANVAS_HEIGHT * smaller_factor;
        let remaining_width = (self.dimensions.0 - self.margin) as f64 - fitting_width;
        let remaining_height = self.dimensions.1 as f64 - fitting_height;
        (((x as f64 - remaining_width / 2.0) / smaller_factor) as isize, ((y as f64 - remaining_height / 2.0) / smaller_factor) as isize)
    }

    pub fn internal_coords_to_screen_coords(dimensions: (u32, u32), margin: usize, x: isize, y: isize) -> (usize, usize) {
        let factor_width: f64 = (dimensions.0 as usize - margin) as f64 / CANVAS_WIDTH;
        let factor_height = dimensions.1 as f64 / CANVAS_HEIGHT;
        let smaller_factor = if factor_width < factor_height {
            factor_width
        } else {
            factor_height
        };
        let fitting_width = CANVAS_WIDTH * smaller_factor;
        let fitting_height = CANVAS_HEIGHT * smaller_factor;
        let remaining_width = (dimensions.0 as usize - margin) as f64 - fitting_width;
        let remaining_height = dimensions.1 as f64 - fitting_height;

        let mut x = x as f64 * smaller_factor;
        x += remaining_width/2.0;
        let mut y = y as f64 * smaller_factor;
        y += remaining_height/2.0;

        (x as usize + margin, y as usize)
    }
}

impl<'a> Drawable for Map<'a> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        let factor_width: f64 = (self.dimensions.0 - self.margin) as f64 / CANVAS_WIDTH;
        let factor_height = self.dimensions.1 as f64 / CANVAS_HEIGHT;
        let smaller_factor = if factor_width < factor_height {
            factor_width
        } else {
            factor_height
        };
        let fitting_width = CANVAS_WIDTH * smaller_factor;
        let fitting_height = CANVAS_HEIGHT * smaller_factor;
        let remaining_width = (self.dimensions.0 - self.margin) as f64 - fitting_width;
        let remaining_height = self.dimensions.1 as f64 - fitting_height;

        let canvas_element = canvas.get_2d_canvas_rendering_context();
        canvas_element.draw_image_with_html_canvas_element_and_dw_and_dh(self.canvas.get_canvas_element(), self.coords.0 as f64 + remaining_width / 2.0 + self.margin as f64, self.coords.1 as f64 + remaining_height / 2.0, fitting_width, fitting_height).unwrap();
    }
}

impl<'a> std::ops::Index<&HexIndex> for Map<'a> {
    type Output = Tile;

    fn index(&self, index: &HexIndex) -> &Self::Output {
        &self.tiles[index.get_index()]
    }
}

impl<'a> std::ops::IndexMut<&HexIndex> for Map<'a> {
    fn index_mut(&mut self, index: &HexIndex) -> &mut Self::Output {
        &mut self.tiles[index.get_index()]
    }
}