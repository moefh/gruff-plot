use raylib::prelude::*;

use super::{
    MouseHandler,
    MouseAction,
    ZoomAxis,
};

pub struct ZoomAxisWidget {
    pub rect: Rectangle,
    pub zoom_axis: ZoomAxis,
    mouse: MouseHandler,
}

impl ZoomAxisWidget {
    const BORDER: f32 = 2.0;

    pub fn new(rect: Rectangle) -> Self {
        ZoomAxisWidget {
            rect,
            zoom_axis: ZoomAxis::Both,
            mouse: MouseHandler::new(),
        }
    }

    pub fn want_focus(&self) -> bool {
        false
    }

    pub fn handle_mouse(&mut self, rl: &mut RaylibDrawHandle<'_>) {
        if ! self.mouse.is_inside(rl, self.rect) {
            return;
        }

        match self.mouse.update(rl) {
            MouseAction::Up(button) if button == MouseButton::MOUSE_BUTTON_LEFT => {
                self.zoom_axis = match self.zoom_axis {
                    ZoomAxis::Both => { ZoomAxis::X }
                    ZoomAxis::X => { ZoomAxis::Y }
                    ZoomAxis::Y => { ZoomAxis::Both }
                };
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
        if self.rect.width.floor() <= 0.0 || self.rect.height.floor() <= 0.0 { return; }

        self.handle_mouse(d);

        d.draw_rectangle_rec(self.rect, Color::WHITE);//Color::new(255, 255, 192, 255));
        d.draw_rectangle_lines_ex(self.rect, Self::BORDER, Color::BLACK);

        let cx = self.rect.x + (0.5 * self.rect.width).floor();
        let cy = self.rect.y + (0.5 * self.rect.height).floor();
        let sx = self.rect.x + 5.0;
        let sy = self.rect.y + 5.0;
        let ex = self.rect.x + self.rect.width - 5.0;
        let ey = self.rect.y + self.rect.height - 5.0;

        match self.zoom_axis {
            ZoomAxis::Both => {
                d.draw_line_ex(Vector2::new(sx, cy), Vector2::new(ex, cy), 2.0, Color::BLUE);
                d.draw_line_ex(Vector2::new(cx, sy), Vector2::new(cx, ey), 2.0, Color::BLUE);
            }
            ZoomAxis::X => {
                d.draw_line_ex(Vector2::new(sx, cy), Vector2::new(ex, cy), 2.0, Color::BLUE);
            }
            ZoomAxis::Y => {
                d.draw_line_ex(Vector2::new(cx, sy), Vector2::new(cx, ey), 2.0, Color::BLUE);
            }
        }
    }
}
