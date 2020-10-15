use anyhow::Result;
use std::io;
use std::io::Read;
use ytt::{convert, Format};

fn main() -> Result<()> {
    let mut stdin = String::new();
    io::stdin().lock().read_to_string(&mut stdin)?;
    let toml = convert(&stdin, Format::Json)?;
    println!("{}", toml);
    Ok(())
}
