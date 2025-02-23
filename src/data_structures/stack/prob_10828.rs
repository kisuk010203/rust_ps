use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn prob_10828() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();
    let mut stack: Vec<i32> = Vec::new();
    reader.read_line(&mut input_line).unwrap();
    let queries = input_line.trim().parse::<i32>().unwrap();
    for _ in 0..queries {
        input_line.clear();
        reader.read_line(&mut input_line).unwrap();
        let parsed_line: Vec<&str> = input_line.split_whitespace().collect();
        match parsed_line[0] {
            "push" => {
                let num = parsed_line[1].parse::<i32>().unwrap();
                stack.push(num);
            }
            "pop" => match stack.pop() {
                Some(num) => writeln!(writer, "{}", num).unwrap(),
                None => writeln!(writer, "-1").unwrap(),
            },
            "size" => {
                writeln!(writer, "{}", stack.len()).unwrap();
            }
            "empty" => {
                writeln!(writer, "{}", stack.is_empty() as i32).unwrap();
            }
            "top" => match stack.last() {
                Some(num) => writeln!(writer, "{}", num).unwrap(),
                None => writeln!(writer, "-1").unwrap(),
            },
            _ => {
                panic!("Invalid query");
            }
        }
    }
}

pub fn main() {
    prob_10828();
}
