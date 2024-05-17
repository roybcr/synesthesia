use colored::Colorize;
use std::{env, fs::File, io::Read, path::PathBuf};

fn main() {
    let mut text = String::new();
    let mut file = get_file("pi.txt");

    file.read_to_string(&mut text).unwrap();

    let mut colored_str = String::new();
    for ch in text.chars() {
        match ch {
            '0' => colored_str.push_str(&"0".truecolor(0, 0, 0).to_string()),
            '1' => colored_str.push_str(&"1".truecolor(0, 0, 0).to_string()),
            '2' => colored_str.push_str(&"2".truecolor(255, 255, 255).to_string()),
            '3' => colored_str.push_str(&"3".truecolor(55, 179, 20).to_string()),
            '4' => colored_str.push_str(&"4".truecolor(179, 20, 32).to_string()),
            '5' => colored_str.push_str(&"5".truecolor(255, 196, 0).to_string()),
            '6' => colored_str.push_str(&"6".truecolor(255, 143, 0).to_string()),
            '7' => colored_str.push_str(&"7".truecolor(64, 45, 27).to_string()),
            '8' => colored_str.push_str(&"8".truecolor(85, 21, 152).to_string()),
            '9' => colored_str.push_str(&"9".truecolor(224, 114, 210).to_string()),
            _ => colored_str.push_str(&ch.to_string()),
        }
    }

    println!("{colored_str}");
}

pub fn get_file(file_name: &str) -> File {
    let binding = env::current_dir().unwrap();
    let curr_dir = binding.to_str().unwrap();
    let path_str = format!("{}/src/{}", curr_dir, file_name);
    let pathbuf = PathBuf::from(path_str);

    match File::open(pathbuf) {
        Ok(file) => file,
        Err(_) => panic!("Unable to open file: {}", file_name),
    }
}
