use std::{
    io::{self},
    str::FromStr,
};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // remove \n at the end of the buffer
    buffer.pop();

    let (first, operator, second) = split_basic(&buffer);
    let result = match operator {
        Operator::Plus => first + second,
    };

    println!("{:?}", result);
    Ok(())
}

enum Operator {
    Plus,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperatorError;

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            _ => Err(ParseOperatorError),
        }
    }
}

#[test]
fn full_result() {
    let input = "4 + 3";
    let input: Vec<&str> = input.split(" ").collect();
    let first = input.get(0).unwrap();
    let second = input.get(2).unwrap();
    let result = first.parse::<i32>().unwrap() + second.parse::<i32>().unwrap();
    assert_eq!(result, 7)
}

fn split_basic(expression: &str) -> (i32, Operator, i32) {
    let input: Vec<&str> = expression.split(" ").collect();
    let first = input.get(0).unwrap().parse::<i32>().unwrap();
    let operator = input.get(1).unwrap().parse::<Operator>().unwrap();
    let second_str = input.get(2).unwrap();
    println!("{}", second_str);
    let second = second_str.parse::<i32>().unwrap();
    (first, operator, second)
}

#[test]
fn with_operator() {
    let input = "4 + 3";
    let (first, operator, second) = split_basic(input);
    let result = match operator {
        Operator::Plus => first + second,
    };

    assert_eq!(result, 7)
}
