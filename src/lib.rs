#![forbid(unsafe_code)]

mod default_environment;
mod interpreter;
mod parser;

pub use default_environment::default_env;
pub use interpreter::{eval, eval_block};
pub use parser::{parse, ParseError};

pub mod model;
pub mod utils;
#[macro_use]
pub mod macros;

use model::Env;
use std::io::{self, prelude::*};
use std::{cell::RefCell, rc::Rc};

// 🦀 I am all over this project!
/// Starts a REPL prompt at stdin/stdout. **This will block the current thread.**
pub fn start_repl(env: Option<Env>) {
    let env_rc = Rc::new(RefCell::new(env.unwrap_or_else(default_env)));

    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        match eval_block(env_rc.clone(), parse(&line.unwrap()).filter_map(|a| a.ok())) {
            Ok(val) => println!("{}", val),
            Err(e) => println!("{}", e),
        };

        print!("> ");
        io::stdout().flush().unwrap();
    }

    // Properly go to the next line after quitting
    println!();
}
