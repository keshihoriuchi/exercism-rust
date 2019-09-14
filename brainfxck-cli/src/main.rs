#![feature(test)]
#![warn(clippy::all)]

extern crate test;

use std::fs;
use std::io::Read;
use std::io::Write;

enum Command {
    RShift(usize),
    LShift(usize),
    Inc(u8),
    Dec(u8),
    PutC,
    GetC,
    WhileS(usize),
    WhileE(usize),
}

pub fn brainfxck<R: Read, W: Write>(s: &str, input: &mut R, output: &mut W) {
    let mut buf: [u8; 30000] = [0; 30000];
    let mut ptr = 1000;
    let mut pc = 0;
    let src: Vec<char> = s.chars().collect();
    let mut len = src.len();
    let mut bracket_stack: Vec<usize> = vec![];
    let mut commands: Vec<Command> = vec![];
    while pc < len {
        match src[pc] {
            '>' => {
                let mut v = 1;
                loop {
                    pc += 1;
                    if src[pc] != '>' {
                        commands.push(Command::RShift(v));
                        break;
                    }
                    v += 1;
                }
                continue;
            }
            '<' => {
                let mut v = 1;
                loop {
                    pc += 1;
                    if src[pc] != '<' {
                        commands.push(Command::LShift(v));
                        break;
                    }
                    v += 1;
                }
                continue;
            }
            '+' => {
                let mut v = 1;
                loop {
                    pc += 1;
                    if src[pc] != '+' {
                        commands.push(Command::Inc(v));
                        break;
                    }
                    v += 1;
                }
                continue;
            }
            '-' => {
                let mut v = 1;
                loop {
                    pc += 1;
                    if src[pc] != '-' {
                        commands.push(Command::Dec(v));
                        break;
                    }
                    v += 1;
                }
                continue;
            }
            '.' => commands.push(Command::PutC),
            ',' => commands.push(Command::GetC),
            '[' => {
                bracket_stack.push(commands.len());
                commands.push(Command::WhileS(0));
            }
            ']' => {
                let left_pc = bracket_stack.pop().unwrap();
                commands[left_pc] = Command::WhileS(commands.len());
                commands.push(Command::WhileE(left_pc));
            }
            _ => (),
        }
        pc += 1;
    }

    len = commands.len();
    pc = 0;
    while pc < len {
        match commands[pc] {
            Command::RShift(n) => ptr += n,
            Command::LShift(n) => ptr -= n,
            Command::Inc(n) => buf[ptr] += n,
            Command::Dec(n) => buf[ptr] -= n,
            Command::PutC => {
                output.write_all(&[buf[ptr]]).unwrap();
            }
            Command::GetC => {
                let mut b = [0; 1];
                input.read_exact(&mut b).unwrap();
                buf[ptr] = b[0];
            }
            Command::WhileS(n) => {
                if buf[ptr] == 0 {
                    pc = n;
                }
            }
            Command::WhileE(n) => {
                pc = n;
                continue;
            }
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
