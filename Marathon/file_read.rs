#[allow(dead_code, unused)]
fn read_file(file_path: String) -> Input {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(file);
    // ここにファイル内容を書き込む
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);

    // TODO: contents をパースして、入力を作れ
    let v = contents.split("\n").collect::<Vec<_>>();

    // let p = v[i]
    //         .split(" ")
    //         .collect::<Vec<_>>()
    //         .iter()
    //         .map(|e| e.parse::<usize>().unwrap())
    //         .collect::<Vec<_>>();
}
