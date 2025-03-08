use std::collections::BinaryHeap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn prob_11279() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let queries = buf.trim().parse::<i32>().unwrap();

    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    for _ in 0..queries {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<i32>().unwrap();
        match num {
            0 => match heap.pop() {
                Some(element) => writeln!(writer, "{}", element).unwrap(),
                None => writeln!(writer, "0").unwrap(),
            },
            _ => heap.push(num),
        }
    }
}
pub fn main() {
    prob_11279();
}
