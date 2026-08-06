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

    fn draw_widget(&mut self, d: &mut RaylibDrawHandle<'_>, expr: Option<&Expr>, eval: &mut ExprEvaluator) {
        let expr = if let Some(expr) = expr {
            expr
        } else {
            d.draw_rectangle_rec(self.rect, Color::new(224, 224, 224, 255));
            d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);
            return;
        };

        d.draw_rectangle_rec(self.rect, Color::WHITE);
        d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);
        let n_samples = 2 * self.rect.width.max(0.0).floor() as u32;
        let x_step = (self.max_x - self.min_x) / n_samples as f64;

        let scale_x = self.rect.width / (self.max_x - self.min_x) as f32;
        let scale_y = -self.rect.height / (self.max_y - self.min_y) as f32;
        let start_x = self.rect.x;
        let start_y = self.rect.y + self.rect.height;

        let x0 = start_x - self.min_x as f32 * scale_x;
        let y0 = start_y - self.min_y as f32 * scale_y;
        d.draw_line_ex(Vector2::new(self.rect.x, y0), Vector2::new(self.rect.x + self.rect.width, y0), 2.0, self.axis_color);
        d.draw_line_ex(Vector2::new(x0, self.rect.y), Vector2::new(x0, self.rect.y + self.rect.height), 2.0, self.axis_color);

        let mut last_x = 0.0;
        let mut last_y = 0.0;
        for i in 0..n_samples {
            let x = self.min_x + i as f64 * x_step;
            eval.set_var("x", x);
            let y = eval.eval(expr);

            let px = start_x + (x - self.min_x) as f32 * scale_x;
            let py = start_y + (y - self.min_y) as f32 * scale_y;
            if i > 0 && (py - last_y).abs() < self.rect.height {
                d.draw_line_ex(Vector2::new(last_x, last_y), Vector2::new(px, py), 2.0, self.graph_color);
            }
            last_x = px;
            last_y = py;
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, expr: Option<&Expr>, eval: &mut ExprEvaluator) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() < 0.0 { return; }

        d.draw_scissor_mode(
            self.rect.x.floor() as i32,
            self.rect.y.floor() as i32,
            self.rect.width.floor() as i32,
            self.rect.height.floor() as i32,
            |mut d| {
                self.draw_widget(&mut d, expr, eval);
            }
        );
    }
}
