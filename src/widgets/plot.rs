use raylib::prelude::*;

use crate::expr::Expr;
use crate::expr::eval::ExprEvaluator;

pub struct GruffPlotWidget {
    pub rect: Rectangle,
    pub graph_color: Color,
    pub axis_color: Color,
    pub x_left: f64,
    pub x_right: f64,
    pub y_top: f64,
    pub y_bottom: f64,
    pub flip_y: bool,
}

impl GruffPlotWidget {
    pub const BORDER: f32 = 2.0;

    pub fn new(rect: Rectangle, flip_y: bool) -> Self {
        GruffPlotWidget {
            rect,
            graph_color: Color::BLACK,
            axis_color: Color::BLUE,
            x_left: -6.0,
            x_right: 6.0,
            y_top: -4.0,
            y_bottom: 4.0,
            flip_y,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, expr: Option<&Expr>, eval: &mut ExprEvaluator) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() < 0.0 { return; }

        d.clear_background(Color::RED);

        let expr = if let Some(expr) = expr {
            expr
        } else {
            d.draw_rectangle_rec(self.rect, Color::new(224, 224, 224, 255));
            d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);
            return;
        };

        d.draw_rectangle_rec(self.rect, Color::WHITE);
        d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);
        let n_samples = self.rect.width.max(0.0).floor() as u32;
        let x_step = (self.x_right - self.x_left) / n_samples as f64;

        let dx = self.rect.width / (self.x_right - self.x_left) as f32;
        let dy = self.rect.height / (self.y_bottom - self.y_top) as f32;

        let x0 = self.rect.x + (0.0 - self.x_left) as f32 * dx;
        let y0 = if self.flip_y {
            self.rect.y - (0.0 - self.y_bottom) as f32 * dy
        } else {
            self.rect.y + (0.0 - self.y_top) as f32 * dy
        };
        d.draw_line_ex(Vector2::new(self.rect.x, y0), Vector2::new(self.rect.x + self.rect.width, y0), 2.0, self.axis_color);
        d.draw_line_ex(Vector2::new(x0, self.rect.y), Vector2::new(x0, self.rect.y + self.rect.height), 2.0, self.axis_color);

        let mut last_x = 0.0;
        let mut last_y = 0.0;
        for i in 0..n_samples {
            let x = self.x_left + i as f64 * x_step;
            eval.set_var("x", x);
            let y = eval.eval(expr);

            let px = self.rect.x + (x - self.x_left) as f32 * dx;
            let py = if self.flip_y {
                self.rect.y - (y - self.y_bottom) as f32 * dy
            } else {
                self.rect.y + (y - self.y_top) as f32 * dy
            };
            //println!("{},{}", px, py);
            if i > 0 {
                d.draw_line_ex(Vector2::new(last_x, last_y), Vector2::new(px, py), 2.0, self.graph_color);
            }
            last_x = px;
            last_y = py;
        }
    }
}
