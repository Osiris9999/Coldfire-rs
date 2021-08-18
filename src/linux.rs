use std::process::{Command,Output};
use std::io::Error;
fn killProcByPID(pid:i32) {
	let p = pid.to_string();
	let cmd = "kill -9".to_owned() + &p;
	
}

//fn info() -> String {
//	let user = Command::new("whoami").output().expect("Failed to execute command");
//	let user = match user {
//		Ok(Result) => output,
//		Err(error) => println!("{}",error),
//	}; 

//}

fn cmdOut(command: String)  -> (String, u32) {
	let cmd = Command::new("bash").arg("-c").arg(command).output();
	if cmd.status != 0 {
		println!("Process exited with exit status {}", cmd.status);
		} 

//	let output = match cmd {
//		Ok(T) => cmd,
//		Err(error) => println!("Command exited with exitstatus {}", cmd.status)
//		};	
	}


fn main() {}
