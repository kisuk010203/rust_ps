use std::io::{self, BufRead, BufReader, BufWriter, Write};
fn prob_1008() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    let numbers: Vec<f64> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let div = numbers[0] / numbers[1];
    writeln!(writer, "{:.9}", div).unwrap();
}
pub fn main() {
    prob_1008();
}
