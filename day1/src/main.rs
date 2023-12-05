use std::fs::read_to_string;

fn main() {
    let input = read_from_file();
    let mut sum = 0;

    for i in input {
        sum += process(&i);
    }

    print!("Sum: {}", sum);
}

        //vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]

fn read_from_file() -> Vec<String> {
    let mut res = Vec::new();

    for line in read_to_string("../input.txt").unwrap().lines().map(String::from) {
        res.push(line);
    }

    res
}

fn process(line: &str) -> i32 {
    let digits: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    let combined = format!("{}{}", first, last);

    combined.parse::<i32>().unwrap()
}

