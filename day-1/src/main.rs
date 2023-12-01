use std::fs::{File};
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let replacements = HashMap::from([
        ("one", "on1e"),
        ("two", "tw2o"),
        ("three", "th3e"),
        ("four", "fo4r"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "seve7n"),
        ("eight", "eigh8t"),
        ("nine", "n9e"),
    ]);

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut total = 0;
        for line_result in lines {
            if let Ok(mut line) = line_result {
                replacements.iter().for_each(|(key, value)| {
                    line = line.replace(key, value);
                });

                let first_number = line.chars()
                    .filter(|c| c.is_digit(10))
                    .next().unwrap();

                let last_number = line.chars().rev()
                    .filter(|c| c.is_digit(10))
                    .next()
                    .unwrap();

                let numbers = format!("{}{}", first_number.to_string(), last_number.to_string()).parse::<i32>().unwrap();
                total += numbers;
            }
        }
        println!("{}", total);
    } else {
        println!("Error reading file");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}