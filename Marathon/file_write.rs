fn file_write(file_path: String, contents: String) {
    use std::fs;
    use std::io::Write;

    let mut f = fs::File::create(file_path).unwrap();
    f.write_all(contents.as_bytes()).unwrap();
}
