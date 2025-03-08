use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn prob_10845() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();

    let mut queue: VecDeque<i32> = VecDeque::new();
    reader.read_line(&mut input_line).unwrap();
    let queries = input_line.trim().parse::<i32>().unwrap();

    for _ in 0..queries {
        input_line.clear();
        reader.read_line(&mut input_line).unwrap();
        let parsed_line: Vec<&str> = input_line.split_whitespace().collect();
        match parsed_line[0] {
            "push" => {
                let num = parsed_line[1].parse::<i32>().unwrap();
                queue.push_back(num);
            }
            "pop" => match queue.pop_front() {
                Some(num) => writeln!(writer, "{}", num).unwrap(),
                None => writeln!(writer, "-1").unwrap(),
            },
            "size" => writeln!(writer, "{}", queue.len()).unwrap(),
            "empty" => writeln!(writer, "{}", queue.is_empty() as i32).unwrap(),
            "front" => match queue.front() {
                Some(num) => writeln!(writer, "{}", num).unwrap(),
                None => writeln!(writer, "-1").unwrap(),
            },
            "back" => match queue.back() {
                Some(num) => writeln!(writer, "{}", num).unwrap(),
                None => writeln!(writer, "-1").unwrap(),
            },
            _ => panic!("Invalid query"),
        }
    }
}

pub fn main() {
    prob_10845();
}
