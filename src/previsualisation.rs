use crate::{idx::*, map::*, *};

pub enum Previsualisation {
    Movement(Option<Vec<HexIndex>>),
    Action(bool, Vec<HexIndex>, Vec<(HexIndex, PrevisualisationItem)>)
}

impl Previsualisation {
    pub fn is_movement_some(&self) -> bool {
        match self {
            Previsualisation::Movement(r) => r.is_some(),
            _ => false,
        }
    }
}

pub enum PrevisualisationItem {
    PushArrow(Direction)
}

const PUSH_ARROW_STYLE: LineStyle = LineStyle {
    cap: LineCap::Round,
    color: Color {
        alpha: 255,
        red: 255,
        green: 255,
        blue: 30,
    },
    join: LineJoin::Miter,
    size: 10.0,
};

impl PrevisualisationItem {
    pub fn draw_on_canvas(&self, mut canvas: &mut Canvas, data: &DrawingData) {
        match self {
            PrevisualisationItem::PushArrow(direction) => {
                if let Some(point2) = data.position.get_neighbour(direction) {
                    let context = canvas.get_2d_canvas_rendering_context();
                    context.begin_path();
    
                    let (x, y) = data.position.get_canvas_coords();
                    let (x, y) = match direction {
                        Direction::TopLeft =>   (x - 35, y - 35),
                        Direction::TopRight =>  (x + 35, y - 35),
                        Direction::Right =>     (x + 50, y),
                        Direction::BottomRight=>(x + 35, y + 35),
                        Direction::BottomLeft=> (x - 35, y + 35),
                        Direction::Left =>      (x - 50, y),
                    };
                    let (x, y) = Map::internal_coords_to_screen_coords(data.dimensions, data.margin, x as isize + 128, y as isize + 256);
                    context.move_to(x as f64, y as f64);
    
                    let (x, y) = point2.get_canvas_coords();
                    let (x, y) = match direction {
                        Direction::TopLeft =>   (x + 35, y + 35),
                        Direction::TopRight =>  (x - 35, y + 35),
                        Direction::Right =>     (x - 50, y),
                        Direction::BottomRight=>(x - 35, y - 35),
                        Direction::BottomLeft=> (x + 35, y - 35),
                        Direction::Left =>      (x + 50, y),
                    };
                    let (x, y) = Map::internal_coords_to_screen_coords(data.dimensions, data.margin, x as isize + 128, y as isize + 256);
                    context.line_to(x as f64, y as f64);

                    // TODO fix strange arrows
                    let (xo, yo) = match direction {
                        Direction::TopLeft =>   (x as f64, y as f64 + 35.0 * data.factor),
                        Direction::TopRight =>  (x as f64, y as f64 + 35.0 * data.factor),
                        Direction::Right =>     (x as f64 - 35.0 * data.factor, y as f64 - 35.0 * data.factor),
                        Direction::BottomRight=>(x as f64, y as f64 - 35.0 * data.factor),
                        Direction::BottomLeft=> (x as f64, y as f64 - 35.0 * data.factor),
                        Direction::Left =>      (x as f64 + 35.0 * data.factor, y as f64 + 35.0 * data.factor),
                    };
                    let (xo2, yo2) = match direction {
                        Direction::TopLeft =>   (x as f64 + 35.0 * data.factor, y as f64),
                        Direction::TopRight =>  (x as f64 - 35.0 * data.factor, y as f64),
                        Direction::Right =>     (x as f64 - 35.0 * data.factor, y as f64 + 35.0 * data.factor),
                        Direction::BottomRight=>(x as f64 - 35.0 * data.factor, y as f64),
                        Direction::BottomLeft=> (x as f64 + 35.0 * data.factor, y as f64),
                        Direction::Left =>      (x as f64 + 35.0 * data.factor, y as f64 - 35.0 * data.factor),
                    };
                    context.move_to(xo, yo);
                    context.line_to(x as f64, y as f64);
                    context.move_to(xo2, yo2);
                    context.line_to(x as f64, y as f64);
    
                    PUSH_ARROW_STYLE.apply_on_canvas(&mut canvas);
                    
                    canvas.get_2d_canvas_rendering_context().stroke();
                }
            }
        }
    }
}