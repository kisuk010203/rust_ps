use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

type User = (i32, String);

fn prob_10814() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    let n = input_line.trim().parse::<i32>().unwrap();
    let mut users: Vec<User> = Vec::new();
    for _ in 0..n {
        input_line.clear();
        reader.read_line(&mut input_line).unwrap();
        let parsed_line: Vec<&str> = input_line.split_whitespace().collect();
        let age = parsed_line[0].parse::<i32>().unwrap();
        let name = parsed_line[1].to_string();
        users.push((age, name));
    }
    users.sort_by(|a, b| a.0.cmp(&b.0));
    for user in users {
        writeln!(writer, "{} {}", user.0, user.1).unwrap();
    }
}
pub fn main() {
    prob_10814();
}
