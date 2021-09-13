use std::convert::TryInto;
use std::fs::read_dir;
use std::process::Command;

fn killProcByPID(pid: u32) -> String {
    let p = pid.to_string();
    let cmd = "kill -9 ".to_owned() + &p;
    let (_, err) = cmdOut(&cmd);
    err
}

fn info() -> String {
    match Command::new("whoami").output() {
        Ok(output) => return String::from_utf8_lossy(&output.stdout).replace("\n", ""),
        Err(err) => "N/A".to_string(),
    }
}

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

fn sandboxFilepath() -> bool {
    let (out, err) = cmdOut("systemd-detect-virt");
    out != "none"
}

fn sandboxTmp(entries: u32) -> bool {
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
    let (out, err) = cmdOut(c);
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
