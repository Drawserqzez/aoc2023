use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let input = read_from_file();
    let mut sum = 0;
    let mut index = 0;

    for i in input {
        sum += process_part_two(&i, index);
        index += 1;
    }

    println!("Sum: {}", sum);
}

fn read_from_file() -> Vec<String> {
    let mut res = Vec::new();
    for line in read_to_string("../input.txt").unwrap().lines().map(String::from) {
        res.push(line);
    }

    res
}

fn get_string_numbers(line: &str) -> (&str, String) {
    let r = Regex::new(r"1|one|2|two|3|three|4|four|5|five|6|six|7|seven|8|eight|9|nine").unwrap();

    let first = r.find(line).unwrap().as_str();

    let last = backwards(line, &r).unwrap();

    (first, last)
}

fn backwards<'a>(line: &'a str, r: &'a Regex) -> Option<String> {
    let mut stringy = String::new();
    let chars: Vec<char> = line.chars().collect();

    for i in (0..chars.len()).rev() {
        let c = chars[i];
        stringy.insert_str(0, &c.to_string());

        if let Some(m) = r.find(&stringy.clone()) {
            return Some(m.as_str().to_owned());
        }
    }

    None
}

fn convert(s: &str) -> Option<&str> {
    match s.to_lowercase().as_str() {
        "one" | "1" => Some("1"),
        "two" | "2" => Some("2"),
        "three" | "3" => Some("3"),
        "four" | "4" => Some("4"),
        "five" | "5" => Some("5"),
        "six" | "6" => Some("6"),
        "seven" | "7" => Some("7"),
        "eight" | "8" => Some("8"),
        "nine" | "9" => Some("9"),
        _ => None
    }
}

fn process_part_two(line: &str, index: i32) -> i32 {
    let (f, l) = get_string_numbers(line);

    let first = convert(f).unwrap(); 
    let last = convert(&l).unwrap(); 

    println!("{}. {}, {}", index, first, last);

    let sum = format!("{}{}", first, last);

    println!("{}", sum);

    sum.parse::<i32>().unwrap()
}

fn process_part_one(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    let combined = format!("{}{}", first, last);

    combined.parse::<i32>().unwrap()
}

