use std::process::Command;

pub fn execute(input: &str) -> Result<Box<str>, Box<dyn std::error::Error + Send + Sync>> {
    let command = Command::new("cmd").arg("-C").arg(input).output();

    let output = match command {
        Ok(output) => output,
        Err(e) => return Err(Box::new(e)),
    };

    if output.status.success() {
        return Ok(Box::from(String::from_utf8_lossy(&output.stdout)));
    }

    Err(Box::from(String::from_utf8_lossy(&output.stderr)))
}
