use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

type Point = (i32, i32);

fn prob_11561() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();

    reader.read_line(&mut input_line).unwrap();
    let n = input_line.trim().parse::<i32>().unwrap();
    let mut points: Vec<Point> = Vec::new();

    for _ in 0..n {
        input_line.clear();
        reader.read_line(&mut input_line).unwrap();
        let parsed_line: Vec<i32> = input_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let x = parsed_line[0];
        let y = parsed_line[1];
        points.push((x, y));
    }
    points.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    for point in points {
        writeln!(writer, "{} {}", point.0, point.1).unwrap();
    }
}
pub fn main() {
    prob_11561();
}
