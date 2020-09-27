mod token;
mod util;
use token::Token;
fn main() {
    let file_path = std::path::Path::new("/home/barry/Desktop/arrow/src/test_files/test.txt");
    let source = util::read_file(file_path).unwrap();
    for item in Token::tokenize(source) {
        print!("{:#?}", item);
    }
}
