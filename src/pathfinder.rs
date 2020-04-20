use crate::{idx::HexIndex, map::*, units::Unit};
use wasm_game_lib::graphics::{drawable::*, canvas::*, color::Color};
use std::convert::TryInto;

pub fn find_route(map: &Map, starting_point: HexIndex, arrival_point: HexIndex) -> Option<Vec<HexIndex>> {
    let mut travel_time: [Option<usize>; 61] = [None; 61];
    travel_time[starting_point.get_index()] = Some(0);
    let mut paths: Vec<HexIndex> = vec![starting_point];

    while !paths.is_empty() && travel_time[arrival_point.get_index()].is_none() {
        let this_path = paths.remove(0);
        let travel_time_to_here = travel_time[this_path.get_index()].unwrap();

        if let Some(path) = this_path.get_right_neighbour() {
            if travel_time[path.get_index()].is_none() && map[&path].1.is_none() {
                travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                paths.push(path);
            }
        }
        if let Some(path) = this_path.get_left_neighbour() {
            if travel_time[path.get_index()].is_none() && map[&path].1.is_none() {
                travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                paths.push(path);
            }
        }
        if let Some(path) = this_path.get_top_right_neighbour() {
            if travel_time[path.get_index()].is_none() && map[&path].1.is_none() {
                travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                paths.push(path);
            }
        }
        if let Some(path) = this_path.get_top_left_neighbour() {
            if travel_time[path.get_index()].is_none() && map[&path].1.is_none() {
                travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                paths.push(path);
            }
        }
        if let Some(path) = this_path.get_bottom_right_neighbour() {
            if travel_time[path.get_index()].is_none() && map[&path].1.is_none() {
                travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                paths.push(path);
            }
        }
        if let Some(path) = this_path.get_bottom_left_neighbour() {
            if travel_time[path.get_index()].is_none() && map[&path].1.is_none() {
                travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                paths.push(path);
            }
        }
    }

    if travel_time[arrival_point.get_index()].is_some() {
        let mut full_path: Vec<HexIndex> = Vec::new();
        let mut last = arrival_point;
        while last != starting_point {
            full_path.push(last.clone());
    
            let neighbours = [last.get_left_neighbour().map(|n| (travel_time[n.get_index()], n)), last.get_right_neighbour().map(|n| (travel_time[n.get_index()], n)), last.get_top_left_neighbour().map(|n| (travel_time[n.get_index()], n)), last.get_top_right_neighbour().map(|n| (travel_time[n.get_index()], n)), last.get_bottom_left_neighbour().map(|n| (travel_time[n.get_index()], n)), last.get_bottom_right_neighbour().map(|n| (travel_time[n.get_index()], n))];
            let mut neighbours: Vec<(usize, &HexIndex)> = neighbours.iter().filter(|v| v.is_some()).map(|v| v.as_ref().unwrap()).collect::<Vec<&(Option<usize>, HexIndex)>>().iter().filter(|(t, _i)| t.is_some()).map(|(t, i)| (t.unwrap(), i)).collect::<Vec<(usize, &HexIndex)>>();
    
            let mut smaller = neighbours[0];
            neighbours.remove(0);
            for neighbour in neighbours {
                if neighbour.0 < smaller.0 {
                    smaller = neighbour;
                }
            }
    
            last = *smaller.1;
        }
        full_path.reverse();
        Some(full_path)
    } else {
        None
    }
}

pub struct Path {
    line_style: LineStyle,
    route: Option<(HexIndex, Option<Vec<HexIndex>>)>,
}

impl Path {
    pub fn new() -> Path {
        Path {
            line_style: LineStyle {
                cap: LineCap::Round,
                color: Color::new(66, 135, 245),
                join: LineJoin::Round,
                size: 14.0,
            },
            route: None
        }
    }

    pub fn handle_mouse_move(&mut self, map: &Map, x: u32, y: u32) {
        if let Some((unit, route)) = &mut self.route {
            let coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
            if let Some(index) = HexIndex::from_canvas_coords(coords) {
                *route = find_route(&map, *unit, index)
            } else {
                *route = None;
            }
        }
    }

    pub fn handle_mouse_click(&mut self, map: &mut Map, x: u32, y: u32) {
        let coords = map.screen_coords_to_internal_canvas_coords(x as usize, y as usize);
        if let Some(index) = HexIndex::from_canvas_coords(coords) {
            if let Some((unit, _route)) = &self.route {
                let tile = &mut map[&unit];
                let unit = tile.1.take();
                map[&index].1 = Some(unit.unwrap());
                self.route = None;
                map.update_canvas();
            } else if map[&index].1.is_some() {
                self.route = Some((index, None));
            }
        }
    }
}

impl Drawable for Path {
    fn draw_on_canvas(&self, mut canvas: &mut Canvas) {
        if let Some((start, Some(route))) = &self.route {
            let canvas_width = canvas.get_width();
            let canvas_height = canvas.get_height();
            let context = canvas.get_2d_canvas_rendering_context();
            context.begin_path();

            let (x, y) = start.get_canvas_coords();
            let (x, y) = Map::internal_coords_to_screen_coords((canvas_width, canvas_height), x as isize + 128, y as isize + 256);
            context.move_to(x as f64, y as f64);

            for tile in route {
                let (x, y) = tile.get_canvas_coords();
                let (x, y) = Map::internal_coords_to_screen_coords((canvas_width, canvas_height), x as isize + 128, y as isize + 256);

                context.line_to(x as f64, y as f64);
            }

            self.line_style.apply_on_canvas(&mut canvas);
            
            canvas.get_2d_canvas_rendering_context().stroke();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn travel() {
        //println!("{:?}", find_route(5.into(), 10.into()));
    }
}