use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn prob_9012() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut reader, mut writer) = (BufReader::new(stdin.lock()), BufWriter::new(stdout.lock()));
    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    let queries = input_line.trim().parse::<i32>().unwrap();

    for _ in 0..queries {
        input_line.clear();
        reader.read_line(&mut input_line).unwrap();
        let curr_par = input_line.trim();
        let mut pointer = 0;
        for char in curr_par.chars() {
            match char {
                '(' => pointer += 1,
                ')' => pointer -= 1,
                _ => panic!("Invalid character"),
            }
            if pointer < 0 {
                break;
            }
        }
        writeln!(writer, "{}", if pointer == 0 { "YES" } else { "NO" }).unwrap();
    }
}

pub fn main() {
    prob_9012();
}
