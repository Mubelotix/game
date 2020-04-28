use crate::{idx::*, units::*, *};
use wasm_game_lib::graphics::canvas::*;

const BORDER_STYLE: LineStyle = LineStyle {
    cap: LineCap::Round,
    join: LineJoin::Miter,
    size: 3.0,
    color: Color {
        alpha: 255,
        red: 180,
        green: 255,
        blue: 180,
    },
};

#[derive(PartialEq)]
pub struct Life {
    max: usize,
    current: usize,
}

impl Life {
    pub fn new(unit_type: &UnitType) -> Life {
        match unit_type {
            UnitType::Archer => Life { max: 2, current: 2 },
            UnitType::Knight => Life { max: 4, current: 4 },
            UnitType::Scout => Life { max: 3, current: 3 },
            UnitType::Barbarian => Life { max: 3, current: 3 },
        }
    }

    pub fn lose_life(&mut self, damage: usize) {
        if damage >= self.current {
            self.current = 0;
        } else {
            self.current -= damage;
        }
    }

    pub fn draw_on_canvas(&self, mut canvas: &mut Canvas, data: &DrawingData) {
        BORDER_STYLE.apply_on_canvas(&mut canvas);

        let (width, height) = match self.max {
            1 => (50.0, 40.0),
            2 => (65.0, 38.0),
            3 => (80.0, 36.0),
            4 => (95.0, 34.0),
            5 => (110.0, 32.0),
            6 => (125.0, 30.0),
            7 => (140.0, 28.0),
            _ => (155.0, 26.0),
        };

        let point_width = (width - 4.0) / self.max as f64;

        let coords = data.position.get_canvas_coords();
        let coords = Map::internal_coords_to_screen_coords(
            data.dimensions,
            data.margin,
            coords.0 as isize + (256 - width as isize) / 2,
            coords.1 as isize + 300,
        );
        let context = canvas.get_2d_canvas_rendering_context();
        context.begin_path();
        context.stroke_rect(
            coords.0 as f64,
            coords.1 as f64,
            width as f64 * data.factor,
            height as f64 * data.factor,
        );
        context.set_fill_style(&JsValue::from_str("rgb(24, 28, 39)"));
        context.fill_rect(
            coords.0 as f64,
            coords.1 as f64,
            width as f64 * data.factor,
            height as f64 * data.factor,
        );

        context.set_fill_style(&JsValue::from_str("rgb(0, 255, 100)"));
        for i in 0..self.current {
            context.fill_rect(
                coords.0 as f64 + (4.0 + i as f64 * point_width) * data.factor,
                coords.1 as f64 + 4.0 * data.factor,
                (point_width - 4.0) * data.factor,
                height as f64 * data.factor - 8.0 * data.factor,
            );
        }
        context.stroke();
    }
}
