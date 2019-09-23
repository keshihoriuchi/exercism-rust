#![warn(clippy::all)]

extern crate getopts;
extern crate term_size;
extern crate unicode_width;

use std::env;
use std::fs;
use std::io::{stderr, stdout, BufWriter, Write};
use std::path::Path;
use unicode_width::UnicodeWidthStr;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let opts = getopts::Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let dir = env::current_dir()?;
    let path = if matches.free.is_empty() {
        &dir
    } else {
        Path::new(&matches.free[0])
    };
    if !path.exists() {
        writeln!(stderr(), "No such file or directory")?;
        std::process::exit(1);
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    if path.is_dir() {
        let entries: Vec<String> = fs::read_dir(path)?
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .filter(|s| !s.starts_with('.'))
            .collect();

        // 行数と列の幅を求める
        let term_width = term_size::dimensions().unwrap().0;            
        let mut num_col = 1;
        let mut col_widths: Vec<usize> = vec![];
        loop {
            let mut cols: Vec<Vec<&str>> = vec![];
            for _ in 0..num_col {
                cols.push(vec![]);
            }
            entries.chunks(num_col).for_each(|chunks| {
                chunks.iter().enumerate().for_each(|(i, s)| {
                    let c = &mut cols[i];
                    c.push(s);
                })
            });
            let new_col_widths: Vec<usize> = cols
                .iter()
                .map(|c| c.iter().map(|s| UnicodeWidthStr::width(*s)).max().unwrap())
                .collect();
            let width = new_col_widths.iter().sum::<usize>() + ((num_col - 1) * 2);
            if width > term_width || num_col == entries.len() {
                if num_col == 1 || num_col == entries.len() {
                    col_widths = new_col_widths;
                } else if width > term_width {
                    num_col -= 1;
                }
                break;
            }
            col_widths = new_col_widths;
            num_col += 1;
        }

        let mut result = String::new();
        entries.chunks(num_col).for_each(|chunks| {
            chunks.iter().enumerate().for_each(|(i, s)| {
                result.push_str(s);
                let num_ws = col_widths[i] - UnicodeWidthStr::width(&*(s.as_str()));
                result.push_str(&" ".repeat(num_ws));
                if i == chunks.len() - 1 {
                    result.push_str("\n");
                } else {
                    result.push_str("  ");
                }
            })
        });
        write!(out, "{}", result).unwrap();
    } else {
        write!(out, "{}  ", path.to_str().unwrap()).unwrap();
    }

    Ok(())
}
