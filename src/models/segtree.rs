pub struct SegTree<T, Op>
where
    T: Clone + Copy,
    Op: Fn(T, T) -> T,
{
    n: usize,
    data: Vec<T>,
    identity: T,
    merge: Op,
}
impl<T, Op> SegTree<T, Op>
where
    T: Clone + Copy,
    Op: Fn(T, T) -> T,
{
    pub fn new(size: usize, identity: T, merge: Op) -> Self {
        let n = size.next_power_of_two();
        Self {
            n,
            data: vec![identity; n << 1],
            identity,
            merge,
        }
    }

    pub fn build(&mut self, values: &Vec<T>) {
        for (idx, &val) in values.iter().enumerate() {
            self.data[idx + self.n] = val;
        }
        for idx in (1..self.n).rev() {
            self.data[idx] = (self.merge)(self.data[idx << 1], self.data[(idx << 1) + 1]);
        }
    }

    pub fn update(&mut self, mut pos: usize, val: T) {
        pos += self.n;
        self.data[pos] = val;
        while pos > 0 {
            pos >>= 1;
            self.data[pos] = (self.merge)(self.data[pos << 1], self.data[(pos << 1) + 1]);
        }
    }

    pub fn query(&self, mut left: usize, mut right: usize) -> T {
        let (mut resl, mut resr) = (self.identity, self.identity);
        left += self.n;
        right += self.n;

        while left <= right {
            if left & 1 == 1 {
                resl = (self.merge)(resl, self.data[left]);
                left += 1;
            }
            if right & 1 == 0 {
                resr = (self.merge)(self.data[right], resr);
                right -= 1;
            }
            left >>= 1;
            right >>= 1;
        }
        (self.merge)(resl, resr)
    }
}
