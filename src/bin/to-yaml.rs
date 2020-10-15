use anyhow::Result;
use rosa_parse::{convert, Format};
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut stdin = String::new();
    io::stdin().lock().read_to_string(&mut stdin)?;
    let yaml = convert(&stdin, Format::Yaml)?;
    println!("{}", yaml);
    Ok(())
}
