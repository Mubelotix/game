use crate::{idx::HexIndex, map::*, pathfinder::find_route};
use arr_macro::arr;
use wasm_game_lib::graphics::{canvas::*, image::*, drawable::*, color::*};
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
    pub attacks: [Attack; 2],
}

impl Unit {
    pub fn new(unit_type: UnitType) -> Unit {
        Unit {
            remaining_moves: unit_type.moves_per_turn(),
            unit_type,
            attacks: [Attack::StickKnock, Attack::StickKnock],
        }
    }

    pub fn get_remaining_moves(&self) -> usize {
        self.remaining_moves
    }
}

#[derive(PartialEq, Clone)]
pub enum Attack {
    StickKnock,
}

impl Attack {
    pub fn apply(&self, map: &mut Map, units: &mut Units) {
        unimplemented!();
    }

    pub fn get_description(&self) -> &str {
        match self {
            Attack::StickKnock => "Hit an adjacent unit (1 damage) and push it away.",
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Attack::StickKnock => "Stick Knock",
        }
    }

    pub fn get_icon_idx(&self) -> usize {
        unimplemented!();
    }

    pub fn can_be_used_by_unit(&self, unit: &UnitType) -> bool {
        match self {
            Attack::StickKnock => true,
        }
    }
}

pub struct Units<'a> {
    units: [Option<Unit>; 61],
    textures: [&'a Image; 4],
    margin: usize,
    line_style: LineStyle,
    selected_unit: Option<(HexIndex, Option<Vec<HexIndex>>)>
}

impl<'a> Units<'a> {
    pub fn new(textures: [&'a Image; 4], margin: usize) -> Units {
        Units {
            units: arr!(None;61),
            textures,
            margin,
            line_style: LineStyle {
                cap: LineCap::Round,
                color: Color::new(66, 135, 245),
                join: LineJoin::Round,
                size: 14.0,
            },
            selected_unit: None,
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

    pub fn set_margin(&mut self, margin: usize) {
        self.margin = margin;
        //TODO OTHER
    }

    pub fn handle_mouse_move(&mut self, map: &Map, x: u32, y: u32) {
        if let Some((unit, _route)) = &self.selected_unit {
            let coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
            if let Some(index) = HexIndex::from_canvas_coords(coords) { // get the tile hovered by the mouse
                self.selected_unit.as_mut().unwrap().1 = find_route(&self, &map, *unit, index, self[unit].get_remaining_moves());
            } else {
                self.selected_unit.as_mut().unwrap().1 = None;
            }
        }
    }

    

    pub fn handle_mouse_click(&mut self, map: &Map, x: u32, y: u32) {
        let coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
        if let Some(clicked_tile_idx) = HexIndex::from_canvas_coords(coords) { // get the tile hovered by the mouse
            if let Some((selected_unit_idx, route)) = &self.selected_unit { // if a unit is selected
                if self.get(&clicked_tile_idx).is_none() && route.is_some() {
                    let selected_unit = self.units[selected_unit_idx.get_index()].take().unwrap();
                    self.set(&clicked_tile_idx, Some(selected_unit));
                    self.selected_unit = None;
                }
            } else if self.get(&clicked_tile_idx).is_some() { // if no unit is selected but a unit has been clicked
                self.selected_unit = Some((clicked_tile_idx, None));
            }
        }
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
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
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

        if let Some((start, Some(route))) = &self.selected_unit {
            let canvas_width = canvas.get_width();
            let canvas_height = canvas.get_height();
            let context = canvas.get_2d_canvas_rendering_context();
            context.begin_path();

            let (x, y) = start.get_canvas_coords();
            let (x, y) = Map::internal_coords_to_screen_coords((canvas_width, canvas_height), self.margin, x as isize + 128, y as isize + 256);
            context.move_to(x as f64, y as f64);

            for tile in route {
                let (x, y) = tile.get_canvas_coords();
                let (x, y) = Map::internal_coords_to_screen_coords((canvas_width, canvas_height), self.margin, x as isize + 128, y as isize + 256);

                context.line_to(x as f64, y as f64);
            }

            self.line_style.apply_on_canvas(&mut canvas);
            
            canvas.get_2d_canvas_rendering_context().stroke();
        }
    }
}