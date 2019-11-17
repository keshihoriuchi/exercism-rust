use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(PartialEq, Debug)]
pub struct Cmd {
    program: String,
    args: Vec<String>,
}

peg::parser!( grammar shell_line() for str {
    pub rule exec_unit() -> Vec<Cmd>
        = c:cmd() ** (_ "|" _) { c }

    rule cmd() -> Cmd
        = _ c:unit() __ a:unit() ** __ { Cmd { program: String::from(c), args: a } }
    
    rule _() = [' ' | '\t']*
    rule __() = [' ' | '\t']+

    rule unit() -> String
        = v:(quoted_unit() / nonquoted_unit()) { v }
    
    rule quoted_unit() -> String
        = "\"" v:$([' ' | '!' | '#'..=std::char::MAX ]*) "\"" { String::from(v) } /
          "'" v:$([' '..='&' | '('..=std::char::MAX ]*) "'" { String::from(v) }

    rule nonquoted_unit() -> String
        = v:$(['!'..='{' | '}' ..=std::char::MAX]+) { String::from(v) }
});

#[cfg(test)]
mod tests {
    use super::shell_line;
    use super::Cmd;
    #[test]
    fn shell_line_works() {
        assert_eq!(
            shell_line::exec_unit(""),
            Ok(vec![])
        );

        assert_eq!(
            shell_line::exec_unit("echo hoge"),
            Ok(vec![Cmd {
                program: String::from("echo"),
                args: vec![String::from("hoge")]
            }])
        );

        assert_eq!(
            shell_line::exec_unit("echo \"hoge fuga\""),
            Ok(vec![Cmd {
                program: String::from("echo"),
                args: vec![String::from("hoge fuga")]
            }])
        );

        assert_eq!(
            shell_line::exec_unit("echo 'hoge fuga'"),
            Ok(vec![Cmd {
                program: String::from("echo"),
                args: vec![String::from("hoge fuga")]
            }])
        );

        assert_eq!(
            shell_line::exec_unit("cat ./foo.txt | grep -v 'moge'|ws -l"),
            Ok(vec![
                Cmd {
                    program: String::from("cat"),
                    args: vec![String::from("./foo.txt")]
                },
                Cmd {
                    program: String::from("grep"),
                    args: vec![String::from("-v"), String::from("moge")]
                },
                Cmd {
                    program: String::from("ws"),
                    args: vec![String::from("-l")]
                }
            ])
        );
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    print!("$ ");
    io::stdout().flush()?;

    for line_result in stdin.lock().lines() {
        let line = line_result?;
        let cmds = shell_line::exec_unit(&line)?;
        if cmds.len() == 0 { continue }

        match line {
            [] => break,
            Some(cmd) => {
                let result = Command::new(cmd)
                    .args(arg_iter.collect::<Vec<&str>>())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status();
                match result {
                    Ok(_status) => (),
                    Err(err) => println!("{:?}", err),
                }
            }
            None => (),
        }

        print!("$ ");
        io::stdout().flush()?;
    }
    Ok(())
}
