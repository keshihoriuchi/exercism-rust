use std::io::Read;
use std::io::Write;

pub fn brainfxck<T: Read, U: Write>(s: &str, input: &mut T, output: &mut U) {
    let mut buf: [u8; 30000] = [0; 30000];
    let mut ptr = 0;
    let mut bracket_stack: Vec<usize> = vec![];
    let mut pc = 0;
    let src: Vec<char> = s.chars().collect();
    let len = src.len();
    'w: while pc < len {
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
                continue 'w;
            }
            _ => {}
        }
        pc += 1;
    }
}

#[cfg(test)]
mod tests {
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
}
