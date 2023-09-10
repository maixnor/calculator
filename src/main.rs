use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input_filename: Option<PathBuf>,
    #[arg(short, long, value_name = "FILE")]
    output_filename: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let input: Box<dyn BufRead> = if let Some(input_filename) = cli.input_filename {
        Box::new(BufReader::new(File::open(input_filename)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut output: Box<dyn Write> = if let Some(output_filename) = cli.output_filename {
        Box::new(File::create(output_filename)?)
    } else {
        Box::new(io::stdout())
    };

    // Process input line by line and write the result to output
    Ok(for line in input.lines() {
        let line = line?;
        let processed_line = match process_line(&line) {
            Ok(res) => res,
            Err(_) => format!("your input: '{:?}' was not okay", line),
        };
        writeln!(output, "{}", processed_line)?;
    })
}

fn process_line(line: &str) -> Result<String, ParseOperatorError> {
    let (first, operator, second) = split_basic(&line);
    let result = match operator {
        Operator::Plus => first + second,
        Operator::Minus => first - second,
        Operator::Times => first * second,
        Operator::DividedBy => first / second,
    };

    Ok(result.to_string())
}

/// expression may not have a newline "\n" at the end
fn split_basic(expression: &str) -> (i32, Operator, i32) {
    let input: Vec<&str> = expression.split(" ").collect();
    let first = input.get(0).unwrap().parse::<i32>().unwrap();
    let operator = input.get(1).unwrap().parse::<Operator>().unwrap();
    let second = input.get(2).unwrap().parse::<i32>().unwrap();
    (first, operator, second)
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Plus,
    Minus,
    Times,
    DividedBy,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperatorError;

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            "-" => Ok(Operator::Minus),
            "*" => Ok(Operator::Times),
            "/" => Ok(Operator::DividedBy),
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

#[test]
fn plus() {
    let input = "4 + 3";
    let (first, operator, second) = split_basic(input);
    let result = match operator {
        Operator::Plus => first + second,
        _ => todo!(),
    };

    assert_eq!(result, 7)
}

#[test]
fn parse_operator() {
    assert_eq!(Operator::Plus, "+".parse::<Operator>().unwrap());
    assert_eq!(Operator::Minus, "-".parse::<Operator>().unwrap());
    assert_eq!(Operator::Times, "*".parse::<Operator>().unwrap());
    assert_eq!(Operator::DividedBy, "/".parse::<Operator>().unwrap());
}
