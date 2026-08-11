use raylib::prelude::*;

use super::{
    expr,
    data,
    GraphSource,
    GraphSourceData,
    GraphSourceKind,
};
use super::widgets::{
    self,
    ZoomAxis,
    WidgetBag,
    TextBoxWidget,
    PlotWidget,
    ButtonWidget,
};

#[derive(Debug)]
struct GraphSourceWidgets {
    kind_widget: usize,
    text_widget: usize,
    //color_widget: usize,
    invalid: bool,
    source: GraphSource,
}

impl GraphSourceWidgets {
    pub fn new(kind_widget: usize, text_widget: usize, source: GraphSource) -> Self {
        GraphSourceWidgets {
            kind_widget,
            text_widget,
            source,
            invalid: false,
        }
    }
}

fn draw_zoom_axis_button(d: &mut RaylibDrawHandle<'_>, zoom_axis: ZoomAxis, rect: Rectangle) {
    let cx = rect.x + (0.5 * rect.width).floor();
    let cy = rect.y + (0.5 * rect.height).floor();
    let sx = rect.x + 5.0;
    let sy = rect.y + 5.0;
    let ex = rect.x + rect.width - 5.0;
    let ey = rect.y + rect.height - 5.0;

    if zoom_axis == ZoomAxis::Both || zoom_axis == ZoomAxis::X {
        d.draw_line_ex(Vector2::new(sx, cy), Vector2::new(ex, cy), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(sx, cy), Vector2::new(sx + 5.0, cy - 5.0), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(sx, cy), Vector2::new(sx + 5.0, cy + 5.0), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(ex, cy), Vector2::new(ex - 5.0, cy - 5.0), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(ex, cy), Vector2::new(ex - 5.0, cy + 5.0), 2.0, Color::BLUE);
    }
    if zoom_axis == ZoomAxis::Both || zoom_axis == ZoomAxis::Y {
        d.draw_line_ex(Vector2::new(cx, sy), Vector2::new(cx, ey), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(cx, sy), Vector2::new(cx - 5.0, sy + 5.0), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(cx, sy), Vector2::new(cx + 5.0, sy + 5.0), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(cx, ey), Vector2::new(cx - 5.0, ey - 5.0), 2.0, Color::BLUE);
        d.draw_line_ex(Vector2::new(cx, ey), Vector2::new(cx + 5.0, ey - 5.0), 2.0, Color::BLUE);
    }
}

pub struct Window {
    width: i32,
    height: i32,
    font_size: f32,
    text_height: f32,
    eval: expr::eval::ExprEvaluator,
    widgets: WidgetBag,

    add_btn_widget: usize,
    rm_btn_widget: usize,
    min_x_widget: usize,
    min_y_widget: usize,
    max_x_widget: usize,
    max_y_widget: usize,
    zoom_axis_widget: usize,
    plot_widget: usize,
    source_widgets: Vec<GraphSourceWidgets>,
}

impl Window {
    const BOUND_WIDTH: f32 = 100.0;
    const BUTTON_WIDTH: f32 = 100.0;
    const KIND_WIDTH: f32 = 60.0;
    const MARGIN: f32 = 12.0;
    const GRAPH_WIDTH: u32 = 1200;
    const GRAPH_HEIGHT: u32 = 800;

    pub fn new(font_size: f32, graph_sources: Vec<GraphSource>) -> Self {
        let text_height = font_size + widgets::TextBoxWidget::PAD_VERTICAL;
        let mut widgets = WidgetBag::new();

        let add_btn_widget = widgets.add_button(
            ButtonWidget::new(Rectangle::new(0.0, 0.0, Self::BUTTON_WIDTH, text_height)).with_text("Add")
        );
        let rm_btn_widget = widgets.add_button(
            ButtonWidget::new(Rectangle::new(0.0, 0.0, Self::BUTTON_WIDTH, text_height)).with_text("Remove")
        );

        let min_x_widget = widgets.add_text_box(
            TextBoxWidget::new(Rectangle::new(0.0, 0.0, Self::BOUND_WIDTH, text_height)).with_text("-5.0")
        );
        let min_y_widget = widgets.add_text_box(
            TextBoxWidget::new(Rectangle::new(0.0, 0.0, Self::BOUND_WIDTH, text_height)).with_text("-3.0")
        );
        let max_x_widget = widgets.add_text_box(
            TextBoxWidget::new(Rectangle::new(0.0, 0.0, Self::BOUND_WIDTH, text_height)).with_text("5.0")
        );
        let max_y_widget = widgets.add_text_box(
            TextBoxWidget::new(Rectangle::new(0.0, 0.0, Self::BOUND_WIDTH, text_height)).with_text("3.0")
        );
        let zoom_axis_widget = widgets.add_button(
            ButtonWidget::new(Rectangle::new(0.0, 0.0, text_height, text_height))
        );
        let plot_widget = widgets.add_plot(
            PlotWidget::new(Rectangle::new(0.0, text_height + 2.0 * Self::MARGIN, 0.0, 0.0))
        );

        let mut source_widgets = Vec::new();
        for source in graph_sources {
            let kind_text = Self::get_source_kind_text(source.kind);
            let kind_widget = widgets.add_button(
                ButtonWidget::new(Rectangle::new(0.0, 0.0, Self::KIND_WIDTH, text_height)).with_text(kind_text)
            );
            let text_widget = widgets.add_text_box(
                TextBoxWidget::new(Rectangle::new(0.0, 0.0, 0.0, text_height)).with_text(&source.text)
            );
            source_widgets.push(GraphSourceWidgets::new(kind_widget, text_widget, source));
        }
        if let Some(w) = source_widgets.first() {
            widgets.focus = w.text_widget;
        }

        let mut window = Window {
            width: 0,
            height: 0,
            font_size,
            text_height,
            widgets,
            eval: expr::eval::ExprEvaluator::new().with_math_funcs().with_math_consts(),

            add_btn_widget,
            rm_btn_widget,
            min_x_widget,
            min_y_widget,
            max_x_widget,
            max_y_widget,
            zoom_axis_widget,
            plot_widget,
            source_widgets,
        };
        window.handle_bounds_texts_changed(true);
        window
    }

    fn get_source_kind_text(kind: GraphSourceKind) -> &'static str {
        match kind {
            GraphSourceKind::Expression => { "f(x)" }
            GraphSourceKind::TxtFile => { "txt" }
            GraphSourceKind::WavFile => { "wav" }
        }
    }

    fn get_source_kind_value(kind: GraphSourceKind) -> &'static str {
        match kind {
            GraphSourceKind::Expression => { "sin(x)" }
            GraphSourceKind::TxtFile => { "file.txt" }
            GraphSourceKind::WavFile => { "file.wav" }
        }
    }

    fn make_new_source_data(kind: GraphSourceKind) -> GraphSourceData {
        match kind {
            GraphSourceKind::Expression => {
                GraphSourceData::Expr(expr::Expr::Func1Call(
                    String::from("sin"),
                    Box::new(expr::Expr::Variable(String::from("x")))
                ))
            }
            GraphSourceKind::TxtFile | GraphSourceKind::WavFile => {
                GraphSourceData::Series(data::DataSeries::new( Vec::new()))
            }
        }
    }

    fn handle_resize(&mut self, d: &mut RaylibDrawHandle<'_>, force: bool) {
        let window_width = d.get_screen_width();
        let window_height = d.get_screen_height();
        if ! force && self.width == window_width && self.height == window_height {
            return;
        }
        self.width = window_width;
        self.height = window_height;

        let window_width = window_width as f32;
        let window_height = window_height as f32;

        // sources
        for (num, source) in self.source_widgets.iter().enumerate() {
            let y = num as f32 * (self.text_height + Self::MARGIN) + Self::MARGIN;
            if let Some(button) = self.widgets.get_button_mut(source.kind_widget) {
                button.rect.x = Self::MARGIN;
                button.rect.y = y;
            }
            if let Some(text) = self.widgets.get_text_box_mut(source.text_widget) {
                text.rect.x = Self::KIND_WIDTH + 2.0 * Self::MARGIN;
                text.rect.y = y;
                text.rect.width = window_width - (text.rect.x + Self::MARGIN);
            }
        }

        // add source button
        if let Some(button) = self.widgets.get_button_mut(self.add_btn_widget) {
            button.rect.x = Self::MARGIN;
            button.rect.y = self.source_widgets.len() as f32 * (self.text_height + Self::MARGIN) + Self::MARGIN;
        }

        // remove source button
        if let Some(button) = self.widgets.get_button_mut(self.rm_btn_widget) {
            button.rect.x = Self::BUTTON_WIDTH + 2.0 * Self::MARGIN;
            button.rect.y = self.source_widgets.len() as f32 * (self.text_height + Self::MARGIN) + Self::MARGIN;
        }

        // zoom axis
        if let Some(button) = self.widgets.get_button_mut(self.zoom_axis_widget) {
            button.rect.x = window_width - button.rect.width - Self::MARGIN;
            button.rect.y = self.source_widgets.len() as f32 * (self.text_height + Self::MARGIN) + Self::MARGIN;
        }

        // bounds text boxes
        for (num, &widget) in [self.min_x_widget, self.min_y_widget, self.max_x_widget, self.max_y_widget].iter().enumerate() {
            if let Some(text) = self.widgets.get_text_box_mut(widget) {
                text.rect.x = window_width - (4 - num) as f32 * (Self::MARGIN + Self::BOUND_WIDTH) - (self.text_height + Self::MARGIN);
                text.rect.y = self.source_widgets.len() as f32 * (self.text_height + Self::MARGIN) + Self::MARGIN;
            }
        }

        // plot
        if let Some(plot) = self.widgets.get_plot_mut(self.plot_widget) {
            let y = (self.source_widgets.len() + 1) as f32 * (self.text_height + Self::MARGIN);
            plot.rect.width = (window_width - 2.0 * Self::MARGIN).min(Self::GRAPH_WIDTH as f32);
            plot.rect.height = (window_height - y - 2.0 * Self::MARGIN).min(Self::GRAPH_HEIGHT as f32);
            plot.rect.x = (0.5 * (window_width - plot.rect.width)).floor();
            plot.rect.y = y + (0.5 * (window_height - plot.rect.height - y)).floor();
        }
    }

    pub fn handle_events(&mut self, d: &mut RaylibDrawHandle<'_>, font: &Font) {
        self.handle_resize(d, false);
        self.widgets.handle_keyboard(d);
        self.widgets.handle_mouse(d);

        // add/remove buttons
        let add_clicked = self.widgets.get_button_mut(self.add_btn_widget).map(|b| b.clicked(d)).unwrap_or(false);
        let rm_clicked = self.widgets.get_button_mut(self.rm_btn_widget).map(|b| b.clicked(d)).unwrap_or(false);
        if add_clicked {
            let kind = GraphSourceKind::Expression;
            let kind_text = Self::get_source_kind_text(kind);
            let text_value = String::from("sin(x)");
            let kind_widget = self.widgets.add_button(
                ButtonWidget::new(Rectangle::new(0.0, 0.0, Self::KIND_WIDTH, self.text_height)).with_text(kind_text)
            );
            let text_widget = self.widgets.add_text_box(
                TextBoxWidget::new(Rectangle::new(0.0, 0.0, 0.0, self.text_height)).with_text(&text_value),
            );
            let source = GraphSource {
                kind,
                text: text_value,
                data: Self::make_new_source_data(kind),
            };
            let source = GraphSourceWidgets::new(kind_widget, text_widget, source);
            self.widgets.focus = source.text_widget;
            self.source_widgets.push(source);
        }
        if rm_clicked && self.source_widgets.len() > 1 && let Some(removed) = self.source_widgets.pop() {
            self.widgets.widgets.pop();   // text box
            self.widgets.widgets.pop();   // kind button
            if self.widgets.focus == removed.text_widget {
                self.widgets.focus = self.source_widgets.last().map(|s| s.text_widget).unwrap_or(0);
            }
        }
        if add_clicked || rm_clicked {
            self.handle_resize(d, true);
            self.draw(d, font);
        }

        // sources
        let mut some_source_kind_changed = false;
        for source in self.source_widgets.iter_mut() {
            let focus = self.widgets.focus;
            let mut source_kind_changed = false;
            if let Some(button) = self.widgets.get_button_mut(source.kind_widget) && button.clicked(d) {
                source.source.kind = match source.source.kind {
                    GraphSourceKind::Expression => { GraphSourceKind::TxtFile }
                    GraphSourceKind::TxtFile => { GraphSourceKind::WavFile }
                    GraphSourceKind::WavFile => { GraphSourceKind::Expression }
                };
                source.source.data = Self::make_new_source_data(source.source.kind);
                button.set_text(Self::get_source_kind_text(source.source.kind));
                source_kind_changed = true;
            }
            if let Some(func) = self.widgets.get_text_box_mut(source.text_widget) {
                let focused = focus == source.text_widget;
                if source_kind_changed {
                    func.set_text(Self::get_source_kind_value(source.source.kind));
                }
                if func.handle_keyboard(d, focused) || source_kind_changed {
                    let text = func.get_text();
                    match source.source.kind {
                        GraphSourceKind::WavFile => {
                            if let Ok(wav) = data::read_wav_file(text) && let Some(chan) = wav.channels.first() {
                                let dx = 1.0 / wav.sample_rate as f64;
                                let data = chan.iter().enumerate().map(|(index, sample)| {
                                    data::DataItem::new(dx * index as f64, *sample as f64 / i16::MAX as f64)
                                }).collect::<Vec<data::DataItem>>();
                                source.source.data = GraphSourceData::Series(data::DataSeries::new(data));
                                source.invalid = false;
                            } else {
                                source.invalid = true;
                            }
                        }
                        GraphSourceKind::TxtFile => {
                            if let Ok(data) = data::read_text_file(text, None) {
                                source.source.data = GraphSourceData::Series(data::DataSeries::new(data));
                                source.invalid = false;
                            } else {
                                source.invalid = true;
                            }
                        }
                        GraphSourceKind::Expression => {
                            if let Ok(expr) = expr::Expr::parse(text) {
                                source.source.data = GraphSourceData::Expr(expr);
                                source.invalid = false;
                            } else {
                                source.invalid = true;
                            }
                        }
                    }
                    source.source.text.replace_range(.., text);
                }
                func.draw(d, font, self.font_size, focused, if source.invalid { Some(Color::RED) } else { None });
            }
            if source_kind_changed {
                some_source_kind_changed = true;
            }
        }
        if some_source_kind_changed {
            self.draw(d, font);
        }

        // bounds
        let mut bounds_changed = false;
        for widget in [self.min_x_widget, self.min_y_widget, self.max_x_widget, self.max_y_widget] {
            let focus = self.widgets.focus;
            if let Some(text) = self.widgets.get_text_box_mut(widget) && text.handle_keyboard(d, focus == widget) {
                bounds_changed = true;
            }
        }
        if bounds_changed {
            self.handle_bounds_texts_changed(false);
        }

        // zoom axis
        let set_zoom_axis = if let Some(zoom_axis) = self.widgets.get_plot(self.plot_widget).map(|plot| plot.zoom_axis) &&
            let Some(button) = self.widgets.get_button_mut(self.zoom_axis_widget) {
                let new_zoom_axis = if button.clicked(d) {
                    zoom_axis.next()
                } else {
                    zoom_axis
                };
                Some(new_zoom_axis)
            } else {
                None
            };

        // set plot zoom axis
        let plot_changed = if let Some(plot) = self.widgets.get_plot_mut(self.plot_widget) && let Some(zoom_axis) = set_zoom_axis {
            plot.zoom_axis = zoom_axis;
            plot.handle_mouse(d)
        } else {
            false
        };
        if plot_changed {
            self.draw_plot(d);
            self.handle_plot_bounds_changed(true);
        }
    }

    fn handle_bounds_texts_changed(&mut self, force_all_texts: bool) {
        let min_x = self.widgets.get_text_box(self.min_x_widget).and_then(|text| text.get_text().parse::<f64>().ok());
        let min_y = self.widgets.get_text_box(self.min_y_widget).and_then(|text| text.get_text().parse::<f64>().ok());
        let max_x = self.widgets.get_text_box(self.max_x_widget).and_then(|text| text.get_text().parse::<f64>().ok());
        let max_y = self.widgets.get_text_box(self.max_y_widget).and_then(|text| text.get_text().parse::<f64>().ok());

        if let Some(plot) = self.widgets.get_plot_mut(self.plot_widget) {
            let mut changed = false;
            if let Some(min_x) = min_x && plot.min_x != min_x { plot.min_x = min_x; changed = true; }
            if let Some(min_y) = min_y && plot.min_y != min_y { plot.min_y = min_y; changed = true; }
            if let Some(max_x) = max_x && plot.max_x != max_x { plot.max_x = max_x; changed = true; }
            if let Some(max_y) = max_y && plot.max_y != max_y { plot.max_y = max_y; changed = true; }
            if changed {
                self.handle_plot_bounds_changed(force_all_texts);
            }
        }
    }

    fn handle_plot_bounds_changed(&mut self, force_all: bool) {
        let bounds = if let Some(plot) = self.widgets.get_plot_mut(self.plot_widget) {
            Some((plot.min_x, plot.min_y, plot.max_x, plot.max_y))
        } else {
            None
        };

        if let Some((min_x, min_y, max_x, max_y)) = bounds {
            let focus = if force_all { self.widgets.widgets.len() } else { self.widgets.focus };
            if focus != self.min_x_widget && let Some(text) = self.widgets.get_text_box_mut(self.min_x_widget) {
                text.set_text(format!("{:.3}", min_x));
            }
            if focus != self.min_y_widget && let Some(text) = self.widgets.get_text_box_mut(self.min_y_widget) {
                text.set_text(format!("{:.3}", min_y));
            }
            if focus != self.max_x_widget && let Some(text) = self.widgets.get_text_box_mut(self.max_x_widget) {
                text.set_text(format!("{:.3}", max_x));
            }
            if focus != self.max_y_widget && let Some(text) = self.widgets.get_text_box_mut(self.max_y_widget) {
                text.set_text(format!("{:.3}", max_y));
            }
        }
    }

    fn draw_plot(&mut self, d: &mut RaylibDrawHandle<'_>) {
        if let Some(plot) = self.widgets.get_plot_mut(self.plot_widget) {
            plot.draw(d);
            for source in self.source_widgets.iter() {
                match &source.source.data {
                    GraphSourceData::Expr(expr) => {
                        plot.draw_expr(d, expr, &mut self.eval);
                    }
                    GraphSourceData::Series(series) => {
                        plot.draw_series(d, &series.items);
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, font: &Font) {
        d.draw_rectangle(0, 0, self.width, self.height, Color::WHITE);

        // add/remove buttons
        if let Some(button) = self.widgets.get_button_mut(self.add_btn_widget) { button.draw(d, font, self.font_size); }
        if let Some(button) = self.widgets.get_button_mut(self.rm_btn_widget) { button.draw(d, font, self.font_size); }
        let focus = self.widgets.focus;

        // sources
        for source in self.source_widgets.iter_mut() {
            if let Some(button) = self.widgets.get_button_mut(source.kind_widget) {
                button.draw(d, font, self.font_size);
            }
            if let Some(func) = self.widgets.get_text_box_mut(source.text_widget) {
                let high_color = if source.invalid { Some(Color::RED) } else { None };
                func.draw(d, font, self.font_size, focus == source.text_widget, high_color);
            }
        }

        // bounds
        for widget in [self.min_x_widget, self.min_y_widget, self.max_x_widget, self.max_y_widget] {
            if let Some(text) = self.widgets.get_text_box_mut(widget) {
                text.draw(d, font, self.font_size, focus == widget, None);
            }
        }

        // zoom axis
        if let Some(zoom_axis) = self.widgets.get_plot(self.plot_widget).map(|plot| plot.zoom_axis) &&
            let Some(button) = self.widgets.get_button_mut(self.zoom_axis_widget) {
                button.draw(d, font, self.font_size);
                draw_zoom_axis_button(d, zoom_axis, button.rect);
            }

        // graph plot
        self.draw_plot(d);
    }
}
