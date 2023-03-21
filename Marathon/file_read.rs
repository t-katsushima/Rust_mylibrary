#[allow(dead_code)]
fn read_file(file_path: String) -> Input {
    let contents = std::fs::read_to_string(&file_path).unwrap();

    // TODO: contents をパースして、入力を作れ
    let v = contents.split("\n").collect::<Vec<_>>();

    // let p = v[i]
    //         .split(" ")
    //         .collect::<Vec<_>>()
    //         .iter()
    //         .map(|e| e.parse::<usize>().unwrap())
    //         .collect::<Vec<_>>();
}
