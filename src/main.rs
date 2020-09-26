mod util;
fn main() {
    let file_path = std::path::Path::new("/home/barry/Desktop/arrow/src/test_files/test.txt");
    let source = util::read_file(file_path).unwrap();

    println!("{}", source);
}
