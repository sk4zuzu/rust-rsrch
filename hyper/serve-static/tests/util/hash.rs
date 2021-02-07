use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn from_file(file: &mut File) -> serve_static::Result<String> {
    let mut contents = vec![];
    file.seek(SeekFrom::Start(0))?;
    file.read_to_end(&mut contents)?;
    Ok(hex::encode(Sha1::digest(&contents)))
}

pub fn from_bytes(bytes: &[u8]) -> serve_static::Result<String> {
    Ok(hex::encode(Sha1::digest(bytes)))
}
