use mapped_fileio::{MappedFile, OpenOptions};
use std::fs;

fn main() {
    fs::write("text.txt", "Hello, World!").unwrap();

    {
        let mut file = MappedFile::open("text.txt", OpenOptions::ReadOnly).unwrap();

        let mut ch = file.next();
        while let Some(character) = ch {
            let character = char::from_u32(character as u32).unwrap();
            println!("{character}");
            ch = file.next();
        }
    }

    fs::remove_file("text.txt").unwrap();
}
