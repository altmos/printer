#![feature(conservative_impl_trait)]
#![feature(crate_in_paths)]

use std::fs::File;

mod error;
mod io;
mod lexer;

fn main() {
    let f = File::open(std::env::args().nth(1).unwrap()).unwrap();

    let foo = io::buf_read(f);

    let foo = lexer::tokenize(foo);

    let mut count = 0;
    for _ in foo {
        //println!("{:?}", i);
        count += 1;
    }

    println!("{:?}", count);
}
