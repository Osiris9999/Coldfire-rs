mod linux;
use rand::Rng;

fn Revert(s: String) -> String {
	let reverted_string = s.chars().rev().collect::<String>();	
	return reverted_string 
	}

fn FileToSlice(file: String) {
	
	}

fn StrToInt(string_integer: String) -> u32 {
	return string_integer.parse::<u32>().unwrap()
	}

fn IntToStr(i: u32) -> String {
	return i.to_string()
	}

fn RandomInt(min: u32, max: u32) -> u32 {
	let rng = rand::thread_rng().gen_range(min..max);
	return rng
	}

fn RemoveNewlines(s: String) -> String {
        let re = s.replace("\n", " ");
        return re.to_string()
        }



fn main() {
    println!("Hello, world!");
}

