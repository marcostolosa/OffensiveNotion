use std::process::Command;
use std::error::Error;

pub fn handle(s: String) -> Result<String, Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg(s)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("/bin/bash")
            .arg("-c")
            .arg(s)
            .output()
            .expect("failed to execute process")
    };
    let output_string: String;
    if output.stderr.len() > 0 {
        output_string = String::from_utf8(output.stderr).unwrap();
    } else {
        output_string = String::from_utf8(output.stdout).unwrap();
    }
    return Ok(output_string);
}