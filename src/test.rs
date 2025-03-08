use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Cli {
    test_case_count: u32,
    problem_number: u32,
}

fn main() {
    const GREEN: &str = "\x1b[32m";
    const RED: &str = "\x1b[31m";
    const BOLD: &str = "\x1b[1m";
    const RESET: &str = "\x1b[0m";

    let args = Cli::parse();

    for i in 0..args.test_case_count {
        let input_file = format!("{}.in", i);
        let output_file = format!("{}.out", i);

        if !Path::new(&input_file).exists() || !Path::new(&output_file).exists() {
            println!(
                "Skipping test {}: missing input/output file",
                i.to_string().bright_red()
            );
            continue;
        }

        let expected_output =
            fs::read_to_string(&output_file).expect(&format!("Failed to read {}", output_file));

        let output = Command::new("cargo")
            .args(["run", "--bin", &format!("prob_{}", args.problem_number)])
            .stdin(fs::File::open(&input_file).expect("Failed to open input file"))
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute command");

        let actual_output = String::from_utf8_lossy(&output.stdout);

        if actual_output.trim() == expected_output.trim() {
            let succeed = format!("{}{}Test {}: ✅ Passed {}", GREEN, BOLD, i, RESET);
            println!("{}", succeed);
        } else {
            let failed = format!("{}{} Test {}: ❌ Failed {}", RED, BOLD, i, RESET);
            println!("{}", failed);
            println!("Expected:\n{}", expected_output);
            println!("Got:\n{}", actual_output);
        }
    }
}
