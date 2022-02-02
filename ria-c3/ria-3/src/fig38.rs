#[deny(unused)]
use rand::prelude::*;

/// FileState -
/// an enum representing the files state.
///
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name:  String,
    pub data:  Vec<u8>,
    pub state: FileState,
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name:  String::from(name),
            data:  Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(
        self: &File,
        save_dir: &mut Vec<u8>,
    ) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from(
                "file must be open before reading",
            ));
        }

        let mut temp = self.data.clone();
        let read_len = temp.len();
        save_dir.reserve(read_len);
        save_dir.append(&mut temp);
        Ok(read_len)
    }
}
///open - opens the file.  Returns a Result <File, String>
///
/// @param `f` - a File struct that holds the name and data of the file.
pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;

    if one_in(10_000) {
        let err = String::from("Permission Denied");
        return Err(err);
    }
    Ok(f)
}

/// close - closes the file reader, throws an error once in 10_000 runs.
pub fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

// @dev trigger sporadic errors.
// @info thread_rng creates a thread local random number generator.
fn one_in(denom: u32) -> bool {
    thread_rng().gen_ratio(1, denom)
}
