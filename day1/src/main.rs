use std::fs;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = fs::read_to_string("input")?;
    if env::args().nth(1) == Some("-2".to_string()) {
        input = input
            // standard cases
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
    }
    input.retain(|c| c.is_whitespace() || c.is_numeric());
    let mut sum: u64 = 0;

    for line in input.lines() {
        let mut digits = String::new();
        let mut chars = line.chars();
        digits.push(chars.next().unwrap());
        match chars.last() {
            Some(x) => {
                digits.push(x);
                sum += dbg!(digits.parse::<u64>().unwrap());
            },
            None => {
                sum += dbg!(digits.parse::<u64>().unwrap() * 11);
            },
        }
    }
    println!("{}", sum);
    Ok(())
}
