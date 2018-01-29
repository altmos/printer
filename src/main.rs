#![feature(conservative_impl_trait)]
#![feature(crate_in_paths)]

use std::fs::File;

mod ascii;
mod io;

fn main() {
    let f = File::open(std::env::args().nth(1).unwrap()).unwrap();

    let foo = io::buf_read(f);

    let mut count = 0;
    for i in foo {
        count += 1;
    }

    println!("{:?}", count);
}
