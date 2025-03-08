use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn prob_1427() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    let mut chars: Vec<char> = input_line.trim().chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    writeln!(writer, "{}", chars.into_iter().collect::<String>()).unwrap();
}
pub fn main() {
    prob_1427();
}
