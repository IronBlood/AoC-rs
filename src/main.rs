use regex::Regex;
use std::fs;
use std::path::Path;

mod lib {
    pub mod matrix;
}

mod y2017 {
    pub mod d01;
    pub mod d02;
    pub mod d03;
    pub mod d04;
    pub mod d05;
    pub mod d06;
    pub mod d07;
    pub mod d08;
    pub mod d09;
    pub mod d10;
    pub mod d11;
    pub mod d12;
    pub mod d13;
    pub mod d14;
    pub mod d15;
    pub mod d16;
    pub mod d17;
    pub mod d18;
    pub mod d19;
    pub mod d20;
    pub mod d21;
    pub mod d22;
    pub mod d23;
    pub mod d24;
    pub mod d25;
}

fn load_input(year: &str, day: &str) -> String {
    let path = format!("src/y{}/d{}/input.txt", year, day);
    fs::read_to_string(Path::new(&path))
        .expect("Failed to read input file")
        .trim_end()
        .to_string()
}

fn exec(year: &str, day: &str) {
    let input = load_input(year, day);
    match (year, day) {
        ("2017", "01") => y2017::d01::run(&input),
        ("2017", "02") => y2017::d02::run(&input),
        ("2017", "03") => y2017::d03::run(&input),
        ("2017", "04") => y2017::d04::run(&input),
        ("2017", "05") => y2017::d05::run(&input),
        ("2017", "06") => y2017::d06::run(&input),
        ("2017", "07") => y2017::d07::run(&input),
        ("2017", "08") => y2017::d08::run(&input),
        ("2017", "09") => y2017::d09::run(&input),
        ("2017", "10") => y2017::d10::run(&input),
        ("2017", "11") => y2017::d11::run(&input),
        ("2017", "12") => y2017::d12::run(&input),
        ("2017", "13") => y2017::d13::run(&input),
        ("2017", "14") => y2017::d14::run(&input),
        ("2017", "15") => y2017::d15::run(&input),
        ("2017", "16") => y2017::d16::run(&input),
        ("2017", "17") => y2017::d17::run(&input),
        ("2017", "18") => y2017::d18::run(&input),
        ("2017", "19") => y2017::d19::run(&input),
        ("2017", "20") => y2017::d20::run(&input),
        ("2017", "21") => y2017::d21::run(&input),
        ("2017", "22") => y2017::d22::run(&input),
        ("2017", "23") => y2017::d23::run(&input),
        ("2017", "24") => y2017::d24::run(&input),
        ("2017", "25") => y2017::d25::run(&input),
        _ => println!("Unimplemented"),
    }
}

fn scaffold(year: &str, day: &str) {
    let folder = format!("src/y{}/d{}", year, day);
    let file_path = format!("{}/mod.rs", folder);
    let path = Path::new(&file_path);

    if path.exists() {
        eprintln!("{} already exists, skipping.", file_path);
        return;
    }

    fs::create_dir_all(&folder).expect("Failed to create directories");
    let template = r##"pub fn run(input: &str) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        todo!();
    }
}
"##;
    fs::write(&file_path, template).expect("Failed to write mod.rs");
    println!("Scaffolded {}", file_path);
}

fn parse_date(arg: &str) -> Option<(String, String)> {
    let re = Regex::new(r"^(\d{4})-(\d{2})$").unwrap();
    re.captures(arg)
        .map(|caps| (caps[1].to_string(), caps[2].to_string()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} YYYY-DD [s]", args[0]);
        return;
    }

    let (year, day) = match parse_date(&args[1]) {
        Some(pair) => pair,
        None => {
            eprintln!("Invalid date format");
            return;
        }
    };

    if args.len() == 2 {
        exec(&year, &day);
    } else if args.len() == 3 && args[2] == "s" {
        scaffold(&year, &day);
    } else {
        eprintln!("Invalid argument pattern");
    }
}
