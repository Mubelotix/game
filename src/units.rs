use crate::{idx::HexIndex, map::*};
use arr_macro::arr;
use wasm_game_lib::graphics::{canvas::*, image::*, drawable::*};
use std::convert::TryInto;

#[derive(PartialEq)]
pub enum UnitType {
    Archer,
    Knight,
    Scout,
    Barbarian,
}

impl UnitType {
    pub fn get_texture_idx(&self) -> usize {
        match self {
            UnitType::Archer => 0,
            UnitType::Knight => 1,
            UnitType::Scout => 2,
            UnitType::Barbarian => 3,
        }
    }

    pub fn moves_per_turn(&self) -> usize {
        match self {
            UnitType::Archer => 4,
            UnitType::Knight => 3,
            UnitType::Scout => 5,
            UnitType::Barbarian => 3
        }
    }
}

#[derive(PartialEq)]
pub struct Unit {
    pub unit_type: UnitType,
    remaining_moves: usize,
}

impl Unit {
    pub fn new(unit_type: UnitType) -> Unit {
        Unit {
            remaining_moves: unit_type.moves_per_turn(),
            unit_type,
        }
    }

    pub fn get_remaining_moves(&self) -> usize {
        self.remaining_moves
    }
}

pub struct Units<'a> {
    units: [Option<Unit>; 61],
    textures: [&'a Image; 4],
    pub margin: usize,
}

impl<'a> Units<'a> {
    pub fn new(textures: [&'a Image; 4], margin: usize) -> Units {
        Units {
            units: arr!(None;61),
            textures,
            margin,
        }
    }

    pub fn get(&self, idx: &HexIndex) -> &Option<Unit> {
        &self.units[idx.get_index()]
    }

    pub fn set(&mut self, idx: &HexIndex, unit: Option<Unit>) {
        self.units[idx.get_index()] = unit;
    }

    pub fn get_mut(&mut self, idx: &HexIndex) -> &mut Option<Unit> {
        &mut self.units[idx.get_index()]
    }
}

impl<'a> std::ops::Index<&HexIndex> for Units<'a> {
    type Output = Unit;

    fn index(&self, index: &HexIndex) -> &Self::Output {
        &self.units[index.get_index()].as_ref().unwrap()
    }
}

impl<'a> std::ops::IndexMut<&HexIndex> for Units<'a> {
    fn index_mut(&mut self, index: &HexIndex) -> &mut Self::Output {
        self.units[index.get_index()].as_mut().unwrap()
    }
}

impl<'a> Drawable for Units<'a> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        let dimensions = (canvas.get_width(), canvas.get_height());
        let factor_width: f64 = dimensions.0 as f64 / CANVAS_WIDTH;
        let factor_height = dimensions.1 as f64 / CANVAS_HEIGHT;
        let factor = if factor_width < factor_height {
            factor_width
        } else {
            factor_height
        };

        let context = canvas.get_2d_canvas_rendering_context();

        for (idx, unit) in self.units.iter().enumerate().filter(|(_i, u)| u.is_some()) {
            let unit = unit.as_ref().unwrap();
            let coords: HexIndex = idx.try_into().unwrap();
            let coords = coords.get_canvas_coords();
            let coords = Map::internal_coords_to_screen_coords(dimensions, self.margin, coords.0 as isize + 50, coords.1 as isize + 160);

            context.draw_image_with_html_image_element_and_dw_and_dh(self.textures[unit.unit_type.get_texture_idx()].get_html_element(), coords.0 as f64, coords.1 as f64, 150.0 * factor, 150.0 * factor).unwrap();
        }
    }
}