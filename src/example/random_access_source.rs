use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, ErrorKind, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;

pub trait RandomAccessSource {
    /// get a u8 from specified position
    fn get(&self, position: u64) -> io::Result<u8>;

    /// Gets an array at the specified position.  If the number of bytes requested cannot be read, the bytes that can be
    /// read will be placed in bytes and the number actually read will be returned.
    fn get_by_bytes(&self, position: u64, bytes: &mut [u8]) -> io::Result<usize>;

    /// length of this source
    fn length(&self) -> u64;
}

pub struct FileRandomAccessSouce {
    file: File,
}

impl FileRandomAccessSouce {
    pub fn new(path: &str) -> Result<FileRandomAccessSouce, io::Error> {
        let file = OpenOptions::new().read(true).open(path)?;
        Ok(FileRandomAccessSouce {
            file
        })
    }

    pub fn read_to_vec(&mut self) -> io::Result<Vec<u8>> {
        let mut result: Vec<u8> = Vec::new();
        match self.file.read_to_end(&mut result) {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }
}

const BUFFER_SIZE: usize = 8192;

impl RandomAccessSource for FileRandomAccessSouce {
    fn get(&self, position: u64) -> io::Result<u8> {
        let mut buffer: [u8; 1] = [0];
        self.file.read_at(&mut buffer, position)?;
        Ok(buffer[0])
    }

    fn get_by_bytes(&self, position: u64, bytes: &mut [u8]) -> io::Result<usize> {
        self.file.read_at(bytes, position)
    }

    fn length(&self) -> u64 {
        match self.file.metadata() {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        }
    }
}

#[test]
fn file_random_access_test() {
    let file = FileRandomAccessSouce::new("hello.txt").unwrap_or_else(|error| {
        panic!("{}", error);
    });
    let byte = file.get(0).unwrap_or_else(|error| {
        panic!("{}", error);
    });
    let mut bytes: [u8; 1024] = [0; 1024];
    let readed = file.get_by_bytes(0, &mut bytes).unwrap_or_else(|error| {
        panic!("{}", error);
    });
    println!("{}", byte);
}

#[test]
fn file_random_read_all_test() {
    let mut file = FileRandomAccessSouce::new("/Users/wangshuwei/Downloads/docs/300m.pdf").unwrap_or_else(|error| {
        panic!("{}", error);
    });

    let bytes = file.read_to_vec().expect("file read error");
    let length = file.length();
    println!("{}-{}", bytes.len(), length);
}