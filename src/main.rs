mod widgets;
mod expr;
mod data;

use std::path::PathBuf;
use raylib::prelude::*;

static FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/ComicMono.ttf");

const FONT_SIZE: i32 = 24;
const GRAPH_WIDTH: u32 = 1200;
const GRAPH_HEIGHT: u32 = 800;
const BOUND_WIDTH: f32 = 100.0;
const WINDOW_MARGIN: f32 = 12.0;

const WIDGET_PLOT: usize = 0;
const WIDGET_ZOOM_AXIS: usize = 1;
const WIDGET_MIN_X: usize = 2;
const WIDGET_MIN_Y: usize = 3;
const WIDGET_MAX_X: usize = 4;
const WIDGET_MAX_Y: usize = 5;
const WIDGET_FUNC: usize = 6;

use widgets::{
    ZoomAxis,
    WidgetBag,
    TextBoxWidget,
    PlotWidget,
    ZoomAxisWidget,
};

fn resize_widgets(widgets: &mut WidgetBag, window_width: i32, window_height: i32) {
    if widgets.width == window_width && widgets.height == window_height {
        return;
    }
    widgets.width = window_width;
    widgets.height = window_height;

    let window_width = window_width as f32;
    let window_height = window_height as f32;

    // zoom axis
    let (zoom_axis_width, zoom_axis_height) = if let Some(zoom_axis) = widgets.get_zoom_axis_mut(WIDGET_ZOOM_AXIS) {
        zoom_axis.rect.x = window_width - WINDOW_MARGIN - zoom_axis.rect.width;
        (zoom_axis.rect.width, zoom_axis.rect.height)
    } else {
        (32.0, 32.0)
    };

    // plot
    if let Some(plot) = widgets.get_plot_mut(WIDGET_PLOT) {
        let y = zoom_axis_height + WINDOW_MARGIN;
        plot.rect.width = (window_width - 2.0 * WINDOW_MARGIN).min(GRAPH_WIDTH as f32);
        plot.rect.height = (window_height - y - 2.0 * WINDOW_MARGIN).min(GRAPH_HEIGHT as f32);
        plot.rect.x = (0.5 * (window_width - plot.rect.width)).floor();
        plot.rect.y = y + (0.5 * (window_height - plot.rect.height - y)).floor();
    }

    // function text box
    if let Some(func) = widgets.get_text_box_mut(WIDGET_FUNC) {
        func.rect.width = window_width - 4.0 * (WINDOW_MARGIN + BOUND_WIDTH) - 3.0 * WINDOW_MARGIN - func.rect.height;
        func.rect.height
    } else {
        0.0
    };

    // bounds text boxes
    for (index, widget_index) in [WIDGET_MIN_X, WIDGET_MIN_Y, WIDGET_MAX_X, WIDGET_MAX_Y].iter().enumerate() {
        if let Some(text) = widgets.get_text_box_mut(*widget_index) {
            text.rect.x = window_width - (4 - index) as f32 * (WINDOW_MARGIN + BOUND_WIDTH) - WINDOW_MARGIN - zoom_axis_width;
        }
    }
}

pub fn update_graph_bounds(widgets: &mut WidgetBag) {
    let bounds = if let Some(plot) = widgets.get_plot_mut(WIDGET_PLOT) && plot.bounds_changed {
        plot.bounds_changed = false;
        Some((plot.min_x, plot.min_y, plot.max_x, plot.max_y, true))
    } else {
        let min_x = widgets.get_text_box(WIDGET_MIN_X).and_then(|text| text.get_text().parse::<f64>().ok());
        let min_y = widgets.get_text_box(WIDGET_MIN_Y).and_then(|text| text.get_text().parse::<f64>().ok());
        let max_x = widgets.get_text_box(WIDGET_MAX_X).and_then(|text| text.get_text().parse::<f64>().ok());
        let max_y = widgets.get_text_box(WIDGET_MAX_Y).and_then(|text| text.get_text().parse::<f64>().ok());

        if let Some(plot) = widgets.get_plot_mut(WIDGET_PLOT) {
            let mut changed = false;
            if let Some(min_x) = min_x && plot.min_x != min_x { plot.min_x = min_x; changed = true; }
            if let Some(min_y) = min_y && plot.min_y != min_y { plot.min_y = min_y; changed = true; }
            if let Some(max_x) = max_x && plot.max_x != max_x { plot.max_x = max_x; changed = true; }
            if let Some(max_y) = max_y && plot.max_y != max_y { plot.max_y = max_y; changed = true; }
            if changed {
                Some((plot.min_x, plot.min_y, plot.max_x, plot.max_y, false))
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some((min_x, min_y, max_x, max_y, force_all)) = bounds {
        let focus = if force_all { widgets.widgets.len() } else { widgets.focus };
        if focus != 1 && let Some(text) = widgets.get_text_box_mut(WIDGET_MIN_X) { text.set_text(format!("{:.3}", min_x)); }
        if focus != 2 && let Some(text) = widgets.get_text_box_mut(WIDGET_MIN_Y) { text.set_text(format!("{:.3}", min_y)); }
        if focus != 3 && let Some(text) = widgets.get_text_box_mut(WIDGET_MAX_X) { text.set_text(format!("{:.3}", max_x)); }
        if focus != 4 && let Some(text) = widgets.get_text_box_mut(WIDGET_MAX_Y) { text.set_text(format!("{:.3}", max_y)); }
    }
}

fn read_cmdline_options() -> (Vec<data::DataSeries>, bool) {
    let mut data_series = Vec::new();
    let mut show_expr = false;
    for opt in std::env::args_os().skip(1) {
        if opt == "-h" {
            println!("USAGE: gruff-plot [options] [filename...]");
            println!();
            println!("options:");
            println!("  -f      show function expression (default: only with no data files)");
            std::process::exit(0);
        }
        if opt == "-f" {
            show_expr = true;
            continue;
        }

        let path = PathBuf::from(&opt);

        // text file
        if path.extension() == Some(&std::ffi::OsString::from("txt")) {
            let data = data::read_text_file(&path, Some((0.0, 1.0))).unwrap_or_else(|e| {
                println!("ERROR loading text file '{}': {}", path.display(), e);
                std::process::exit(1);
            });
            data_series.push(data);
            continue;
        }

        // wav file
        if path.extension() == Some(&std::ffi::OsString::from("wav")) {
            let wav = data::read_wav_file(&path).unwrap_or_else(|e| {
                println!("ERROR loading WAV file '{}': {}", path.display(), e);
                std::process::exit(1);
            });
            let dx = 1.0 / wav.sample_rate as f64;
            for chan in wav.channels.iter() {
                let data = chan.iter().enumerate().map(|(index, sample)| {
                    data::DataItem::new(dx * index as f64, *sample as f64 / i16::MAX as f64)
                }).collect::<Vec<data::DataItem>>();
                data_series.push(data::DataSeries::new(data));
            }
            continue;
        }

        println!("ERROR: unknown option or file type: '{}'", path.display());
        std::process::exit(1);
    }
    if data_series.is_empty() {
        show_expr = true;
    }
    (data_series, show_expr)
}

pub fn main() {
    let (data_series, show_expr) = read_cmdline_options();

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

    let font = match rl.load_font_from_memory(&thread, ".ttf", FONT_BYTES, FONT_SIZE, None) {
        Ok(font) => { font }
        Err(e) => { println!("ERROR loading font: {}", e); return; }
    };

    let mut eval = expr::eval::ExprEvaluator::new().with_math_funcs().with_math_consts();
    let mut expr = None;
    let mut invalid_expr = false;

    let text_height = FONT_SIZE as f32 + widgets::TextBoxWidget::PAD_VERTICAL;
    let mut widgets = WidgetBag::new();
    widgets.add_plot(PlotWidget::new(Rectangle::new(0.0, text_height + 2.0 * WINDOW_MARGIN, 0.0, 0.0)).with_data(data_series));
    widgets.add_zoom_axis(ZoomAxisWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, text_height, text_height)));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, BOUND_WIDTH, text_height)).with_text("-5.0"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, BOUND_WIDTH, text_height)).with_text("-3.0"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, BOUND_WIDTH, text_height)).with_text("5.0"));
    widgets.add_text_box(TextBoxWidget::new(Rectangle::new(0.0, WINDOW_MARGIN, BOUND_WIDTH, text_height)).with_text("3.0"));
    if show_expr {
        widgets.add_text_box(TextBoxWidget::new(Rectangle::new(WINDOW_MARGIN, WINDOW_MARGIN, 0.0, text_height)).with_text("sin(x)"));
    }

    if let Some(func) = widgets.get_text_box_mut(WIDGET_FUNC) {
        func.changed = true;
    }

    while ! rl.window_should_close() {
        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // update widgets
        resize_widgets(&mut widgets, window_width, window_height);
        widgets.handle_keyboard(&d);
        widgets.handle_mouse(&d);

        // draw widgets
        let focus = widgets.focus;
        if let Some(func) = widgets.get_text_box_mut(WIDGET_FUNC) {
            func.draw(&mut d, &font, FONT_SIZE as f32, focus == 0, if invalid_expr { Some(Color::RED) } else { None });
            if func.changed {
                if let Ok(new_expr) = expr::Expr::parse(func.get_text()) {
                    expr = Some(new_expr);
                    invalid_expr = false;
                } else {
                    invalid_expr = true;
                }
            }
        }
        for index in [WIDGET_MIN_X, WIDGET_MIN_Y, WIDGET_MAX_X, WIDGET_MAX_Y] {
            if let Some(text) = widgets.get_text_box_mut(index) {
                text.draw(&mut d, &font, FONT_SIZE as f32, focus == index, None);
            }
        }
        let zoom_axis = if let Some(zoom_axis) = widgets.get_zoom_axis_mut(WIDGET_ZOOM_AXIS) {
            zoom_axis.draw(&mut d);
            zoom_axis.zoom_axis
        } else {
            ZoomAxis::Both
        };
        update_graph_bounds(&mut widgets);
        if let Some(plot) = widgets.get_plot_mut(WIDGET_PLOT) {
            plot.draw(&mut d, zoom_axis, expr.as_ref(), &mut eval);
        }

        // quit
        let control_held = d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || d.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        if d.is_key_pressed(KeyboardKey::KEY_Q) && control_held {
            break;
        }
    }
}
