use raylib::prelude::*;

use crate::expr::Expr;
use crate::expr::eval::ExprEvaluator;

pub struct PlotWidget {
    pub rect: Rectangle,
    pub graph_color: Color,
    pub axis_color: Color,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl PlotWidget {
    pub const BORDER: f32 = 2.0;

    pub fn new(rect: Rectangle) -> Self {
        PlotWidget {
            rect,
            graph_color: Color::BLACK,
            axis_color: Color::BLUE,
            min_x: -1.0,
            min_y: -1.0,
            max_x: 1.0,
            max_y: 1.0,
        }
    }

    pub fn want_focus(&self) -> bool {
        false
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, rect: Rectangle, expr: Option<&Expr>, eval: &mut ExprEvaluator) {
        if rect.width.floor() <= 0.0 || rect.height.floor() < 0.0 { return; }

        d.clear_background(Color::RED);

        let expr = if let Some(expr) = expr {
            expr
        } else {
            d.draw_rectangle_rec(rect, Color::new(224, 224, 224, 255));
            d.draw_rectangle_lines_ex(rect, Self::BORDER, Color::BLACK);
            return;
        };

        d.draw_rectangle_rec(rect, Color::WHITE);
        d.draw_rectangle_lines_ex(rect, Self::BORDER, Color::BLACK);
        let n_samples = rect.width.max(0.0).floor() as u32;
        let x_step = (self.max_x - self.min_x) / n_samples as f64;

        let dx = rect.width / (self.max_x - self.min_x) as f32;
        let dy = rect.height / (self.max_y - self.min_y) as f32;

        let x0 = rect.x + (0.0 - self.min_x) as f32 * dx;
        let y0 = rect.y + (0.0 - self.min_y) as f32 * dy;
        d.draw_line_ex(Vector2::new(rect.x, y0), Vector2::new(rect.x + rect.width, y0), 2.0, self.axis_color);
        d.draw_line_ex(Vector2::new(x0, rect.y), Vector2::new(x0, rect.y + rect.height), 2.0, self.axis_color);

        let mut last_x = 0.0;
        let mut last_y = 0.0;
        for i in 0..n_samples {
            let x = self.min_x + i as f64 * x_step;
            eval.set_var("x", x);
            let y = eval.eval(expr);

            let px = rect.x + (x - self.min_x) as f32 * dx;
            let py = rect.y + (y - self.min_y) as f32 * dy;
            //println!("{},{}", px, py);
            if i > 0 {
                d.draw_line_ex(Vector2::new(last_x, last_y), Vector2::new(px, py), 2.0, self.graph_color);
            }
            last_x = px;
            last_y = py;
        }
    }
}
