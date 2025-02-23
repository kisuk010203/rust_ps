use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
struct SegTree {
    tree: Vec<i64>,
}
impl SegTree {
    fn init(input: Vec<i64>) -> Self {
        let pow = input.len().next_power_of_two();
        let mut tree = vec![0; pow << 1];
        for (idx, num) in input.into_iter().enumerate() {
            tree[idx + pow] = num;
        }
        for idx in (1..pow).rev() {
            tree[idx] = (tree[idx << 1] * tree[(idx << 1) + 1]) % 1000000007;
        }
        Self { tree }
    }
    fn update(&mut self, mut idx: usize, new_val: i64) {
        idx += self.tree.len() >> 1;
        self.tree[idx] = new_val;
        while idx > 0 {
            idx >>= 1;
            self.tree[idx] = (self.tree[idx << 1] * self.tree[(idx << 1) + 1]) % 1000000007;
        }
    }
    fn query(&self, mut left: usize, mut right: usize) -> i64 {
        let mut res = 1;
        left += self.tree.len() >> 1;
        right += self.tree.len() >> 1;
        while left <= right {
            if left & 1 == 1 {
                res = (res * self.tree[left]) % 1000000007;
                left += 1;
            }
            if right & 1 == 0 {
                res = (res * self.tree[right]) % 1000000007;
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}
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

    let mut segtree = SegTree::init(input);
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
