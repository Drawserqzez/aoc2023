use std::fs::read_to_string;

fn main() {
    let input = read_from_file();
    let mut sum = 0;

    for i in input {
        sum += process_part_one(&i);
    }

    print!("Sum: {}", sum);
}

fn read_from_file() -> Vec<String> {
    let mut res = Vec::new();

    for line in read_to_string("../input.txt").unwrap().lines().map(String::from) {
        res.push(line);
    }

    res
}

fn is_valid_number(input: &str) -> bool {
    match input {
        "one" | "1" => true,
        "two" | "2" => true,
        "three" | "3" => true,
        "four" | "4" => true,
        "five" | "5" => true,
        "six" | "6" => true,
        "seven" | "7" => true,
        "eight" | "8" => true,
        "nine" | "9" => true,
        _ => false
    }
}

fn get_string_number(s: &str) -> Option<i32> {
    match s.to_lowercase().as_str() {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None
    }
}

fn process_part_two(line: &str) -> i32 {

}

fn process_part_one(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    let combined = format!("{}{}", first, last);

    combined.parse::<i32>().unwrap()
}

