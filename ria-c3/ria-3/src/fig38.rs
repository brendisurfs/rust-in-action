#[deny(unused)]
use rand::prelude::*;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
}

#[allow(dead_code)]
impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(self: &File, save_dir: &mut Vec<u8>) -> Result<usize, String> {
        let mut temp = self.data.clone();
        let read_len = temp.len();
        save_dir.reserve(read_len);
        save_dir.append(&mut temp);
        Ok(read_len)
    }
}
pub fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        // throws an artificial error once in 10000 times.
        let err = String::from("Permission Denied");
        return Err(err);
    }
    Ok(f)
}

// close - closes the file reader, throws an error once in 10_000 runs.
pub fn close(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err = String::from("signal interrupt");
        return Err(err);
    }
    Ok(f)
}

// @dev trigger sporadic errors.
// @info thread_rng creates a thread local random number generator.
fn one_in(denom: u32) -> bool {
    thread_rng().gen_ratio(1, denom)
}
