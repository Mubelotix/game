use crate::{
    actions::*, button::*, idx::HexIndex, life::*, map::*, pathfinder::*, previsualisation::*,
    textbox::*, *,
};
use arr_macro::arr;
use std::{cell::RefCell, convert::TryInto};
use wasm_bindgen::JsValue;
use wasm_game_lib::graphics::{canvas::*, color::*, drawable::*, font::*, image::*};

const UNIT_NUMBER: usize = 7;

#[derive(PartialEq)]
pub enum UnitType {
    Archer,
    Knight,
    Scout,
    Barbarian,
    BarbarianVariant,
    ArmoredBarbarian,
    BarbarianLordOfDeath,
}

impl UnitType {
    pub fn get_texture_idx(&self) -> usize {
        match self {
            UnitType::Archer => 0,
            UnitType::Knight => 1,
            UnitType::Scout => 2,
            UnitType::Barbarian => 3,
            UnitType::BarbarianVariant => 4,
            UnitType::ArmoredBarbarian => 5,
            UnitType::BarbarianLordOfDeath => 6,
        }
    }

    pub fn moves_per_turn(&self) -> usize {
        match self {
            UnitType::Archer => 4,
            UnitType::Knight => 3,
            UnitType::Scout => 5,
            UnitType::Barbarian => 3,
            UnitType::BarbarianVariant => 4,
            UnitType::ArmoredBarbarian => 2,
            UnitType::BarbarianLordOfDeath => 2,
        }
    }

    pub fn is_barbarian(&self) -> bool {
        match self {
            UnitType::Archer | UnitType::Knight | UnitType::Scout => false,
            UnitType::Barbarian
            | UnitType::BarbarianVariant
            | UnitType::ArmoredBarbarian
            | UnitType::BarbarianLordOfDeath => true,
        }
    }
}

#[derive(PartialEq)]
pub struct Unit {
    pub unit_type: UnitType,
    pub remaining_moves: usize,
    pub attacks: (Attack, Attack),
    pub life: Life,
    pub action_remaining: bool,
    pub barbarian_next_action: Option<(Attack, Vec<Direction>)>,
}

impl Unit {
    pub fn new(unit_type: UnitType) -> Unit {
        Unit {
            remaining_moves: unit_type.moves_per_turn(),
            life: Life::new(&unit_type),
            action_remaining: true,
            attacks: match unit_type {
                UnitType::Archer => (Attack::VolleyOfArrows, Attack::Heal),
                UnitType::Scout => (Attack::StickKnock, Attack::Heal),
                UnitType::Knight => (Attack::OffensiveSwordFight, Attack::DefensiveSwordFight),
                UnitType::Barbarian => (Attack::StickKnock, Attack::Heal),
                UnitType::BarbarianVariant => (Attack::StickKnock, Attack::Heal),
                UnitType::ArmoredBarbarian => (Attack::StickKnock, Attack::Heal),
                UnitType::BarbarianLordOfDeath => (Attack::StickKnock, Attack::Heal),
            },
            barbarian_next_action: None,
            unit_type,
        }
    }

    pub fn get_remaining_moves(&self) -> usize {
        self.remaining_moves
    }

    pub fn draw_on_canvas(
        &self,
        mut canvas: &mut Canvas,
        data: &DrawingData,
        textures: [&Image; UNIT_NUMBER],
    ) {
        let coords = data.position.get_canvas_coords();
        let coords = Map::internal_coords_to_screen_coords(
            data.dimensions,
            data.margin,
            coords.0 as isize + 50,
            coords.1 as isize + 160,
        );

        canvas
            .context
            .draw_image_with_html_image_element_and_dw_and_dh(
                textures[self.unit_type.get_texture_idx()].get_html_element(),
                coords.0 as f64,
                coords.1 as f64,
                150.0 * data.factor,
                150.0 * data.factor,
            )
            .unwrap();

        self.life.draw_on_canvas(&mut canvas, data);
    }
}

struct SelectedUnit<'a> {
    pub position: HexIndex,
    pub reachable_tiles: [Option<usize>; 61],
    pub action_textboxes: (TextBox<'a>, TextBox<'a>),
    pub previsualisation: Previsualisation,
}

pub struct Units<'a> {
    units: [Option<Unit>; 61],
    textures: [&'a Image; UNIT_NUMBER],
    overground: [&'a Image; 2],
    margin: usize,
    line_style: LineStyle,
    next_turn_button: Button<'a>,
    selected_unit: Option<SelectedUnit<'a>>,
    barbarian_actions: Vec<(HexIndex, PrevisualisationItem)>,
    animation_frame: RefCell<u64>,
}

impl<'a> Units<'a> {
    pub fn new(
        textures: [&'a Image; UNIT_NUMBER],
        overground: [&'a Image; 2],
        margin: usize,
        arial: &'a Font,
    ) -> Units<'a> {
        Units {
            units: arr!(None;61),
            textures,
            margin,
            overground,
            next_turn_button: Button::new((10.0, 10.0), None, &arial, String::from("Next turn")),
            line_style: LineStyle {
                cap: LineCap::Round,
                color: Color::new(66, 135, 245),
                join: LineJoin::Round,
                size: 14.0,
            },
            selected_unit: None,
            barbarian_actions: Vec::new(),
            animation_frame: RefCell::new(0),
        }
    }

    pub fn get(&self, idx: &HexIndex) -> &Option<Unit> {
        &self.units[idx.get_index()]
    }

    pub fn _get_mut(&mut self, idx: &HexIndex) -> &mut Option<Unit> {
        &mut self.units[idx.get_index()]
    }

    pub fn set(&mut self, idx: &HexIndex, unit: Option<Unit>) {
        self.units[idx.get_index()] = unit;
    }

    pub fn set_margin(&mut self, margin: usize) {
        self.margin = margin;
        if let Some(selected_unit) = &mut self.selected_unit {
            selected_unit.action_textboxes.0.set_width(margin - 20);
            selected_unit.action_textboxes.1.set_width(margin - 20);
        }
    }

    pub fn handle_resize_event(&mut self, canvas: &mut Canvas) {
        let canvas_height = canvas.get_height() as usize;
        if let Some(selected_unit) = &mut self.selected_unit {
            selected_unit
                .action_textboxes
                .0
                .set_y(canvas_height - selected_unit.action_textboxes.0.get_height());
            selected_unit.action_textboxes.1.set_y(
                canvas_height
                    - selected_unit.action_textboxes.0.get_height()
                    - selected_unit.action_textboxes.1.get_height(),
            );
        }
    }

    pub fn apply_barbarian_actions(&mut self, map: &mut Map) {
        Attack::apply(self.barbarian_actions.split_off(0), map, &mut self.units)
    }

    pub fn make_ai_play(&mut self) {
        for unit in self
            .units
            .iter_mut()
            .filter_map(|u| u.as_mut())
            .filter(|u| u.unit_type.is_barbarian())
        {
            unit.barbarian_next_action = Some((Attack::StickKnock, vec![!Direction::BottomLeft]));
        }
    }

    pub fn update_barbarian_actions(&mut self, map: &Map) {
        let mut consequences = Vec::new();
        for (position, (action, directions)) in self
            .units
            .iter()
            .enumerate()
            .filter(|u| u.1.is_some())
            .map(|u| (u.0, u.1.as_ref().unwrap()))
            .filter(|u| u.1.barbarian_next_action.is_some())
            .map(|u| (u.0, u.1.barbarian_next_action.as_ref().unwrap()))
        {
            let position: HexIndex = position.try_into().unwrap();
            let mut target = Some(position);
            for direction in directions {
                if let Some(target2) = target {
                    target = target2.get_neighbour(&direction);
                }
            }
            if let Some(target) = target {
                consequences.append(&mut action.get_consequences(
                    map,
                    &self.units,
                    &position,
                    &target,
                ));
            }
        }
        self.barbarian_actions = consequences;
    }

    pub fn handle_mouse_move(&mut self, map: &Map, x: u32, y: u32) {
        let coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
        if let Some(index) = HexIndex::from_canvas_coords(coords) {
            // get the tile hovered by the mouse
            match self {
                Units {
                    selected_unit:
                        Some(SelectedUnit {
                            position,
                            reachable_tiles,
                            previsualisation: Previsualisation::Movement(previsualisation),
                            ..
                        }),
                    ..
                } => *previsualisation = find_route(&reachable_tiles, *position, index),
                Units {
                    selected_unit:
                        Some(SelectedUnit {
                            position,
                            previsualisation:
                                Previsualisation::Action(actions, targets, consequences),
                            ..
                        }),
                    units,
                    ..
                } => {
                    *consequences = if *actions && targets.contains(&index) {
                        units[position.get_index()]
                            .as_ref()
                            .unwrap()
                            .attacks
                            .1
                            .get_consequences(&map, units, position, &index)
                    } else if targets.contains(&index) {
                        units[position.get_index()]
                            .as_ref()
                            .unwrap()
                            .attacks
                            .0
                            .get_consequences(&map, units, position, &index)
                    } else {
                        Vec::new()
                    };
                }
                _ => (),
            }
        }
    }

    pub fn move_selected_unit(&mut self, to: &HexIndex) {
        if !self[&self.selected_unit.as_ref().unwrap().position]
            .unit_type
            .is_barbarian()
        {
            let selected_unit = self.selected_unit.as_mut().unwrap();
            let mut unit = self.units[selected_unit.position.get_index()]
                .take()
                .unwrap();
            unit.remaining_moves -= selected_unit.reachable_tiles[to.get_index()].unwrap();
            self.set(&to, Some(unit));
        }
        self.selected_unit = None;
    }

    pub fn apply_action_of_selected_unit(&mut self, target: &HexIndex, mut map: &mut Map) {
        if let Units {
            units,
            selected_unit:
                Some(SelectedUnit {
                    position,
                    previsualisation: Previsualisation::Action(_action, targets, consequences),
                    ..
                }),
            ..
        } = self
        {
            if targets.contains(&target)
                && !units[position.get_index()]
                    .as_ref()
                    .unwrap()
                    .unit_type
                    .is_barbarian()
            {
                Attack::apply(consequences.split_off(0), &mut map, units);
                units[position.get_index()]
                    .as_mut()
                    .unwrap()
                    .action_remaining = false;
            }
            self.selected_unit = None;
        }
    }

    pub fn select_unit(
        &mut self,
        index: HexIndex,
        mut canvas: &mut Canvas,
        arial: &'a Font,
        map: &Map,
    ) {
        let canvas_height = canvas.get_height() as usize;
        let mut t1 = TextBox::new(
            (10.0, 200.0),
            self.margin - 20,
            &arial,
            self[&index].attacks.0.get_description(),
        );
        let mut t2 = TextBox::new(
            (10.0, 300.0),
            self.margin - 20,
            &arial,
            self[&index].attacks.1.get_description(),
        );
        t1.init(&mut canvas);
        t2.init(&mut canvas);
        t2.set_y(canvas_height - t2.get_height());
        t1.set_y(canvas_height - t2.get_height() - t1.get_height());
        self.selected_unit = Some(SelectedUnit {
            position: index,
            previsualisation: Previsualisation::Movement(None),
            reachable_tiles: compute_travel_time(
                &self,
                &map,
                index,
                self[&index].get_remaining_moves(),
            ),
            action_textboxes: (t1, t2),
        });
    }

    pub fn action_selection(&mut self, mouse_position: (u32, u32), map: &Map) -> bool {
        if let Some(selected_unit) = &self.selected_unit {
            if selected_unit
                .action_textboxes
                .0
                .is_hover_with_mouse_position(mouse_position)
                && self[&self.selected_unit.as_ref().unwrap().position].action_remaining
            {
                let targets = self[&selected_unit.position]
                    .attacks
                    .0
                    .get_potential_targets(&map, &self, &selected_unit.position);
                self.selected_unit.as_mut().unwrap().previsualisation =
                    Previsualisation::Action(false, targets, Vec::new());
                true
            } else if selected_unit
                .action_textboxes
                .1
                .is_hover_with_mouse_position(mouse_position)
                && self[&self.selected_unit.as_ref().unwrap().position].action_remaining
            {
                let targets = self[&selected_unit.position]
                    .attacks
                    .1
                    .get_potential_targets(&map, &self, &selected_unit.position);
                self.selected_unit.as_mut().unwrap().previsualisation =
                    Previsualisation::Action(true, targets, Vec::new());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn next_turn(&mut self, mouse_position: (u32, u32), map: &mut Map) -> bool {
        if self
            .next_turn_button
            .is_hover_with_mouse_position(mouse_position)
        {
            for unit in self.units.iter_mut().filter_map(|u| u.as_mut()) {
                unit.remaining_moves = unit.unit_type.moves_per_turn();
                unit.action_remaining = true;
            }

            self.apply_barbarian_actions(map);
            self.make_ai_play();
            self.update_barbarian_actions(&map);

            true
        } else {
            false
        }
    }

    pub fn handle_mouse_click(
        &mut self,
        mut map: &mut Map,
        x: u32,
        y: u32,
        arial: &'a Font,
        canvas: &mut Canvas,
    ) {
        // get the tile hovered by the mouse
        let coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
        if let Some(clicked_tile_idx) = HexIndex::from_canvas_coords(coords) {
            if let Some(selected_unit) = &self.selected_unit {
                if (self.get(&clicked_tile_idx).is_none()
                    || clicked_tile_idx == selected_unit.position)
                    && selected_unit.previsualisation.is_movement_some()
                {
                    self.move_selected_unit(&clicked_tile_idx);
                    self.update_barbarian_actions(&map);
                } else if let Previsualisation::Action(_, _, _) = &selected_unit.previsualisation {
                    self.apply_action_of_selected_unit(&clicked_tile_idx, &mut map);
                    self.update_barbarian_actions(&map);
                }
            } else if self.get(&clicked_tile_idx).is_some() {
                self.select_unit(clicked_tile_idx, canvas, arial, map);
            }
        } else if !self.action_selection((x, y), map) {
            self.next_turn((x, y), &mut map);
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
        *self.animation_frame.borrow_mut() += 1;
        let animation_frame: u64 = *self.animation_frame.borrow();
        let dimensions = (canvas.get_width(), canvas.get_height());
        let factor_width: f64 = (dimensions.0 as usize - self.margin) as f64 / CANVAS_WIDTH;
        let factor_height = dimensions.1 as f64 / CANVAS_HEIGHT;
        let factor = if factor_width < factor_height {
            factor_width
        } else {
            factor_height
        };

        let drawing_data = DrawingData {
            factor,
            dimensions,
            margin: self.margin,
            position: &0.try_into().unwrap(),
            animation_frame,
        };

        // draw units
        for (idx, unit) in self.units.iter().enumerate().filter(|(_i, u)| u.is_some()) {
            let unit = unit.as_ref().unwrap();
            unit.draw_on_canvas(
                &mut canvas,
                &DrawingData {
                    position: &idx.try_into().unwrap(),
                    ..drawing_data
                },
                self.textures,
            );
        }

        // draw barbabarian actions
        for (position, consequence) in &self.barbarian_actions {
            consequence.draw_on_canvas(
                &mut canvas,
                &DrawingData {
                    position: &position,
                    ..drawing_data
                },
            )
        }

        // draw selected unit info
        if let Some(selected_unit) = &self.selected_unit {
            let canvas_width = canvas.get_width();
            let canvas_height = canvas.get_height();

            if let Previsualisation::Action(_action, targets, consequences) =
                &selected_unit.previsualisation
            {
                for target in targets {
                    let (x, y) = target.get_canvas_coords();
                    let (x, y) = Map::internal_coords_to_screen_coords(
                        (canvas_width, canvas_height),
                        self.margin,
                        x as isize,
                        y as isize,
                    );

                    canvas
                        .get_2d_canvas_rendering_context()
                        .draw_image_with_html_image_element_and_dw_and_dh(
                            self.overground[1].get_html_element(),
                            x as f64,
                            y as f64,
                            256.0 * factor,
                            384.0 * factor,
                        )
                        .unwrap();
                }

                // draw consequences of the selected action
                for (position, consequence) in consequences {
                    consequence.draw_on_canvas(
                        &mut canvas,
                        &DrawingData {
                            position,
                            ..drawing_data
                        },
                    )
                }
            } else {
                if let Previsualisation::Movement(Some(route)) = &selected_unit.previsualisation {
                    let context = canvas.get_2d_canvas_rendering_context();
                    context.begin_path();

                    let (x, y) = selected_unit.position.get_canvas_coords();
                    let (x, y) = Map::internal_coords_to_screen_coords(
                        (canvas_width, canvas_height),
                        self.margin,
                        x as isize + 128,
                        y as isize + 256,
                    );
                    context.move_to(x as f64, y as f64);

                    for tile in route {
                        let (x, y) = tile.get_canvas_coords();
                        let (x, y) = Map::internal_coords_to_screen_coords(
                            (canvas_width, canvas_height),
                            self.margin,
                            x as isize + 128,
                            y as isize + 256,
                        );

                        context.line_to(x as f64, y as f64);
                    }

                    self.line_style.apply_on_canvas(&mut canvas);

                    canvas.get_2d_canvas_rendering_context().stroke();
                }

                for reachable_tile in selected_unit
                    .reachable_tiles
                    .iter()
                    .enumerate()
                    .filter(|v| v.1.is_none())
                    .map(|v| {
                        let v: HexIndex = v.0.try_into().unwrap();
                        v
                    })
                {
                    let (x, y) = reachable_tile.get_canvas_coords();
                    let (x, y) = Map::internal_coords_to_screen_coords(
                        (canvas_width, canvas_height),
                        self.margin,
                        x as isize,
                        y as isize,
                    );

                    canvas
                        .get_2d_canvas_rendering_context()
                        .draw_image_with_html_image_element_and_dw_and_dh(
                            self.overground[0].get_html_element(),
                            x as f64,
                            y as f64,
                            256.0 * factor,
                            384.0 * factor,
                        )
                        .unwrap();
                }
            }

            canvas.draw(&selected_unit.action_textboxes.0);
            canvas.draw(&selected_unit.action_textboxes.1);
        }

        canvas.draw(&self.next_turn_button);
    }
}
