use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use rust_ps_lib::models::segtree::SegTree;

#[derive(Clone, Copy)]
struct MinMax {
    min: i64,
    max: i64,
}
impl MinMax {
    fn get_min_max(a: MinMax, b: MinMax) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }
    fn new() -> Self {
        Self {
            min: i64::MAX,
            max: i64::MIN,
        }
    }
    fn new_with_values(a: i64, b: i64) -> Self {
        Self { min: a, max: b }
    }
}
fn prob_2357() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let mut nm = buf
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse::<i64>);
    let n = nm.next().unwrap();
    let m = nm.next().unwrap();

    let mut input = Vec::new();
    for _ in 0..n {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let val = buf.trim().parse::<i64>().unwrap();
        input.push(MinMax::new_with_values(val, val));
    }
    let identity = MinMax::new();
    let mut segtree = SegTree::new(n as usize, identity, MinMax::get_min_max);
    segtree.build(&input);
    for _ in 0..m {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let mut lr = buf
            .trim()
            .split_ascii_whitespace()
            .flat_map(str::parse::<usize>);
        let left = lr.next().unwrap() - 1;
        let right = lr.next().unwrap() - 1;
        let res = segtree.query(left, right);
        writeln!(writer, "{} {}", res.min, res.max).unwrap();
    }
}
pub fn main() {
    prob_2357();
}
