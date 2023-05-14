use mapped_fileio::{MappedFile, OpenOptions};
use std::{
    fs,
    io::{Read, Write},
};

fn main() {
    // Create an empty file
    fs::write("text.txt", "Hello, World!").unwrap();

    {
        let mut file = MappedFile::open("text.txt", OpenOptions::ReadWrite).unwrap();

        file.write_all(String::from("More stuff :D").as_bytes())
            .unwrap();
    }

    {
        let mut file = MappedFile::open("text.txt", OpenOptions::ReadWrite).unwrap();
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        println!("{text}");
    }

    fs::remove_file("text.txt").unwrap();
}
