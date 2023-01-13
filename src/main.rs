use std::collections::HashSet;
use std::fmt::Error;
use std::io::Write;
use std::process::{Command, Stdio};
fn main() -> Result<(), std::io::Error> {
    let mut child = Command::new("python")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    println!("{:?}", child);
    Ok(())
}
