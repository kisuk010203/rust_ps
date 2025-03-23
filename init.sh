echo "use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};"
echo "fn prob_$1() {"
echo "let (mut reader, mut writer) = (
        BufReader::new(stdin().lock()),
        BufWriter::new(stdout().lock()),
    );
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    buf.clear();"
echo "}"
echo "fn main() {prob_$1();}"