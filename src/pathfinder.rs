use crate::{idx::HexIndex, map::*, units::*};
use wasm_game_lib::graphics::{drawable::*, canvas::*, color::Color};
use std::convert::TryInto;

pub fn compute_travel_time(units: &Units, _map: &Map, starting_point: HexIndex, max_moves: usize) -> [Option<usize>; 61] {
    let mut travel_time: [Option<usize>; 61] = [None; 61];
    travel_time[starting_point.get_index()] = Some(0);
    let mut paths: Vec<HexIndex> = vec![starting_point];

    while !paths.is_empty() {
        let this_path = paths.remove(0);
        let travel_time_to_here = travel_time[this_path.get_index()].unwrap();

        if travel_time_to_here < max_moves {
            if let Some(path) = this_path.get_right_neighbour() {
                if travel_time[path.get_index()].is_none() && units.get(&path).is_none() {
                    travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                    paths.push(path);
                }
            }
            if let Some(path) = this_path.get_left_neighbour() {
                if travel_time[path.get_index()].is_none() && units.get(&path).is_none() {
                    travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                    paths.push(path);
                }
            }
            if let Some(path) = this_path.get_top_right_neighbour() {
                if travel_time[path.get_index()].is_none() && units.get(&path).is_none() {
                    travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                    paths.push(path);
                }
            }
            if let Some(path) = this_path.get_top_left_neighbour() {
                if travel_time[path.get_index()].is_none() && units.get(&path).is_none() {
                    travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                    paths.push(path);
                }
            }
            if let Some(path) = this_path.get_bottom_right_neighbour() {
                if travel_time[path.get_index()].is_none() && units.get(&path).is_none() {
                    travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                    paths.push(path);
                }
            }
            if let Some(path) = this_path.get_bottom_left_neighbour() {
                if travel_time[path.get_index()].is_none() && units.get(&path).is_none() {
                    travel_time[path.get_index()] = Some(travel_time_to_here + 1);
                    paths.push(path);
                }
            }
        }
    }

    travel_time
}

pub fn find_route(travel_time: &[Option<usize>; 61], starting_point: HexIndex, arrival_point: HexIndex) -> Option<Vec<HexIndex>> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn travel() {
        //println!("{:?}", find_route(5.into(), 10.into()));
    }
}