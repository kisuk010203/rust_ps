use std::io::{self, BufWriter, Write};

fn prob_2557() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "Hello World!").unwrap();
    writer.flush().unwrap();
}

#[allow(dead_code)]
pub fn main() {
    prob_2557()
}
