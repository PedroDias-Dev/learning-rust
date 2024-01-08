pub fn read_file(path: &str) {
    let contents = std::fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}