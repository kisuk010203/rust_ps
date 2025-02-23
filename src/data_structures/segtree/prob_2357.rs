use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
#[derive(Clone)]
struct MinMax {
    min: i64,
    max: i64,
}
impl MinMax {
    fn get_min_max(a: &MinMax, b: &MinMax) -> Self {
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
struct SegTree {
    tree: Vec<MinMax>,
}
impl SegTree {
    fn init(input: Vec<i64>) -> Self {
        let pow = input.len().next_power_of_two();
        let mut tree = vec![MinMax::new(); pow << 1];
        for (idx, num) in input.into_iter().enumerate() {
            tree[idx + pow] = MinMax::new_with_values(num, num);
        }
        for idx in (1..pow).rev() {
            tree[idx] = MinMax::get_min_max(&tree[idx << 1], &tree[(idx << 1) + 1]);
        }
        Self { tree }
    }
    fn query(&self, left: usize, right: usize) -> MinMax {
        let mut res = MinMax::new();
        let mut left = left + (self.tree.len() >> 1);
        let mut right = right + (self.tree.len() >> 1);
        while left <= right {
            if left & 1 == 1 {
                res = MinMax::get_min_max(&res, &self.tree[left]);
                left += 1;
            }
            if right & 1 == 0 {
                res = MinMax::get_min_max(&res, &self.tree[right]);
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        res
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
        input.push(buf.trim().parse::<i64>().unwrap());
    }
    let seg_tree = SegTree::init(input);
    for _ in 0..m {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let mut lr = buf
            .trim()
            .split_ascii_whitespace()
            .flat_map(str::parse::<usize>);
        let left = lr.next().unwrap() - 1;
        let right = lr.next().unwrap() - 1;
        let res = seg_tree.query(left, right);
        writeln!(writer, "{} {}", res.min, res.max).unwrap();
    }
}
pub fn main() {
    prob_2357();
}
