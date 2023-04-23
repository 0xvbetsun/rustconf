mod tailwind;
mod server;
use std::{env, error::Error};

type DynError = Box<dyn Error>;

pub mod tasks {
    use crate::DynError;
    use crate::tailwind::{build, build_dev};
    use crate::server::serve;

    pub fn tailwind(env: Option<String>) -> Result<(), DynError> {
        match env {
            Some(env) => build(env),
            None => build_dev(),
        }
    }

    pub fn server() -> Result<(), DynError> {
       serve()
    }

    pub fn print_help() {
        println!(
            "
Usage: Run with `cargo xtask <task>`, eg. `cargo xtask tailwind prod`.
    Tasks:
        tailwind <env>: Builds tailwind css for specified env.
        serve: Runs development server.
"
        );
    }
}

fn main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task {
        None => tasks::print_help(),
        Some(t) => match t.as_str() {
            "tailwind" => tasks::tailwind(env::args().nth(2))?,
            "serve" => tasks::server()?,
            invalid => return Err(format!("Invalid task name: {}", invalid).into()),
        },
    };
    Ok(())
}
