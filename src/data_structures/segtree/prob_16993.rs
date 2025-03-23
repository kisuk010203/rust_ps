use std::cmp::max;
use std::i32;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

use rust_ps_lib::models::io::parse_into_vector;
use rust_ps_lib::models::segtree::SegTree;

#[derive(Copy, Clone)]
struct Node {
    sum: i32,
    left_sum: i32,
    right_sum: i32,
    max_sum: i32,
}
impl Node {
    fn new(val: i32) -> Node {
        Node {
            sum: val,
            left_sum: val,
            right_sum: val,
            max_sum: val,
        }
    }
    fn identity() -> Node {
        Node {
            sum: 0,
            left_sum: i32::MIN,
            right_sum: i32::MIN,
            max_sum: i32::MIN,
        }
    }
    fn merge(a: Node, b: Node) -> Node {
        Node {
            sum: a.sum.saturating_add(b.sum),
            left_sum: max(a.left_sum, a.sum.saturating_add(b.left_sum)),
            right_sum: max(b.right_sum, b.sum.saturating_add(a.right_sum)),
            max_sum: *([a.max_sum, b.max_sum, a.right_sum.saturating_add(b.left_sum)]
                .iter()
                .max()
                .unwrap()),
        }
    }
}

fn prob_16993() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().expect(&"N must be usize");
    buf.clear();

    reader.read_line(&mut buf).unwrap();
    let input: Vec<i32> = parse_into_vector(&mut buf, str::parse);
    let input_to_node: Vec<Node> = input.iter().map(|x| Node::new(*x)).collect();
    buf.clear();

    reader.read_line(&mut buf).unwrap();
    let queries: i32 = buf.trim().parse().expect(buf.as_str());
    buf.clear();

    let mut tree = SegTree::new(n, Node::identity(), Node::merge);
    tree.build(&input_to_node);

    for _ in 0..queries {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let query: Vec<usize> = parse_into_vector(&mut buf, str::parse);
        let (left, right) = (query[0] - 1, query[1] - 1);
        let query_result = tree.query(left, right);
        writeln!(writer, "{}", query_result.max_sum.to_string()).unwrap();
    }
}

pub fn main() {
    prob_16993();
}
