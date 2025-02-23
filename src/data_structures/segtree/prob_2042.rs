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
            tree[idx] = tree[idx << 1] + tree[(idx << 1) + 1];
        }
        Self { tree }
    }
    fn update(&mut self, mut idx: usize, new_val: i64) {
        idx += self.tree.len() >> 1;
        self.tree[idx] = new_val;
        while idx > 0 {
            idx >>= 1;
            self.tree[idx] = self.tree[idx << 1] + self.tree[(idx << 1) + 1];
        }
    }
    fn query(&mut self, mut left: usize, mut right: usize) -> i64 {
        let mut res: i64 = 0;
        left += self.tree.len() >> 1;
        right += self.tree.len() >> 1;

        while left <= right {
            if left & 1 == 1 {
                res += self.tree[left];
                left += 1;
            }
            if right & 1 == 0 {
                res += self.tree[right];
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}
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
    let mut segtree = SegTree::init(input_vec);
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
