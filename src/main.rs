mod widgets;
mod expr;

use raylib::prelude::*;

static FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/ComicMono.ttf");

const FONT_SIZE: i32 = 24;
const GRAPH_WIDTH: u32 = 1200;
const GRAPH_HEIGHT: u32 = 800;
const WINDOW_MARGIN: f32 = 12.0;

use widgets::{
    GruffTextBoxWidget,
    GruffPlotWidget,
};

pub fn main() {
    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_ERROR)
        .resizable()
        .size(1200, 800)
        .title("Gruff Plot")
        .vsync()
        .build();
    rl.set_target_fps(60);
    rl.set_exit_key(None);
    rl.set_window_min_size(400, 300);

    let mut graph_tex = match rl.load_render_texture(&thread, GRAPH_WIDTH, GRAPH_HEIGHT) {
        Ok(tex) => { tex }
        Err(e) => { println!("ERROR creating graph texture: {}", e); return; }
    };
    let font = match rl.load_font_from_memory(&thread, ".ttf", FONT_BYTES, FONT_SIZE, None) {
        Ok(font) => { font }
        Err(e) => { println!("ERROR loading font: {}", e); return; }
    };

    let mut eval = expr::eval::ExprEvaluator::new().with_math_funcs().with_math_consts();
    let mut expr = None;

    let mut text_box = GruffTextBoxWidget::new(Rectangle::new(
        WINDOW_MARGIN,
        WINDOW_MARGIN,
        300.0,
        FONT_SIZE as f32 + widgets::GruffTextBoxWidget::PAD_VERTICAL
    )).with_text("sin(x)");
    let mut plot = GruffPlotWidget::new(Rectangle::new(0.0, 0.0, 300.0, 300.0), false);

    while ! rl.window_should_close() {
        let window_width = rl.get_screen_width() as f32;
        let window_height = rl.get_screen_height() as f32;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // resize
        text_box.rect.width = window_width - 2.0 * WINDOW_MARGIN;
        plot.rect.width = (window_width - 2.0 * WINDOW_MARGIN).min(GRAPH_WIDTH as f32);
        plot.rect.height = (window_height - text_box.rect.height - 3.0 * WINDOW_MARGIN).min(GRAPH_HEIGHT as f32);
        let plot_x = 0.5 * (window_width - plot.rect.width);
        let plot_y = WINDOW_MARGIN + text_box.rect.height + 0.5 * (window_height - plot.rect.height - WINDOW_MARGIN - text_box.rect.height);

        // draw
        text_box.draw(&mut d, &font, 24.0, true, if expr.is_some() { None } else { Some(Color::RED) });
        if text_box.changed {
            expr = expr::Expr::parse(text_box.get_text()).ok();
            text_box.changed = false;
        }
        d.draw_texture_mode(&thread, &mut graph_tex, |mut tex| {
            tex.clear_background(Color::WHITE);
            plot.draw(&mut tex, expr.as_ref(), &mut eval);
        });
        d.draw_texture_rec(
            graph_tex.texture(),
            Rectangle::new(0.0, GRAPH_HEIGHT as f32 - plot.rect.height, plot.rect.width, plot.rect.height),
            Vector2::new(plot_x, plot_y),
            Color::WHITE
        );

        // quit
        let control_held = d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || d.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        if d.is_key_pressed(KeyboardKey::KEY_Q) && control_held {
            break;
        }
    }
}
