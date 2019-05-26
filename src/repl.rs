extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::parse;

pub fn repl(default_prompt: &str) {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(default_prompt);
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    match parse::parse_input(&line) {
                        /* TODO: eval all expressions */
                        Ok(exprs) => for expr in exprs {
                            println!("=> {}", expr)
                        },
                        Err(err) => println!("Parse error: {:?}", err)
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("To quit, use Ctrl-D")
            },
            Err(ReadlineError::Eof) => {
                println!("Exiting.");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
