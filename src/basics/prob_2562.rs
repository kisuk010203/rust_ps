use std::io::{self, BufRead, BufReader, BufWriter, Write};
fn prob_2562() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();
    let mut max = -1;
    let mut max_index = -1;
    for i in 0..9 {
        input_line.clear();
        reader.read_line(&mut input_line).unwrap();
        let num = input_line.trim().parse::<i32>().unwrap();
        if num > max {
            max = num;
            max_index = i;
        }
    }
    writeln!(writer, "{}\n{}", max, max_index + 1).unwrap();
}
pub fn main() {
    prob_2562();
}
