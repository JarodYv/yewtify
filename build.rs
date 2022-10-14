use anyhow::{anyhow, Error};
use rsass::{compile_scss_path, output};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    let path = "./styles/yewtify.sass".as_ref();
    let format = output::Format {
        style: output::Style::Compressed,
        ..Default::default()
    };
    let css = compile_scss_path(path, format)
        .map_err(|err| anyhow!("sass builder failed with: {}", err))?;
    let mut path = PathBuf::from(env::var("YEWTIFY_OUT")?);
    path.push("yewtify.css");
    let mut file = File::create(path)?;
    file.write_all(&css)?;
    Ok(())
}
