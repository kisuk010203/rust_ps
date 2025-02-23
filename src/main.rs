use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::io::{BufRead, BufReader};

use clap::Parser;

#[allow(dead_code)]
fn max_of_input() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());

    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();

    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sum: i32 = numbers.iter().sum();

    writeln!(writer, "{}", sum).unwrap();
    writer.flush().unwrap();
}
#[derive(Parser, Debug)]
struct Cli {
    mode: String,
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();

    if args.mode == "sort" {
        println!("Sorting file : {:?}", &args.path);
        let file = File::open(&args.path).unwrap();
        let reader = BufReader::new(file);

        let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        lines.sort();

        let mut output_file = File::create(&args.path).unwrap();
        for line in lines {
            writeln!(output_file, "{}", line).unwrap();
        }
        println!("{:?} is sorted", &args.path);
    } else {
    }
}
