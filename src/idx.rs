use std::convert::TryInto;

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

fn line_lenght(y: usize) -> usize {
    match y {
        0 => 5,
        1 => 6,
        2 => 7,
        3 => 8,
        4 => 9,
        5 => 8,
        6 => 7,
        7 => 6,
        8 => 5,
        y => panic!("{} out of bounds", y)
    }
}

/*fn idx_to_coords(idx: usize) -> (usize, usize) {
    (idx_to_x(idx), idx_to_y(idx))
}

fn coords_to_idx(x: usize, y: usize) -> usize {
    let index = match y {
        0 => 0,
        1 => 5,
        2 => 11,
        3 => 18,
        4 => 26,
        5 => 35,
        6 => 43,
        7 => 50,
        8 => 56,
        y => panic!("y:{} is out of bound", y),
    } + x;
    assert!(x < line_lenght(y));
    index
}*/

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct HexIndex {
    pos: (usize, usize, usize), // (x, y, index)
}

#[allow(dead_code)]
impl HexIndex {
    pub fn get_index(&self) -> usize {
        self.pos.2
    }

    pub fn get_x(&self) -> usize {
        self.pos.0
    }

    pub fn get_y(&self) -> usize {
        self.pos.1
    }

    pub fn get_coords(&self) -> (usize, usize) {
        (self.pos.0, self.pos.1)
    }

    pub fn right_bloc_present(&self) -> bool {
        self.pos.0 < line_lenght(self.pos.1) - 1
    }

    pub fn left_bloc_present(&self) -> bool {
        self.pos.0 != 0
    }

    pub fn top_left_bloc_present(&self) -> bool {
        self.pos.1 != 0 && (self.pos.0 != 0 || self.pos.1 > 4)
    }

    pub fn top_right_bloc_present(&self) -> bool {
        self.pos.1 != 0 && (self.pos.0 < line_lenght(self.pos.1) - 1|| self.pos.1 > 4)
    }

    pub fn bottom_left_bloc_present(&self) -> bool {
        self.pos.1 != 8 && (self.pos.0 != 0 || self.pos.1 < 4)
    }

    pub fn bottom_right_bloc_present(&self) -> bool {
        self.pos.1 != 8 && (self.pos.0 < line_lenght(self.pos.1) - 1 || self.pos.1 < 4)
    }

    pub fn neighbors_present(&self) -> (bool, bool, bool, bool, bool, bool) {
        (self.top_right_bloc_present(), self.right_bloc_present(), self.bottom_right_bloc_present(), self.bottom_left_bloc_present(), self.left_bloc_present(), self.top_left_bloc_present())
    }

    pub fn get_right_neighbour(&self) -> Option<HexIndex> {
        if self.right_bloc_present() {
            Some(HexIndex {
                pos: (self.pos.0 + 1, self.pos.1, self.pos.2 + 1)
            })
        } else {
            None
        }
    }

    pub fn get_left_neighbour(&self) -> Option<HexIndex> {
        if self.left_bloc_present() {
            Some(HexIndex {
                pos: (self.pos.0 - 1, self.pos.1, self.pos.2 - 1)
            })
        } else {
            None
        }
    }

    pub fn get_top_right_neighbour(&self) -> Option<HexIndex> {
        if self.top_right_bloc_present() {
            if self.pos.1 <= 4 {
                Some(HexIndex {
                    pos: (self.pos.0, self.pos.1 - 1, self.pos.2 - (line_lenght(self.pos.1) - 1))
                })
            } else {
                Some(HexIndex {
                    pos: (self.pos.0 + 1, self.pos.1 - 1, self.pos.2 - line_lenght(self.pos.1))
                })
            }
        } else {
            None
        }
    }

    pub fn get_top_left_neighbour(&self) -> Option<HexIndex> {
        if self.top_left_bloc_present() {
            if self.pos.1 <= 4 {
                Some(HexIndex {
                    pos: (self.pos.0 - 1, self.pos.1 - 1, self.pos.2 - line_lenght(self.pos.1))
                })
            } else {
                Some(HexIndex {
                    pos: (self.pos.0, self.pos.1 - 1, self.pos.2 - line_lenght(self.pos.1) - 1)
                })
            }
        } else {
            None
        }
    }

    pub fn get_bottom_right_neighbour(&self) -> Option<HexIndex> {
        if self.bottom_right_bloc_present() {
            if self.pos.1 < 4 {
                Some(HexIndex {
                    pos: (self.pos.0 + 1, self.pos.1 + 1, self.pos.2 + line_lenght(self.pos.1) + 1)
                })
            } else {
                Some(HexIndex {
                    pos: (self.pos.0, self.pos.1 + 1, self.pos.2 + line_lenght(self.pos.1))
                })
            }
        } else {
            None
        }
    }

    pub fn get_bottom_left_neighbour(&self) -> Option<HexIndex> {
        if self.bottom_left_bloc_present() {
            if self.pos.1 < 4 {
                Some(HexIndex {
                    pos: (self.pos.0, self.pos.1 + 1, self.pos.2 + line_lenght(self.pos.1))
                })
            } else {
                Some(HexIndex {
                    pos: (self.pos.0 - 1, self.pos.1 + 1, self.pos.2 + line_lenght(self.pos.1) - 1)
                })
            }
        } else {
            None
        }
    }

    pub fn get_canvas_coords(&self) -> (usize, usize) {
        let canvas_coords = (self.pos.0 * 253, self.pos.1 * 193);
        let offset = match self.pos.1 {
            0 | 8 => 4,
            1 | 7 => 3,
            2 | 6 => 2,
            3 | 5 => 1,
            _ => 0,
        } * 128;
        let x = canvas_coords.0 + offset;
        let y = canvas_coords.1;
        (x, y)
    }

    pub fn from_canvas_coords(mut coords: (isize, isize)) -> Option<HexIndex> {
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
        (coords.0 as usize, coords.1 as usize).try_into().ok()
    }
}

impl std::convert::TryFrom<usize> for HexIndex {
    type Error = ();

    fn try_from(index: usize) -> Result<Self, Self::Error> {
        if index > 60 {
            return Err(())
        }
        Ok(HexIndex {
            pos: (idx_to_x(index), idx_to_y(index), index)
        })
    }
}

impl std::convert::TryFrom<(usize, usize)> for HexIndex {
    type Error = ();

    fn try_from((x, y): (usize, usize)) -> Result<Self, Self::Error> {
        let index = match y {
            0 => 0,
            1 => 5,
            2 => 11,
            3 => 18,
            4 => 26,
            5 => 35,
            6 => 43,
            7 => 50,
            8 => 56,
            _ => return Err(())
        } + x;
        if x >= line_lenght(y) {
            return Err(())
        }
        Ok(HexIndex {
            pos: (x, y, index)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ()")]
    fn usize_into_hexindex_failing() {
        let _hexindex: HexIndex = 89.try_into().unwrap();
    }

    #[test]
    fn usize_into_hexindex_working() {
        let hexindex: HexIndex = 23.try_into().unwrap();
        assert_eq!(hexindex.get_coords(), (5, 3));
        assert_eq!(hexindex.get_index(), 23);

        let hexindex: HexIndex = 42.try_into().unwrap();
        assert_eq!(hexindex.get_coords(), (7, 5));
        assert_eq!(hexindex.get_index(), 42);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ()")]
    fn usize_tuple_into_hexindex_failing() {
        let _hexindex: HexIndex = (52, 23).try_into().unwrap();
    }

    #[test]
    fn usize_tuple_into_hexindex_working() {
        let hexindex: HexIndex = (7, 5).try_into().unwrap();
        assert_eq!(hexindex.get_coords(), (7, 5));
        assert_eq!(hexindex.get_index(), 42);
    }

    #[test]
    fn test_neighbors() {
        let top_top_left: HexIndex = (0,0).try_into().unwrap();
        let top_left: HexIndex = (0,2).try_into().unwrap();
        let left: HexIndex = (0,4).try_into().unwrap();
        let bottom_left: HexIndex = (0,6).try_into().unwrap();
        let bottom_bottom_left: HexIndex = (0,8).try_into().unwrap();

        let top_top_right: HexIndex = (4,0).try_into().unwrap();
        let top_right: HexIndex = (6,2).try_into().unwrap();
        let right: HexIndex = (8,4).try_into().unwrap();
        let bottom_right: HexIndex = (6,6).try_into().unwrap();
        let bottom_bottom_right: HexIndex = (4,8).try_into().unwrap();

        let top: HexIndex = (2,0).try_into().unwrap();
        let middle: HexIndex = (3,4).try_into().unwrap();
        let bottom: HexIndex = (3,8).try_into().unwrap();

        assert_eq!(top_top_left.neighbors_present(), (false, true, true, true, false, false));
        assert_eq!(top_left.neighbors_present(), (true, true, true, true, false, false));
        assert_eq!(left.neighbors_present(), (true, true, true, false, false, false));
        assert_eq!(bottom_left.neighbors_present(), (true, true, true, false, false, true));
        assert_eq!(bottom_bottom_left.neighbors_present(), (true, true, false, false, false, true));

        assert_eq!(top_top_right.neighbors_present(), (false, false, true, true, true, false));
        assert_eq!(top_right.neighbors_present(), (false, false, true, true, true, true));
        assert_eq!(right.neighbors_present(), (false, false, false, true, true, true));
        assert_eq!(bottom_right.neighbors_present(), (true, false, false, true, true, true));
        assert_eq!(bottom_bottom_right.neighbors_present(), (true, false, false, false, true, true));

        assert_eq!(top.neighbors_present(), (false, true, true, true, true, false));
        assert_eq!(middle.neighbors_present(), (true, true, true, true, true, true));
        assert_eq!(bottom.neighbors_present(), (true, true, false, false, true, true));

        assert_eq!(top_top_left.get_bottom_left_neighbour().unwrap().get_bottom_left_neighbour().unwrap(), top_left);
        assert_eq!(top_top_right.get_bottom_right_neighbour().unwrap().get_bottom_right_neighbour().unwrap(), top_right);
        assert_eq!(bottom_bottom_left.get_top_left_neighbour().unwrap().get_top_left_neighbour().unwrap(), bottom_left);
        assert_eq!(bottom_bottom_right.get_top_right_neighbour().unwrap().get_top_right_neighbour().unwrap(), bottom_right);
        assert_eq!(top_top_left.get_right_neighbour().unwrap().get_right_neighbour().unwrap(), top);
        assert_eq!(top.get_left_neighbour().unwrap().get_left_neighbour().unwrap(), top_top_left);
    }
}