use std::{fs, io};
use std::path::PathBuf;


pub fn directory_size(path: PathBuf) -> io::Result<u64> {
    let mut total_size = 0;
    let mut dirs = vec![path];
    while let Some(dir) = dirs.pop() {
        //println!("{}", dir.display());
        let metadata = fs::symlink_metadata(&dir)?;
        if metadata.is_dir() {
            let read_dir = fs::read_dir(&dir)?;
            for dir_entry in read_dir {
                let entry = dir_entry?;
                dirs.push(entry.path());
            }
        }
        total_size += metadata.len();
    }

    Ok(total_size)
}

const UNIT_SIZE: u64 = 1024;

fn convert(size: u64) -> String {
    let size_units = vec!["B", "KB", "MB", "GB", "TB"];
    for index in (1..size_units.len()-1).rev(){
        let step: u64 = UNIT_SIZE.pow(index as u32);
        if size as u64 > step {
            return format!("{:.2}{}", (size as f64 / step as f64), size_units[index]);
        }
    }
    println!("{:?}", size_units);
    size.to_string()
}

#[test]
fn directory_size_test() {
    match directory_size(PathBuf::from("/Users/wangshuwei/Downloads")) {
        Ok(size) => println!("total size: {}", size),
        Err(err) => println!("{:?}", err),
    };
}