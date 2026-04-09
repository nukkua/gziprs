use std::{
    env,
    fs,
    io::{self, Read},
};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let buf: Vec<u8> = if args.len() > 1 {
        fs::read(&args[1]).expect("read file")
    } else {
        let mut b: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut b).expect("read stdin");
        b
    };
    assert!(buf[0] == 0x1F && buf[1] == 0x8B, "not a gzip file");
    let mut offset = 10;

    if buf[3] == 8 {
        offset += buf[10..].iter().position(|&b| b == 0).unwrap() + 1;
    } else if buf[3] != 0 {
        panic!("unsupported gzip format");
    }

    println!("{}", offset);
}
