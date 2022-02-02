#![allow(unused_variables)]

mod fig38;

use fig38::{close, open, File};

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//         }
//     }
//     fn read(self: &mut File, save_to: &mut Vec<u8>) -> usize {
//         let mut tmp = self.data.clone();
//         let read_len = tmp.len();

//         // @note Vec method reserve
//         save_to.reserve(read_len); //ensures that there is sufficient space to fit the incoming data.
//         save_to.append(&mut tmp); // allocates sufficient data in the save_to buffer to hold the contents of f.

//         read_len
//     }
// }

fn main() {
    // struct literal creation.
    let f1_data: Vec<u8> = vec![118, 113, 24, 254];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);

    let mut b: Vec<u8> = vec![];

    f1 = open(f1).unwrap();

    let f1_len = f1.read(&mut b).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&b);

    println!("{f1:?}");
    println!("{} is {} bytes long", f1.name, f1_len);
}
