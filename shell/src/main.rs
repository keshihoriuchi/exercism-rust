use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(PartialEq, Debug)]
pub struct Cmd {
    pub program: String,
    pub args: Vec<String>,
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

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    print!("$ ");
    io::stdout().flush()?;

    for line_result in stdin.lock().lines() {
        let line = line_result?;
        let cmds = match shell_line::exec_unit(&line) {
            Ok(cmds) => cmds,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "illegal format"))
        };
        if cmds.len() == 0 { continue }

        let procs: Vec<std::process::Child> = cmds.iter().enumerate().map(|(i, cmd)| {
            let stdin = if i == 0 {
                Stdio::inherit()
            } else {
                Stdio::piped()
            };
            let stdout = if i == cmds.len() - 1 {
                Stdio::inherit()
            } else {
                Stdio::piped()
            };
            let r = Command::new(&cmd.program)
                .args(&cmd.args).stdin(stdin).stdout(stdout).stderr(Stdio::inherit()).spawn();
            match r {
                Ok(child) => child,
                Err(e) => {
                    println!("{:?}", e);
                    panic!();
                }
            }
        }).collect();

        print!("$ ");
        io::stdout().flush()?;
    }
    Ok(())
}
