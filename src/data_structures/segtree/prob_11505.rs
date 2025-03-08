use rust_ps_lib::models::segtree::SegTree;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn prob_11505() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let nmk: Vec<i64> = buf
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();
    let (n, m, k) = (nmk[0], nmk[1], nmk[2]);
    let mut input: Vec<i64> = Vec::new();
    for _ in 0..n {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        input.push(buf.trim().parse().unwrap());
    }

    let mut segtree = SegTree::new(n as usize, 1, |a, b| a * b % 1000000007);
    segtree.build(&input);
    for _ in 0..(m + k) {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let query: Vec<i64> = buf
            .trim()
            .split_ascii_whitespace()
            .flat_map(str::parse)
            .collect();
        let (a, b, c) = (query[0], query[1], query[2]);
        match a {
            1 => segtree.update(b as usize - 1, c),
            2 => writeln!(writer, "{}", segtree.query(b as usize - 1, c as usize - 1)).unwrap(),
            _ => panic!("Invalid query"),
        }
    }
}
pub fn main() {
    prob_11505();
}
