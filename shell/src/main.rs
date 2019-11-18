#![warn(clippy::all)]

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(PartialEq, Debug)]
pub struct Cmd {
    pub program: String,
    pub args: Vec<String>,
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
}

pub enum UnitRedirect {
    Unit(String),
    In(String),
    Out(String),
}

peg::parser!( grammar shell_line() for str {
    pub rule exec_unit() -> Vec<Cmd>
        = c:cmd() ** (_ "|" _) { c }

    rule cmd() -> Cmd = cmd_with_args() / cmd_without_args()

    rule cmd_with_args() -> Cmd
        = _ c:unit() __ a:unit_or_redirect() ** __ {
            let mut args = vec![];
            let mut stdin_file = None;
            let mut stdout_file = None;
            for u_or_r in a {
                match u_or_r {
                    UnitRedirect::Unit(s) => {
                        args.push(s)
                    },
                    UnitRedirect::In(s) => {
                        stdin_file = Some(s)
                    },
                    UnitRedirect::Out(s) => {
                        stdout_file = Some(s)
                    }
                }
            }
            Cmd { program: c, args: args, stdin_file: stdin_file, stdout_file: stdout_file}
        }

    rule cmd_without_args() -> Cmd
        = _ c:unit() _ { Cmd { program: c, args: vec![], stdin_file: None, stdout_file: None} }
    
    rule _() = [' ' | '\t']*
    rule __() = [' ' | '\t']+

    rule unit_or_redirect() -> UnitRedirect
        = ">" _ u:unit() { UnitRedirect::Out(u) } / "<" _ u:unit() { UnitRedirect::In(u) } / u:unit() { UnitRedirect::Unit(u) }

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
        assert_eq!(shell_line::exec_unit(""), Ok(vec![]));

        assert_eq!(
            shell_line::exec_unit("echo hoge"),
            Ok(vec![Cmd {
                program: String::from("echo"),
                args: vec![String::from("hoge")],
                stdin_file: None,
                stdout_file: None
            }])
        );

        assert_eq!(
            shell_line::exec_unit("echo \"hoge fuga\""),
            Ok(vec![Cmd {
                program: String::from("echo"),
                args: vec![String::from("hoge fuga")],
                stdin_file: None,
                stdout_file: None
            }])
        );

        assert_eq!(
            shell_line::exec_unit("echo 'hoge fuga'"),
            Ok(vec![Cmd {
                program: String::from("echo"),
                args: vec![String::from("hoge fuga")],
                stdin_file: None,
                stdout_file: None
            }])
        );

        assert_eq!(
            shell_line::exec_unit("echo fuga | cat"),
            Ok(vec![
                Cmd {
                    program: String::from("echo"),
                    args: vec![String::from("fuga")],
                    stdin_file: None,
                    stdout_file: None
                },
                Cmd {
                    program: String::from("cat"),
                    args: vec![],
                    stdin_file: None,
                    stdout_file: None
                }
            ])
        );

        assert_eq!(
            shell_line::exec_unit("cat ./foo.txt | grep -v 'moge'|ws -l"),
            Ok(vec![
                Cmd {
                    program: String::from("cat"),
                    args: vec![String::from("./foo.txt")],
                    stdin_file: None,
                    stdout_file: None
                },
                Cmd {
                    program: String::from("grep"),
                    args: vec![String::from("-v"), String::from("moge")],
                    stdin_file: None,
                    stdout_file: None
                },
                Cmd {
                    program: String::from("ws"),
                    args: vec![String::from("-l")],
                    stdin_file: None,
                    stdout_file: None
                }
            ])
        );

        assert_eq!(
            shell_line::exec_unit("ls > y"),
            Ok(vec![Cmd {
                program: String::from("ls"),
                args: vec![],
                stdin_file: None,
                stdout_file: Some(String::from("y"))
            }])
        );

        assert_eq!(
            shell_line::exec_unit("cat < y | sort | uniq | wc > y1"),
            Ok(vec![
                Cmd {
                    program: String::from("cat"),
                    args: vec![],
                    stdin_file: Some(String::from("y")),
                    stdout_file: None
                },
                Cmd {
                    program: String::from("sort"),
                    args: vec![],
                    stdin_file: None,
                    stdout_file: None
                },
                Cmd {
                    program: String::from("uniq"),
                    args: vec![],
                    stdin_file: None,
                    stdout_file: None
                },
                Cmd {
                    program: String::from("wc"),
                    args: vec![],
                    stdin_file: None,
                    stdout_file: Some(String::from("y1"))
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
        // 行をパース
        let line = line_result?;
        let cmds = match shell_line::exec_unit(&line) {
            Ok(cmds) => cmds,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "illegal format")),
        };

        // プロセスを起動しない処理
        if cmds.is_empty() {
            print!("$ ");
            io::stdout().flush()?;
            continue;
        }
        if cmds.len() == 1 && &cmds[0].program == "exit" {
            return Ok(());
        }

        // プロセスを起動する処理
        let mut c = cmds.iter().enumerate().fold(
            None,
            |previous_proc: Option<std::process::Child>, (i, cmd)| {
                let stdin = match (previous_proc, &cmd.stdin_file) {
                    (_, Some(f)) => {
                        let file = File::open(f).unwrap();
                        Stdio::from(file)
                    }
                    (None, None) => Stdio::inherit(),
                    (Some(p), _) => Stdio::from(p.stdout.unwrap()),
                };

                let stdout = match &cmd.stdout_file {
                    Some(f) => {
                        let file = File::create(f).unwrap();
                        Stdio::from(file)
                    }
                    None if i == cmds.len() - 1 => Stdio::inherit(),
                    None => Stdio::piped(),
                };

                let r = Command::new(&cmd.program)
                    .args(&cmd.args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .stderr(Stdio::inherit())
                    .spawn();
                match r {
                    Ok(c) => Some(c),
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                }
            },
        );

        // 末尾のプロセスをウェイト
        if let Some(ref mut child) = c {
            child.wait()?;
        }

        print!("$ ");
        io::stdout().flush()?;
    }
    Ok(())
}
