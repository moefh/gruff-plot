mod widgets;
mod expr;

use raylib::prelude::*;

static FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/ComicMono.ttf");

const FONT_SIZE: i32 = 24;
const GRAPH_WIDTH: u32 = 1200;
const GRAPH_HEIGHT: u32 = 800;
const WINDOW_MARGIN: f32 = 12.0;

use widgets::{
    WidgetBag,
    TextBoxWidget,
    PlotWidget,
};

fn resize_widgets(widgets: &mut WidgetBag, window_width: i32, window_height: i32) {
    if widgets.width == window_width && widgets.height == window_height {
        return;
    }
    widgets.width = window_width;
    widgets.height = window_height;

    let window_width = window_width as f32;
    let window_height = window_height as f32;

    let mut x = WINDOW_MARGIN;
    let mut y = 0.0;

    // function text box
    if let Some(func) = widgets.get_text_box_mut(0) {
        func.rect.width = (0.5 * window_width).floor() - 2.0 * WINDOW_MARGIN;
        x = func.rect.x + func.rect.width;
        y = func.rect.y + func.rect.height;
    }

    // bounds check boxes
    for index in 1..5 {
        if let Some(text) = widgets.get_text_box_mut(index) {
            text.rect.x = x + WINDOW_MARGIN;
            x = text.rect.x + text.rect.width;
        }
    }

    // plot
    if let Some(plot) = widgets.get_plot_mut(5) {
        plot.rect.width = (window_width - 2.0 * WINDOW_MARGIN).min(GRAPH_WIDTH as f32);
        plot.rect.height = (window_height - y - 2.0 * WINDOW_MARGIN).min(GRAPH_HEIGHT as f32);
        plot.rect.x = (0.5 * (window_width - plot.rect.width)).floor();
        plot.rect.y = y + (0.5 * (window_height - plot.rect.height - y)).floor();
    }
}

pub fn update_graph_bounds(widgets: &mut WidgetBag) {
    let min_x = widgets.get_text_box(1).and_then(|text| text.get_text().parse::<f64>().ok());
    let min_y = widgets.get_text_box(2).and_then(|text| text.get_text().parse::<f64>().ok());
    let max_x = widgets.get_text_box(3).and_then(|text| text.get_text().parse::<f64>().ok());
    let max_y = widgets.get_text_box(4).and_then(|text| text.get_text().parse::<f64>().ok());

    let bounds = if let Some(plot) = widgets.get_plot_mut(5) {
        if let Some(min_x) = min_x && plot.x_left != min_x { plot.x_left = min_x; }
        if let Some(min_y) = min_y && plot.y_bottom != min_y { plot.y_bottom = min_y; }
        if let Some(max_x) = max_x && plot.x_right != max_x { plot.x_right = max_x; }
        if let Some(max_y) = max_y && plot.y_top != max_y { plot.y_top = max_y; }
        Some((plot.x_left, plot.y_bottom, plot.x_right, plot.y_top))
    } else {
        None
    };

    let focus = widgets.focus;
    if let Some((min_x, min_y, max_x, max_y)) = bounds {
        if focus != 1 && let Some(text) = widgets.get_text_box_mut(1) { text.set_text(format!("{}", min_x)); text.changed = false; }
        if focus != 2 && let Some(text) = widgets.get_text_box_mut(2) { text.set_text(format!("{}", min_y)); text.changed = false; }
        if focus != 3 && let Some(text) = widgets.get_text_box_mut(3) { text.set_text(format!("{}", max_x)); text.changed = false; }
        if focus != 4 && let Some(text) = widgets.get_text_box_mut(4) { text.set_text(format!("{}", max_y)); text.changed = false; }
    }
}

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
    rl.set_window_min_size(900, 600);

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

    let text_box_height = FONT_SIZE as f32 + widgets::TextBoxWidget::PAD_VERTICAL;
    let mut widgets = WidgetBag::new();
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(WINDOW_MARGIN, WINDOW_MARGIN, 300.0, text_box_height)).with_text("sin(x)"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, 100.0, text_box_height)).with_text("-10.0"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, 100.0, text_box_height)).with_text("-10.0"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, 100.0, text_box_height)).with_text("10.0"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, 100.0, text_box_height)).with_text("10.0"));
    widgets.add_plot(PlotWidget::new(Rectangle::new(0.0, text_box_height + 2.0 * WINDOW_MARGIN, 300.0, 300.0)));

    while ! rl.window_should_close() {
        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        resize_widgets(&mut widgets, window_width, window_height);
        if d.is_key_pressed(KeyboardKey::KEY_TAB) {
            let direction = if d.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) || d.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT) { -1 } else { 1 };
            widgets.advance_focus(direction);
        }

        // draw
        let focus = widgets.focus;
        if let Some(func) = widgets.get_text_box_mut(0) {
            func.draw(&mut d, &font, 24.0, focus == 0, if expr.is_some() { None } else { Some(Color::RED) });
            if func.changed {
                expr = expr::Expr::parse(func.get_text()).ok();
            }
        }
        for i in 1..5 {
            if let Some(text) = widgets.get_text_box_mut(i) {
                text.draw(&mut d, &font, 24.0, focus == i, None);
            }
        }
        if let Some(plot) = widgets.get_plot_mut(5) {
            let rect = Rectangle::new(0.0, 0.0, plot.rect.width, plot.rect.height);
            d.draw_texture_mode(&thread, &mut graph_tex, |mut tex| {
                tex.clear_background(Color::WHITE);
                plot.draw(&mut tex, rect, expr.as_ref(), &mut eval);
            });
            d.draw_texture_rec(
                graph_tex.texture(),
                Rectangle::new(0.0, GRAPH_HEIGHT as f32 - rect.height, rect.width, rect.height),
                Vector2::new(plot.rect.x, plot.rect.y),
                Color::WHITE
            );
        }
        update_graph_bounds(&mut widgets);

        // quit
        let control_held = d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || d.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        if d.is_key_pressed(KeyboardKey::KEY_Q) && control_held {
            break;
        }
    }
}
