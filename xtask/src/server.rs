use std::error::Error;
use std::process::Command;

type DynError = Box<dyn Error>;

pub fn serve() -> Result<(), DynError>  {
    Command::new("trunk")
        .args(&[
            "serve",
            "--proxy-backend=https://yew.rs/tutorial",
        ])
        .current_dir("./rustconf")
        .status()?;

    Ok(())
}