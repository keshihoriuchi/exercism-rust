#![feature(test)]
#![warn(clippy::all)]

extern crate test;

use std::io::Read;
use std::io::Write;
use std::fs;

pub fn brainfxck<T: Read, U: Write>(s: &str, input: &mut T, output: &mut U) {
    let mut buf: [u8; 30000] = [0; 30000];
    let mut ptr = 0;
    let mut bracket_stack: Vec<usize> = vec![];
    let mut pc = 0;
    let src: Vec<char> = s.chars().collect();
    let len = src.len();
    while pc < len {
        match src[pc] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => buf[ptr] += 1,
            '-' => buf[ptr] -= 1,
            '.' => {
                output.write_all(&[buf[ptr]]).unwrap();
            }
            ',' => {
                let mut b = [0; 1];
                input.read_exact(&mut b).unwrap();
                buf[ptr] = b[0];
            }
            '[' => {
                if buf[ptr] == 0 {
                    while src[pc] != ']' {
                        pc += 1;
                    }
                } else {
                    bracket_stack.push(pc);
                }
            }
            ']' => {
                pc = bracket_stack.pop().unwrap();
                continue;
            }
            _ => {}
        }
        pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn hello_world() {
        let mut v = vec![];
        super::brainfxck(
            r#"
                >+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++
                ++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>
                ++++++++[<++++>-]<+.[-]++++++++++.
            "#,
            &mut std::io::stdin(),
            &mut v,
        );
        assert_eq!("Hello World!\n", std::str::from_utf8(&v).unwrap());
    }

    #[bench]
    fn bench_brainfxck(b: &mut Bencher) {
        b.iter(|| {
            let mut v = vec![];
            super::brainfxck(
                r#"
                >+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++
                ++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>
                ++++++++[<++++>-]<+.[-]++++++++++.
            "#,
                &mut std::io::stdin(),
                &mut v,
            );
        });
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = fs::read_to_string(&args[1]).unwrap();

    brainfxck(&code, &mut std::io::stdin(), &mut std::io::stdout());
}
