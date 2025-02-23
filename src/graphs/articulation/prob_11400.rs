use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn find_min_reachable_rank(
    node: usize,
    parent: usize,
    graph: &Vec<Vec<usize>>,
    dfs_rank: &mut Vec<i32>,
    curr_cnt: &mut i32,
    bridges: &mut Vec<(usize, usize)>,
) -> i32 {
    *curr_cnt += 1;
    dfs_rank[node] = *curr_cnt;
    let mut res = dfs_rank[node];

    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }
        if dfs_rank[child] == 0 {
            let min_reacahble_child_rank =
                find_min_reachable_rank(child, node, graph, dfs_rank, curr_cnt, bridges);
            res = res.min(min_reacahble_child_rank);
            if min_reacahble_child_rank > dfs_rank[node] {
                bridges.push((node, child));
            }
        } else {
            res = res.min(dfs_rank[child]);
        }
    }
    res
}

fn prob_11400() {
    let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let mut ve = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let v = ve.next().unwrap();
    let e = ve.next().unwrap();

    let mut graph = vec![Vec::<usize>::new(); v];
    let mut bridges = Vec::<(usize, usize)>::new();
    let mut dfs_rank = vec![0; v];
    let mut curr_cnt = 0;

    for _ in 0..e {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let mut uv = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let u = uv.next().unwrap() - 1;
        let v = uv.next().unwrap() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }
    for i in 0..v {
        if dfs_rank[i] == 0 {
            find_min_reachable_rank(
                i,
                usize::MAX,
                &graph,
                &mut dfs_rank,
                &mut curr_cnt,
                &mut bridges,
            );
        }
    }
    
    bridges = bridges
        .iter()
        .map(|(a, b)| (*a.min(b), *a.max(b)))
        .collect();
    bridges.sort();
    writeln!(writer, "{}", bridges.len()).unwrap();
    for (a, b) in bridges {
        writeln!(writer, "{} {}", a + 1, b + 1).unwrap();
    }
}
pub fn main() {
    prob_11400();
}
