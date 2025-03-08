use rust_ps_lib::models::segtree::SegTree;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn prob_2042() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let nmk: Vec<i64> = buf
        .trim()
        .split_ascii_whitespace()
        .flat_map(str::parse::<i64>)
        .collect();
    let (n, m, k) = (nmk[0], nmk[1], nmk[2]);

    let mut input_vec: Vec<i64> = Vec::new();
    for _ in 0..n {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let num = buf.trim().parse::<i64>().unwrap();
        input_vec.push(num);
    }
    let mut segtree = SegTree::new(n as usize, 0, |a, b| a + b);
    segtree.build(&input_vec);
    //let mut segtree = SegTree::init(input_vec);
    for _ in 0..(m + k) {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let abc: Vec<i64> = buf
            .trim()
            .split_ascii_whitespace()
            .flat_map(str::parse::<i64>)
            .collect();
        let (a, b, c) = (abc[0], abc[1], abc[2]);
        match a {
            1 => segtree.update(b as usize - 1, c),
            2 => writeln!(writer, "{}", segtree.query(b as usize - 1, c as usize - 1)).unwrap(),
            _ => panic!("Invalid Query"),
        }
    }
}
pub fn main() {
    prob_2042();
}
