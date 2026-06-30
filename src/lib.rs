use std::{fs::{self, File}, io::{Read, Write}};

pub fn get_file_content(path: &str) -> Vec<u8> {
    let mut file_buf = Vec::<u8>::new();
    let _ = (match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            panic!("{:?}", err)
        }
    })
    .read_to_end(&mut file_buf);

    file_buf
}
