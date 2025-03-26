pub struct LazySeg<NT, LT, OpNN, OpNL, OpLL, OpLZ>
where
    NT: Clone + Copy,
    LT: Clone + Copy,
    OpNN: Fn(NT, NT) -> NT,
    OpNL: Fn(NT, LT) -> NT,
    OpLL: Fn(LT, LT) -> LT,
    OpLZ: Fn(LT, usize) -> LT,
{
    data: Vec<NT>,
    lazy: Vec<LT>,
    node_identity: NT,
    lazy_identity: LT,
    merge_node: OpNN,
    apply_lazy: OpNL,
    merge_lazy: OpLL,
    scala_mult: OpLZ,
}
impl<NT, LT, OpNN, OpNL, OpLL, OpLZ> LazySeg<NT, LT, OpNN, OpNL, OpLL, OpLZ>
where
    NT: Clone + Copy,
    LT: Clone + Copy + PartialEq,
    OpNN: Fn(NT, NT) -> NT,
    OpNL: Fn(NT, LT) -> NT,
    OpLL: Fn(LT, LT) -> LT,
    OpLZ: Fn(LT, usize) -> LT,
{
    pub fn new(
        size: usize,
        node_identity: NT,
        lazy_identity: LT,
        merge_node: OpNN,
        apply_lazy: OpNL,
        merge_lazy: OpLL,
        scala_mult: OpLZ,
    ) -> Self {
        let n = size.next_power_of_two();
        Self {
            data: vec![node_identity; n << 1],
            lazy: vec![lazy_identity; n << 1],
            node_identity: node_identity,
            lazy_identity: lazy_identity,
            merge_node: merge_node,
            apply_lazy: apply_lazy,
            merge_lazy: merge_lazy,
            scala_mult: scala_mult,
        }
    }

    pub fn build(&mut self, values: &Vec<NT>) {
        self._build_rec(values, 1, 0, values.len() - 1);
    }

    fn _build_rec(&mut self, values: &Vec<NT>, idx: usize, st: usize, en: usize) {
        if st == en {
            self.data[idx] = values[st];
        } else {
            self._build_rec(values, idx << 1, st, (st + en) >> 1);
            self._build_rec(values, idx << 1 | 1, ((st + en) >> 1) + 1, en);
            self.data[idx] = (self.merge_node)(self.data[idx << 1], self.data[idx << 1 | 1])
        }
    }

    fn prop(&mut self, idx: usize, st: usize, en: usize) {
        if self.lazy[idx] != self.lazy_identity {
            if st != en {
                let left = idx << 1;
                let rght = (idx << 1) | 1;
                self.lazy[left] = (self.merge_lazy)(self.lazy[left], self.lazy[idx]);
                self.lazy[rght] = (self.merge_lazy)(self.lazy[rght], self.lazy[idx]);
            }
            self.data[idx] = (self.apply_lazy)(
                self.data[idx],
                (self.scala_mult)(self.lazy[idx], en - st + 1),
            );
            self.lazy[idx] = self.lazy_identity;
        }
    }

    pub fn update(&mut self, idx: usize, l: usize, r: usize, st: usize, en: usize, val: LT) {
        self.prop(idx, st, en);
        let left = idx << 1;
        let rght = (idx << 1) | 1;
        if r < st || en < l {
            return;
        }
        if l <= st && en <= r {
            self.data[idx] = (self.apply_lazy)(self.data[idx], (self.scala_mult)(val, en - st + 1));
            if st != en {
                self.lazy[left] = (self.merge_lazy)(self.lazy[left], val);
                self.lazy[rght] = (self.merge_lazy)(self.lazy[rght], val);
            }
            return;
        }

        let mid = (st + en) >> 1;
        self.update(left, l, r, st, mid, val);
        self.update(rght, l, r, mid + 1, en, val);
        self.data[idx] = (self.merge_node)(self.data[idx << 1], self.data[idx << 1 | 1]);
    }

    pub fn query(&mut self, idx: usize, l: usize, r: usize, st: usize, en: usize) -> NT {
        self.prop(idx, st, en);
        if r < st || en < l {
            self.node_identity
        } else if l <= st && en <= r {
            self.data[idx]
        } else {
            let mid = (st + en) >> 1;
            let left_query = self.query(idx << 1, l, r, st, mid);
            let right_query = self.query(idx << 1 | 1, l, r, mid + 1, en);
            (self.merge_node)(left_query, right_query)
        }
    }
}
