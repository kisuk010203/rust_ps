use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::io::{BufRead, BufReader};
use std::path::Path;

use clap::Parser;
use colored::Colorize;

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

    if args.mode == "submit" {
        println!("Writing file to submittable format : {}", &args.path.to_str().unwrap().bold().cyan());
        let file = File::open(&args.path).unwrap();
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut output_vec = Vec::new();
        let mut output_file = File::create(Path::new("temp.rs")).unwrap();
        for line in lines {
            if line.starts_with("use rust_ps_lib::") {
                let root = String::from("src/");
                let mut deliminated_module_path: Vec<&str> = line.strip_prefix("use rust_ps_lib::").unwrap().split("::").collect();
                deliminated_module_path.pop();
                let mut module_path_string =  deliminated_module_path.join("/") + ".rs";
                module_path_string = root + &module_path_string;
                let module_path = Path::new(&module_path_string);
                println!("Adding Module from Module path : {}", module_path_string.bright_green());
                let module_file = File::open(&module_path).unwrap();
                let module_reader = BufReader::new(module_file);
                let mut module_content: Vec<String> = module_reader.lines().map(|l| l.unwrap()).collect();

                output_vec.append(&mut module_content);
            } else {
                output_vec.push(line);
            }
        }
        
        for line in output_vec {
            writeln!(output_file, "{line}").unwrap();
        }
    } else {
    }
}
