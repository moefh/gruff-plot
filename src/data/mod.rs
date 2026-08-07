mod text_file;
mod wav_file;
mod reader;

pub use text_file::*;
pub use wav_file::*;

#[derive(Debug)]
pub struct DataItem {
    pub x: f64,
    pub y: f64,
}

impl DataItem {
    pub fn new(x: f64, y: f64) -> Self {
        DataItem { x, y }
    }
}

#[derive(Debug)]
pub struct DataSeries {
    pub items: Vec<DataItem>,
}

impl DataSeries {
    pub fn new(items: Vec<DataItem>) -> Self {
        DataSeries { items }
    }
}
