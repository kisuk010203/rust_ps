use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};
fn prob_2751() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();

    reader.read_line(&mut input_line).unwrap();
    input_line.clear();

    reader.read_to_string(&mut input_line).unwrap();
    let mut numbers: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    numbers.sort();
    for n in numbers {
        writeln!(writer, "{}", n).unwrap();
    }
}
pub fn main() {
    prob_2751();
}
