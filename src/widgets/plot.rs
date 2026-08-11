use raylib::prelude::*;

use crate::expr::Expr;
use crate::expr::eval::ExprEvaluator;
use crate::data;

use super::{
    MouseHandler,
    MouseAction,
};

#[derive(Copy, Clone, PartialEq)]
pub enum ZoomAxis {
    Both,
    X,
    Y,
}

impl ZoomAxis {
    pub fn has_x(self) -> bool { self == ZoomAxis::Both || self == ZoomAxis::X }
    pub fn has_y(self) -> bool { self == ZoomAxis::Both || self == ZoomAxis::Y }
    pub fn next(self) -> Self {
        match self {
            ZoomAxis::Both => { ZoomAxis::X }
            ZoomAxis::X => { ZoomAxis::Y }
            ZoomAxis::Y => { ZoomAxis::Both }
        }
    }
}

struct CoordTransform {
    scale_x: f32,
    scale_y: f32,
    start_x: f32,
    start_y: f32,
    min_x: f64,
    min_y: f64,
}

impl CoordTransform {
    fn new(rect: Rectangle, min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        CoordTransform {
            scale_x: rect.width / (max_x - min_x) as f32,
            scale_y: -rect.height / (max_y - min_y) as f32,
            start_x: rect.x,
            start_y: rect.y + rect.height,
            min_x,
            min_y,
        }
    }

    fn graph_to_window(&self, x: f64, y: f64) -> (f32, f32) {
        (
            self.start_x + (x - self.min_x) as f32 * self.scale_x,
            self.start_y + (y - self.min_y) as f32 * self.scale_y
        )
    }

    fn window_to_graph(&self, x: f32, y: f32) -> (f64, f64) {
        (
            self.min_x + ((x - self.start_x) / self.scale_x) as f64,
            self.min_y + ((y - self.start_y) / self.scale_y) as f64
        )
    }
}

pub struct PlotWidget {
    pub rect: Rectangle,
    pub axis_color: Color,
    pub zoom_axis: ZoomAxis,
    pub hover_x: f64,
    pub hover_y: f64,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    mouse: MouseHandler,
}

impl PlotWidget {
    pub const BORDER: f32 = 2.0;

    pub fn new(rect: Rectangle) -> Self {
        PlotWidget {
            rect,
            axis_color: Color::BLACK,
            zoom_axis: ZoomAxis::Both,
            hover_x: 0.0,
            hover_y: 0.0,
            min_x: -1.0,
            min_y: -1.0,
            max_x: 1.0,
            max_y: 1.0,
            mouse: MouseHandler::new(),
        }
    }

    fn get_transform(&self) -> CoordTransform {
        CoordTransform::new(self.rect, self.min_x, self.min_y, self.max_x, self.max_y)
    }

    pub fn want_focus(&self) -> bool {
        false
    }

    pub fn mouse_cursor(&self) -> Option<MouseCursor> {
        Some(MouseCursor::MOUSE_CURSOR_CROSSHAIR)
    }

    pub fn handle_mouse(&mut self, rl: &mut RaylibDrawHandle<'_>) -> bool {
        if ! self.mouse.is_inside(rl, self.rect) {
            return false;
        }

        let mouse_pos = rl.get_mouse_position();

        match self.mouse.update(rl) {
            MouseAction::Drag(MouseButton::MOUSE_BUTTON_LEFT, delta) => {
                let tr = self.get_transform();
                self.min_x += (delta.x / tr.scale_x) as f64;
                self.min_y += (delta.y / tr.scale_y) as f64;
                self.max_x += (delta.x / tr.scale_x) as f64;
                self.max_y += (delta.y / tr.scale_y) as f64;

                (self.hover_x, self.hover_y) = self.get_transform().window_to_graph(mouse_pos.x, mouse_pos.y);
                true
            }
            MouseAction::Wheel(delta) => {
                let zoom = if delta < 0.0 { 1.1 } else { 1.0/1.1 };
                let cur_width = self.max_x - self.min_x;
                let cur_height = self.max_y - self.min_y;
                let new_width = cur_width * zoom;
                let new_height = cur_height * zoom;

                let tr = self.get_transform();
                let (graph_x, graph_y) = tr.window_to_graph(mouse_pos.x, mouse_pos.y);
                let delta_x = (graph_x - self.min_x) / cur_width;
                let delta_y = (graph_y - self.min_y) / cur_height;
                if self.zoom_axis.has_x() {
                    self.min_x = graph_x - delta_x * new_width;
                    self.max_x = graph_x + (1.0 - delta_x) * new_width;
                }
                if self.zoom_axis.has_y() {
                    self.min_y = graph_y - delta_y * new_height;
                    self.max_y = graph_y + (1.0 - delta_y) * new_height;
                }

                (self.hover_x, self.hover_y) = self.get_transform().window_to_graph(mouse_pos.x, mouse_pos.y);
                true
            }
            _ => {
                (self.hover_x, self.hover_y) = self.get_transform().window_to_graph(mouse_pos.x, mouse_pos.y);
                false
            }
        }
    }

    fn draw_widget(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle_rec(self.rect, Color::new(240, 240, 255, 255));
        d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);

        let tr = self.get_transform();
        let (x0, y0) = tr.graph_to_window(0.0, 0.0);
        d.draw_line_ex(Vector2::new(self.rect.x, y0), Vector2::new(self.rect.x + self.rect.width, y0), 2.0, self.axis_color);
        d.draw_line_ex(Vector2::new(x0, self.rect.y), Vector2::new(x0, self.rect.y + self.rect.height), 2.0, self.axis_color);
    }

    pub fn draw_series(&self, d: &mut RaylibDrawHandle<'_>, items: &[data::DataItem], color: Color) {
        if items.is_empty() { return; }
        d.draw_scissor_mode(
            self.rect.x.floor() as i32,
            self.rect.y.floor() as i32,
            self.rect.width.floor() as i32,
            self.rect.height.floor() as i32,
            |mut d| {
                let tr = self.get_transform();
                let win_x_min = self.rect.x;
                let win_x_max = self.rect.x + self.rect.width;
                let (mut last_x, mut last_y) = tr.graph_to_window(items[0].x, items[0].y);
                for item in &items[1..] {
                    let (px, py) = tr.graph_to_window(item.x, item.y);
                    if (py - last_y).abs() < self.rect.height {
                        if (last_x < win_x_min && px < win_x_min) || (last_x > win_x_max && px > win_x_max) { continue; }
                        d.draw_line_ex(Vector2::new(last_x, last_y), Vector2::new(px, py), 2.0, color);
                    }
                    last_x = px;
                    last_y = py;
                }
            }
        );
    }

    pub fn draw_expr(&self, d: &mut RaylibDrawHandle<'_>, expr: &Expr, eval: &mut ExprEvaluator, color: Color) {
        d.draw_scissor_mode(
            self.rect.x.floor() as i32,
            self.rect.y.floor() as i32,
            self.rect.width.floor() as i32,
            self.rect.height.floor() as i32,
            |mut d| {
                let tr = self.get_transform();
                let n_samples = 2 * self.rect.width.max(0.0).floor() as u32;
                let x_step = (self.max_x - self.min_x) / n_samples as f64;
                let mut last_x = 0.0;
                let mut last_y = 0.0;
                for i in 0..n_samples {
                    let x = self.min_x + i as f64 * x_step;
                    eval.set_var("x", x);
                    let y = eval.eval(expr);

                    let (px, py) = tr.graph_to_window(x, y);
                    if i > 0 && (py - last_y).abs() < self.rect.height {
                        d.draw_line_ex(Vector2::new(last_x, last_y), Vector2::new(px, py), 2.0, color);
                    }
                    last_x = px;
                    last_y = py;
                }
            }
        );
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() <= 0.0 { return; }
        self.draw_widget(d);
    }
}
