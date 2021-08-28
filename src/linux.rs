use std::process::Command;

fn isRoot() -> bool {
    let (u, err) = cmdOut("whoami");
    if u == "root" {
        true
    } else {
        false
    }
}

fn cmdOut(command: &str) -> (String, String) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    let out = String::from_utf8_lossy(&output.stdout).replace("\n", "");
    let err = String::from_utf8_lossy(&output.stderr).replace("\n", "");
    (out, err)
}

fn main() {}
