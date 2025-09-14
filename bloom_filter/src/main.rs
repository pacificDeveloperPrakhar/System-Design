use std::fs::File;
use std::io::BufReader;
use std::io;
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

    use bloom_filter::bloom_filter::Bloom_filter;

    let mut bloom=Bloom_filter::new(1_000_000.0,0.01);


    // now insert every value into the bloom
    for i in inputs.iter()
    {
        bloom.add_new_value(i);
    };

    let mut buffer:String=String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("{}",bloom.check(&String::from("hello")));
}