use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

#[derive(PartialEq, Eq)]
struct Number {
    number: i32,
}
impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .number
            .abs()
            .cmp(&self.number.abs())
            .then(other.number.cmp(&self.number))
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn prob_11286() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();

    reader.read_line(&mut buf).unwrap();
    let queries = buf.trim().parse::<i32>().unwrap();
    let mut heap: BinaryHeap<Number> = BinaryHeap::new();

    for _ in 0..queries {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<i32>().unwrap();
        match num {
            0 => match heap.pop() {
                Some(Number { number: top }) => writeln!(writer, "{}", top).unwrap(),
                None => writeln!(writer, "0").unwrap(),
            },
            _ => heap.push(Number { number: num }),
        }
    }
}
pub fn main() {
    prob_11286();
}
