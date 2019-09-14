#![feature(test)]
#![warn(clippy::all)]

extern crate test;

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::io::Write;

pub fn brainfxck<T: Read, U: Write>(s: &str, input: &mut T, output: &mut U) {
    let mut buf: [u8; 30000] = [0; 30000];
    let mut ptr = 1000;
    let mut pc = 0;
    let src: Vec<char> = s
        .chars()
        .filter(|c| match c {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        })
        .collect();
    let len = src.len();
    let mut bracket_stack: Vec<usize> = vec![];
    let mut jump_table = HashMap::new();
    while pc < len {
        match src[pc] {
            '[' => bracket_stack.push(pc),
            ']' => {
                let left_pc = bracket_stack.pop().unwrap();
                let right_pc = pc;
                jump_table.insert(left_pc, right_pc);
                jump_table.insert(right_pc, left_pc);
            }
            _ => (),
        }
        pc += 1;
    }
    pc = 0;
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
                    pc = *jump_table.get(&pc).unwrap();
                }
            }
            ']' => {
                pc = *jump_table.get(&pc).unwrap();
                continue;
            }
            _ => panic!(),
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

    #[test]
    fn mul_to_8() {
        let mut v = vec![];
        super::brainfxck(
            r#"
                ++++>++><<
                [-
                  >[->>+<<]
                  >>[-<+<+>>]
                  <<<
                ]>>
                ++++++++++++++++++++++++++++++++++++++++++++++++.
            "#,
            &mut std::io::stdin(),
            &mut v,
        );
        assert_eq!("8", std::str::from_utf8(&v).unwrap());
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
