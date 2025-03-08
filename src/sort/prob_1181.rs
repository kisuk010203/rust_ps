use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
fn prob_1181() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();

    reader.read_line(&mut input_line).unwrap();
    input_line.clear();
    reader.read_to_string(&mut input_line).unwrap();

    let mut words: Vec<&str> = input_line.trim().split_whitespace().collect();
    words.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(&b)));
    words.dedup();

    writeln!(writer, "{}", words.join("\n")).unwrap();
}
pub fn main() {
    prob_1181();
}
