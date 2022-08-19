#[macro_use] extern crate text_io;

use std::io::Read;
use std::io::{self, BufRead};
mod token;

fn main() {
    // let input: String = read!("{}\n");
    // let linesplit = token::tokenize(input);
    // for cmd in linesplit.iter(){
    //     for token in cmd.iter(){
    //         println!("{:?}", token);
    //     }
    // }
    let input: String = read!("{}\n");
    let mut tokens = token::tokenize(input);
    remove_first(&mut tokens);
    // for token in tokens.iter(){
    //     println!("{:?}", token);
    // }
    for token in tokens.iter(){
        println!("{:?}",token::parser(token.to_vec()));
    }
}

fn remove_first<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }
    Some(vec.remove(0))
}