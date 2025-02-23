use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn find_min_reachable_rank(
    node: usize,
    is_root: bool,
    graph: &Vec<Vec<i32>>,
    dfs_rank: &mut Vec<i32>,
    curr_cnt: &mut i32,
    is_articulation: &mut Vec<bool>,
) -> i32 {
    *curr_cnt += 1;
    dfs_rank[node] = *(curr_cnt);
    let mut res = dfs_rank[node];
    let mut child_cnt = 0;
    for &child in graph[node].iter() {
        if dfs_rank[child as usize] == 0 {
            child_cnt += 1;
            let min_reachable_child_rank = find_min_reachable_rank(
                child as usize,
                false,
                graph,
                dfs_rank,
                curr_cnt,
                is_articulation,
            );
            res = res.min(min_reachable_child_rank);
            if !is_root && min_reachable_child_rank >= dfs_rank[node] {
                is_articulation[node] = true;
            }
        } else {
            res = res.min(dfs_rank[child as usize]);
        }
    }
    if is_root && child_cnt >= 2 {
        is_articulation[node] = true;
    }
    res
}
fn prob_11266() {
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

    let mut graph = vec![Vec::<i32>::new(); v];
    let mut is_articulation = vec![false; v];
    let mut dfs_rank = vec![0; v];
    let mut curr_cnt = 0;

    for _ in 0..e {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let mut uv = buf
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let u = uv.next().unwrap();
        let v = uv.next().unwrap();
        graph[u - 1].push(v as i32 - 1);
        graph[v - 1].push(u as i32 - 1);
    }
    for i in 0..v {
        if dfs_rank[i] == 0 {
            find_min_reachable_rank(i, true, &graph, &mut dfs_rank, &mut curr_cnt, &mut is_articulation);
        }
    }
    let articulation_points: Vec<usize> = is_articulation.iter().enumerate().filter(|(_, &x)| x).map(|(idx, _)| idx + 1).collect();
    writeln!(writer, "{}", articulation_points.len()).unwrap();
    writeln!(writer, "{}", articulation_points.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
}
fn main() {
    prob_11266();
}
