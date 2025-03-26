use std::env;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use rust_ps_lib::models::io::parse_into_vector;
use rust_ps_lib::models::lazyseg::LazySeg;

fn prob_10999() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let nmk: Vec<usize> = parse_into_vector(&buf, str::parse);
    let (n, m, k) = (nmk[0], nmk[1], nmk[2]);

    let sum_fn = |a: i64, b: i64| a + b;
    let mut tree = LazySeg::new(n, 0, 0, sum_fn, sum_fn, sum_fn, |a, n| a * (n as i64));

    let mut input_vec = Vec::new();
    for _ in 0..n {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        input_vec.push(buf.trim().parse().unwrap());
    }

    tree.build(&input_vec);

    for _ in 0..(m + k) {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let query_props: Vec<i64> = parse_into_vector(&buf, str::parse);
        match query_props[0] {
            1 => tree.update(
                1,
                (query_props[1] - 1) as usize,
                (query_props[2] - 1) as usize,
                0,
                n - 1,
                query_props[3],
            ),
            2 => {
                let query_res = tree.query(
                    1,
                    (query_props[1] - 1) as usize,
                    (query_props[2] - 1) as usize,
                    0,
                    n - 1,
                );
                writeln!(writer, "{}", query_res).unwrap();
            }
            _ => panic!("Unexpected Query Given"),
        }
    }
    buf.clear();
}
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    prob_10999();
}
