use error_chain::error_chain;
use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

// error_chain! {
//     errors { CmdError }
//     foreign_links {
//         Io(std::io::Error);
//         Utf8(std::string::FromUtf8Error);
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("python3")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        // loop through the output and find unique words
        let raw_output = String::from_utf8(output.stdout)?;

        let unique_words = raw_output
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", unique_words.len());
        println!("{:#?}", unique_words);
    } else {
        println!("Error: {}", String::from_utf8(output.stderr)?);
    }

    Ok(())
}
