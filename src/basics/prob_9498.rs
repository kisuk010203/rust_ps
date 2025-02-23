use std::io::{self, BufRead, BufReader, BufWriter, Write};
fn prob_9498() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = BufReader::new(stdin.lock());
    let mut writer = BufWriter::new(stdout.lock());
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    let score: i32 = input_line.trim().parse().unwrap();
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    writeln!(writer, "{}", grade).unwrap();
}
pub fn main() {
    prob_9498();
}
