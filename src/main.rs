mod widgets;
mod expr;
mod data;
mod window;

use std::io::{
    Result,
    Error,
};
use raylib::prelude::*;

static FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/ComicMono.ttf");
const FONT_SIZE: i32 = 24;

#[derive(Debug, Copy, Clone)]
pub enum GraphSourceKind {
    Expression,
    TxtFile,
    WavFile,
}

#[derive(Debug)]
pub enum GraphSourceData {
    Expr(expr::Expr),
    Series(data::DataSeries),
}

#[derive(Debug)]
pub struct GraphSource {
    pub kind: GraphSourceKind,
    pub data: GraphSourceData,
    pub text: String,
}

impl GraphSource {
    fn expression(text: String, expr_env: &expr::Environment) -> Result<Self> {
        let expr = expr::Expr::parse(&text, expr_env).map_err(|e| {
            Error::other(format!("ERROR parsing expression '{}': {}", text, e))
        })?;
        Ok(GraphSource {
            kind: GraphSourceKind::Expression,
            data: GraphSourceData::Expr(expr),
            text,
        })
    }

    fn txt_file(filename: &std::ffi::OsString) -> Result<Self> {
        let data = data::read_text_file(filename, Some((0.0, 1.0))).map_err(|e| {
            Error::other(format!("ERROR reading text file {:?}: {}", filename, e))
        })?;
        Ok(GraphSource {
            kind: GraphSourceKind::TxtFile,
            text: filename.to_string_lossy().to_string(),
            data: GraphSourceData::Series(data::DataSeries::new(data)),
        })
    }

    fn wav_file(filename: &std::ffi::OsString) -> Result<Self> {
        let wav = data::read_wav_file(filename).map_err(|e| {
            Error::other(format!("ERROR reading WAV file {:?}: {}", filename, e))
        })?;
        let dx = 1.0 / wav.sample_rate as f64;
        if let Some(chan) = wav.channels.first() {
            let data = chan.iter().enumerate().map(|(index, sample)| {
                data::DataItem::new(dx * index as f64, *sample as f64 / i16::MAX as f64)
            }).collect::<Vec<data::DataItem>>();
            Ok(GraphSource {
                kind: GraphSourceKind::WavFile,
                text: filename.to_string_lossy().to_string(),
                data: GraphSourceData::Series(data::DataSeries::new(data)),
            })
        } else {
            Err(Error::other("WAV file hs no channels"))
        }
    }
}

fn read_cmdline_options(expr_env: &expr::Environment) -> Result<Vec<GraphSource>> {
    let mut sources = Vec::new();

    let argv = std::env::args_os().skip(1).collect::<Vec<_>>();
    let mut argn = 0;
    while let Some(arg) = argv.get(argn) {
        if arg == "-h" {
            println!("USAGE: gruff-plot [options]");
            println!();
            println!("options:");
            println!("  -h            show this help");
            println!("  -e EXPR       plot expression (a function of 'x')");
            println!("  -w FILE.wav   plot WAV file samples");
            println!("  -t FILE.txt   plot text file values");
            std::process::exit(0);
        }

        if arg == "-e" {
            argn += 1;
            if let Some(expr) = argv.get(argn) {
                sources.push(GraphSource::expression(expr.to_string_lossy().to_string(), expr_env)?);
            } else {
                println!("ERROR: option '-e' requires expression");
                std::process::exit(1);
            }
            argn += 1;
            continue;
        }

        if arg == "-t" {
            argn += 1;
            if let Some(filename) = argv.get(argn) {
                sources.push(GraphSource::txt_file(filename)?);
            } else {
                println!("ERROR: option '-t' requires file name");
                std::process::exit(1);
            }
            argn += 1;
            continue;
        }

        if arg == "-w" {
            argn += 1;
            if let Some(filename) = argv.get(argn) {
                sources.push(GraphSource::wav_file(filename)?);
            } else {
                println!("ERROR: option '-w' requires file name");
                std::process::exit(1);
            }
            argn += 1;
            continue;
        }

        println!("ERROR: unknown option: '{}'", arg.display());
        std::process::exit(1);
    }

    if sources.is_empty() {
        sources.push(GraphSource::expression(String::from("sin(x)"), expr_env)?);
    }
    Ok(sources)
}

pub fn main() {
    let expr_env = expr::Environment::new().with_math_functions().with_math_constants().with_var("x", 0.0);
    let sources = match read_cmdline_options(&expr_env) {
        Ok(s) => { s }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let (mut rl, thread) = raylib::init()
        .log_level(TraceLogLevel::LOG_ERROR)
        .resizable()
        .size(1200, 800)
        .title("Gruff Plot")
        .vsync()
        .build();
    rl.set_target_fps(60);
    rl.set_exit_key(None);
    rl.set_window_min_size(800, 600);

    let font = match rl.load_font_from_memory(&thread, ".ttf", FONT_BYTES, FONT_SIZE, None) {
        Ok(font) => { font }
        Err(e) => { println!("ERROR loading font: {}", e); return; }
    };

    let mut window = window::Window::new(FONT_SIZE as f32, sources, expr_env);

    while ! rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.enable_event_waiting();
        d.clear_background(Color::WHITE);

        window.handle_events(&mut d, &font);
        window.draw(&mut d, &font);

        let control_held = d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || d.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL);
        if d.is_key_pressed(KeyboardKey::KEY_Q) && control_held {
            break;
        }
    }
}
