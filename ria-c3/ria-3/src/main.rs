#![allow(unused_variables)]
mod fig38;
use fig38::{close, open, File};

fn main() {
    let f1_data: Vec<u8> = vec![118, 113, 24, 254];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);
    println!("{f1:?}");

    let mut b: Vec<u8> = vec![];

    f1 = open(f1).unwrap();

    println!(" current state{:?}", &f1.state);

    let f1_len = f1
        .read(&mut b)
        .expect("the length of the buffer to be read.");
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&b);

    println!("{f1:?}");
    println!("{} is {} bytes long", f1.name, f1_len);
}
