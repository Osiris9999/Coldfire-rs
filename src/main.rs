mod linux;

fn Revert(s: String) -> String {
	let reverted_string = s.chars().rev().collect::<String>();	
	return reverted_string 
	}

fn StrToInt(string_integer: String) -> u32 {
	return string_integer.parse::<u32>().unwrap()
	}

fn IntToStr(i: u32) -> String {
	return i.to_string()
	}

fn main() {
    println!("Hello, world!");
}
