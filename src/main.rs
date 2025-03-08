use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use colored::Colorize;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Subcommand)]
enum Commands {
    Submit { path: PathBuf },
    Parse { url: String },
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

async fn scrape_website(url: &str) -> u32 {
    println!("Scraping from: {}", url.bright_yellow());
    let client = Client::new();
    let response = client.get(url).header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36").send().await.expect(&"Failed to scrape".bright_red());

    let body = response
        .text()
        .await
        .expect(&"Failed to get body".bright_red());
    let document = Html::parse_document(&body);
    let sample_inputs = Selector::parse(r#"pre.sampledata[id^="sample-input-"]"#)
        .expect(&"Parsing Sample Input Errors".bright_red());
    let sample_outputs = Selector::parse(r#"pre.sampledata[id^="sample-output-"]"#)
        .expect(&"Parsing Sample Output Errors".bright_red());

    let mut cnt = 0;
    for (i, (input_element, output_element)) in document
        .select(&sample_inputs)
        .zip(document.select(&sample_outputs))
        .enumerate()
    {
        let mut input_file = File::create(Path::new(format!("{}.in", i).as_str())).unwrap();
        let mut output_file = File::create(Path::new(format!("{}.out", i).as_str())).unwrap();

        writeln!(
            input_file,
            "{}",
            input_element.text().collect::<Vec<_>>()[0]
        )
        .unwrap();
        writeln!(
            output_file,
            "{}",
            output_element.text().collect::<Vec<_>>()[0]
        )
        .unwrap();
        cnt += 1;
    }
    cnt
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Submit { path } => {
            println!(
                "Writing file to submittable format : {}",
                path.to_str().unwrap().bold().cyan()
            );
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
            let mut output_vec = Vec::new();
            let mut output_file = File::create(Path::new("temp.rs")).unwrap();
            for line in lines {
                if line.starts_with("use rust_ps_lib::") {
                    let root = String::from("src/");
                    let mut deliminated_module_path: Vec<&str> = line
                        .strip_prefix("use rust_ps_lib::")
                        .unwrap()
                        .split("::")
                        .collect();
                    deliminated_module_path.pop();
                    let mut module_path_string = deliminated_module_path.join("/") + ".rs";
                    module_path_string = root + &module_path_string;
                    let module_path = Path::new(&module_path_string);
                    println!(
                        "Adding Module from Module path : {}",
                        module_path_string.bright_green()
                    );
                    let module_file = File::open(&module_path).unwrap();
                    let module_reader = BufReader::new(module_file);
                    let mut module_content: Vec<String> =
                        module_reader.lines().map(|l| l.unwrap()).collect();

                    output_vec.append(&mut module_content);
                } else {
                    output_vec.push(line);
                }
            }

            for line in output_vec {
                writeln!(output_file, "{line}").unwrap();
            }
        }
        Commands::Parse { url } => {
            let test_case_count = scrape_website(url).await;
            let problem_number: u32 =
                str::parse(url.split("/").collect::<Vec<_>>().last().unwrap()).unwrap();
            let output = std::process::Command::new("cargo")
                .args([
                    "run",
                    "--bin",
                    "test",
                    &format!("{}", test_case_count),
                    &format!("{}", problem_number),
                ])
                .output()
                .expect("Error while running test");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
