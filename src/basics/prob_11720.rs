use std::io::{self, BufRead, BufReader, BufWriter, Write};
fn prob_11720() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    input_line.clear();
    reader.read_line(&mut input_line).unwrap();
    let sum = input_line
        .trim()
        .chars()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap());
    writeln!(writer, "{}", sum).unwrap();
}
pub fn main() {
    prob_11720();
}
