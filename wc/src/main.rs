#![warn(clippy::all)]

extern crate getopts;
use std::env;
use std::fs;
use std::io;

fn read<T: io::BufRead>(mut reader: T, c: bool, l: bool, w: bool) -> Vec<u32> {
    let mut bytes: u32 = 0;
    let mut lines: u32 = 0;
    let mut words: u32 = 0;
    loop {
        let mut buf = String::new();
        let num_bytes = reader.read_line(&mut buf).unwrap();
        if num_bytes == 0 {
            break;
        }
        bytes += num_bytes as u32;
        lines += 1;
        words += buf.split_whitespace().count() as u32;
    }

    let mut v = vec![];
    if l {
        v.push(lines);
    }
    if w {
        v.push(words);
    }
    if c {
        v.push(bytes);
    }
    v
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optflag("c", "bytes", "print bytes");
    opts.optflag("l", "lines", "print lines");
    opts.optflag("w", "words", "print word");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let results = if matches.free.is_empty() {
        let i = io::stdin();
        read(io::BufReader::new(i), true, true, true)
    } else {
        let f = fs::File::open(&matches.free[0])?;
        read(io::BufReader::new(f), true, true, true)
    };
    println!("{} {} {}", results[0], results[1], results[2]);
    Ok(())
}
