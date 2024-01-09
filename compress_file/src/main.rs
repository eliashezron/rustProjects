extern crate flate2;
use flate2::write::GzEncoder;
use flate2::Compression;

use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main(){
    if args().len() > 3{
        eprint!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());  // takes 1st input from args and opens it using BufReader 
    let output = File::create(args().nth(2).unwrap()).unwrap(); // reads the name of the output file from args and creates it
    let mut encoder = GzEncoder::new(output, Compression::default());   // creates a new encoder with default compression, the output file is the target file
    let start = Instant::now(); // starts the timer
    copy(&mut input, &mut encoder).unwrap(); // copies the input file to the encoder
    let output = encoder.finish().unwrap(); // finishes the encoder and returns the output file
    let duration = start.elapsed(); // stops the timer
    println!("source file size is: {:?}", input.get_ref().metadata().unwrap().len()); // prints the size of the input file
    println!("target file size is: {:?}", output.metadata().unwrap().len()); // prints the size of the output file
    println!("Time elapsed in compressing file is: {:?}", duration); // prints the time elapsed in compressing the file
}
