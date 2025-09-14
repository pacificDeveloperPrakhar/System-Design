use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main()  {
    let file = File::open("../inputs.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    
    let inputs: Vec<String> = contents
    .split('\n')
    .map(|s| s.trim().to_string())
    .collect();

    
}