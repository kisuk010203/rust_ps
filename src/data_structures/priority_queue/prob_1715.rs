use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn prob_1715() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut answer: i64 = 0;

    reader.read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<i64>().unwrap();
    for _ in 0..n {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<i64>().unwrap();
        heap.push(Reverse(num));
    }
    while heap.len() > 1 {
        let Reverse(first) = heap.pop().unwrap();
        let Reverse(second) = heap.pop().unwrap();
        answer += first + second;
        heap.push(Reverse(first + second));
    }
    writeln!(writer, "{}", answer).unwrap();
}
pub fn main() {
    prob_1715();
}
