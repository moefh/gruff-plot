use raylib::prelude::*;

use crate::expr::Expr;
use crate::expr::eval::ExprEvaluator;

use super::WidgetBehavior;

pub struct PlotWidget {
    pub rect: Rectangle,
    pub graph_color: Color,
    pub axis_color: Color,
    pub x_left: f64,
    pub x_right: f64,
    pub y_top: f64,
    pub y_bottom: f64,
}

impl WidgetBehavior for PlotWidget {
}

impl PlotWidget {
    pub const BORDER: f32 = 2.0;

    pub fn new(rect: Rectangle) -> Self {
        PlotWidget {
            rect,
            graph_color: Color::BLACK,
            axis_color: Color::BLUE,
            x_left: -6.0,
            x_right: 6.0,
            y_top: 4.0,
            y_bottom: -4.0,
        }
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
        let x_step = (self.x_right - self.x_left) / n_samples as f64;

        let dx = rect.width / (self.x_right - self.x_left) as f32;
        let dy = rect.height / (self.y_top - self.y_bottom) as f32;

        let x0 = rect.x + (0.0 - self.x_left) as f32 * dx;
        let y0 = rect.y + (0.0 - self.y_bottom) as f32 * dy;
        d.draw_line_ex(Vector2::new(rect.x, y0), Vector2::new(rect.x + rect.width, y0), 2.0, self.axis_color);
        d.draw_line_ex(Vector2::new(x0, rect.y), Vector2::new(x0, rect.y + rect.height), 2.0, self.axis_color);

        let mut last_x = 0.0;
        let mut last_y = 0.0;
        for i in 0..n_samples {
            let x = self.x_left + i as f64 * x_step;
            eval.set_var("x", x);
            let y = eval.eval(expr);

            let px = rect.x + (x - self.x_left) as f32 * dx;
            let py = rect.y + (y - self.y_bottom) as f32 * dy;
            //println!("{},{}", px, py);
            if i > 0 {
                d.draw_line_ex(Vector2::new(last_x, last_y), Vector2::new(px, py), 2.0, self.graph_color);
            }
            last_x = px;
            last_y = py;
        }
    }
}
