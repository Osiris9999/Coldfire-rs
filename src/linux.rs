use std::process::{Command, Output, ExitStatus};
use std::io::Error;
use nix::unistd::getuid;

fn killProcByPID(pid:i32) {
	let p = pid.to_string();
	let cmd = "kill -9".to_owned() + &p;
	let err = cmdOut(cmd);	
}

//fn info() -> String {
//	let user = Command::new("whoami").output().expect("Failed to execute command");
//	let user = match user {
//		Ok(Result) => output,
//		Err(error) => println!("{}",error),
//	}; 

//}

fn isRoot() -> String  {
	 let u = getuid();
         if u32::from(u) == 0 {
                 "root".to_string()
                }
        else {
                 "not root".to_string()
                }

	

	}

fn cmdOut(command: String)  -> ExitStatus {
	let output = Command::new("bash").arg("-c").arg(command).status().expect("Failed to execute command");
	 output
	}


fn main() {}
