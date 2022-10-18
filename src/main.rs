use std::{fs::OpenOptions, io::Write, os::unix::fs::FileExt};
use rand::Rng;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .open("tests/test.png")
        .unwrap();
    let file_length = file.metadata().unwrap().len();
    let rounds: u64 = 100;
    let mut rng = rand::thread_rng();

    // Can corrupt header of file
    for _ in 0..rounds {
        file.write_at(&[rand::random::<u8>()], rng.gen_range(0..file_length)).unwrap();
    }
}
