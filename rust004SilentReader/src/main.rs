use std::io::{self, Write};
use std::process::Command;

fn get_pass() -> Result<String, io::Error> {
    print!("in: ");
    
    io::stdout().flush()?;
    Command::new("stty").arg("-echo").status()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Command::new("stty").arg("echo").status()?;

    println!();

    Ok(input.trim_end().to_string())
}

fn main() {
    match get_pass() {
        Ok(a) => println!("res: {a}"),
        Err(e) => println!("err: {e}"),
    }
}