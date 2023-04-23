use std::error::Error;
use std::process::Command;

type DynError = Box<dyn Error>;

pub fn build(env: String) -> Result<(), DynError> {
    match env.as_str() {
        "prod" => build_prod()?,
        _ => build_dev()?,
    }
    Ok(())
}

pub fn build_dev() -> Result<(), DynError> {
    Command::new("npx")
        .args(&[
            "tailwindcss",
            "-c",
            "./tailwind.config.js",
            "-o",
            "./tailwind.css",
        ])
        .current_dir("./rustconf")
        .status()?;
    Ok(())
}

fn build_prod() -> Result<(), DynError> {
    Command::new("npx")
        .args(&[
            "tailwindcss",
            "-c",
            "./tailwind.config.js",
            "-o",
            "./tailwind.css",
            "--minify",
        ])
        .current_dir("./rustconf")
        .status()?;
    Ok(())
}
