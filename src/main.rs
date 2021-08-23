mod linux;
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

// RemoveNewLines removes possible newlines from a string.
fn RemoveNewlines(s: String) -> String {
        let re = s.replace("\n", " ");
        return re.to_string()
        }

// FullRemove removes all instances of a string from another string.
fn FullRemove(str: String, to_remove: String) -> String {
        return str.replace(&to_remove, "" )
	}



fn main() {
    
}
