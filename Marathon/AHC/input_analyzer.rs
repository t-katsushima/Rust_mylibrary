const INPUT_NUM: usize = unimplemented!();

fn main() {
    let input_path = "/YOUR_INPUT_PATH".to_string();

    let mut inputs = vec![];
    for i in 0..INPUT_NUM {
        let file_path = format!("{}/{}.txt", input_path, i);
        inputs.push(read_file(file_path));
    }

    // inputs内の要素に対して処理を書く
}

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

    // contents をパースして、入力を作れ
}
