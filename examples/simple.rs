use mapped_fileio::{MappedFile, OpenOptions};
use std::{fs, io::Read};

fn main() {
    fs::write("text.txt", "Hello, World!").unwrap();

    {
        let mut file = MappedFile::open("text.txt", OpenOptions::ReadOnly).unwrap();
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        println!("{text}");
    }

    fs::remove_file("text.txt").unwrap();
}
