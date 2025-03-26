use std::env;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use rust_ps_lib::models::io::parse_into_vector;
use rust_ps_lib::models::lazyseg::LazySeg;

const MOD: i64 = 1000000007;

#[derive(Copy, Clone, PartialEq)]
struct Pair {
    a: i64,
    b: i64,
}
impl Pair {
    fn merge_pair(one: Pair, other: Pair) -> Pair {
        Pair {
            a: (one.a * other.a) % MOD,
            b: ((one.b * other.a) + other.b) % MOD,
        }
    }
    fn apply_to_i64(one: i64, other: Pair) -> i64 {
        (one * other.a + other.b) % MOD
    }
    fn scala_mult(lhs: Pair, rhs: usize) -> Pair {
        Pair {
            a: lhs.a,
            b: (lhs.b * rhs as i64) % MOD,
        }
    }
    fn identity() -> Pair {
        Pair { a: 1, b: 0 }
    }
}

fn prob_13925() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();

    let n = buf.trim().parse().unwrap();

    let mut tree = LazySeg::new(
        n,
        0,
        Pair::identity(),
        |a, b| (a + b) % MOD,
        Pair::apply_to_i64,
        Pair::merge_pair,
        Pair::scala_mult,
    );

    buf.clear();
    reader.read_line(&mut buf).unwrap();
    let input_vec = parse_into_vector(&buf, str::parse);
    tree.build(&input_vec);

    buf.clear();
    reader.read_line(&mut buf).unwrap();
    let m = buf.trim().parse().unwrap();
    for _ in 0..m {
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
                Pair {
                    a: 1,
                    b: query_props[3],
                },
            ),
            2 => tree.update(
                1,
                (query_props[1] - 1) as usize,
                (query_props[2] - 1) as usize,
                0,
                n - 1,
                Pair {
                    a: query_props[3],
                    b: 0,
                },
            ),
            3 => tree.update(
                1,
                (query_props[1] - 1) as usize,
                (query_props[2] - 1) as usize,
                0,
                n - 1,
                Pair {
                    a: 0,
                    b: query_props[3],
                },
            ),
            4 => {
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
    prob_13925();
}
