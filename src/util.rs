use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

//read_file reads a file into a string
pub fn read_file(path: &Path) -> Result<String> {
    match File::open(path) {
        Ok(mut f) => {
            let mut source = String::new();
            match f.read_to_string(&mut source) {
                Ok(n) => {
                    println!("Bytes read {}", n);
                }
                Err(e) => {
                    return Err(e);
                }
            }
            Ok(source)
        }
        Err(e) => {
            return Err(e);
        }
    }
}
