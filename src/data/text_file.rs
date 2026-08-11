use std::fs::File;
use std::path::Path;
use std::io::{
    Error,
    Result,
    BufRead,
    BufReader,
};

use super::DataItem;

fn skip_spaces(input: &str) -> usize {
    input
        .char_indices()
        .find(|(_index, ch)| *ch != ' ' && *ch != '\t')
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn parse_float(input: &str) -> Option<(f64, usize)> {
    let start = skip_spaces(input);
    let end = start + input[start..]
        .char_indices()
        .find(|(_index, ch)| ! "0123456789.e-".contains(*ch))
        .map(|(index, _)| index)
        .unwrap_or(input.len());
    let val = input[start..end].parse::<f64>().ok()?;
    Some((val, end))
}

pub fn read_text_file(filename: impl AsRef<Path>, x_range: Option<(f64, f64)>) -> Result<Vec<DataItem>> {
    let mut num_columns = None;
    let mut reader = BufReader::new(File::open(filename.as_ref())?);
    let mut buffer = String::with_capacity(256);
    let mut data = Vec::new();
    let mut line_num = 0;
    while reader.read_line(&mut buffer)? > 0 {
        line_num += 1;
        let (val1, end) = parse_float(&buffer).ok_or_else(|| {
            Error::other(format!("line {}: syntax error: '{}'", line_num, buffer))
        })?;
        let end = end + skip_spaces(&buffer[end..]);
        let end = if buffer[end..].starts_with(',') || buffer[end..].starts_with(';') {
            end + 1 + skip_spaces(&buffer[end+1..])
        } else {
            end
        };
        if end >= buffer.len() || buffer[end..].starts_with('\n') || buffer[end..].starts_with('\r') {
            if num_columns == Some(2) {
                return Err(Error::other(format!("line {}: inconsistent number of columns", line_num)));
            } else if num_columns.is_none() {
                num_columns = Some(1);
            }
            data.push(DataItem::new(0.0, val1));
            buffer.clear();
            continue;
        }

        if num_columns == Some(1) {
            return Err(Error::other(format!("line {}: inconsistent number of columns", line_num)));
        } else if num_columns.is_none() {
            num_columns = Some(2);
        }
        let (val2, _) = parse_float(&buffer[end..]).ok_or_else(|| {
             Error::other(format!("line {}: syntax error: '{}'", line_num, buffer))
        })?;
        data.push(DataItem::new(val1, val2));
        buffer.clear();
    }

    if num_columns == Some(1) {
        let (x_start, x_end) = x_range.unwrap_or((0.0, 1.0));
        if data.len() == 1 {
            data[0].x = x_start;
        } else if data.len() > 1 {
            let dx = (x_end - x_start) / (data.len() - 1) as f64;
            for (i, item) in data.iter_mut().enumerate() {
                item.x = x_start + i as f64 * dx;
            }
        }
    }

    Ok(data)
}
