use std::process::Command;

fn main() {
    // Our command will be "python test.py"
    let mut cmd = Command::new("python");
    cmd.arg("test.py");

    // Execute the command
    match cmd.output() {
        Ok(o) => {
            println!("Output: {}", String::from_utf8_lossy(&o.stdout));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
