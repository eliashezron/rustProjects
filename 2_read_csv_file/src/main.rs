extern crate csv;
use std::error::Error;
fn main() {
    if let Err(e) = readcsv("./customers.csv"){ // if readcsv returns an error, print it and exit
        eprintln!("{}", e); // eprintln! prints to stderr
        std::process::exit(1); // exit with error code 1
    }
}

fn readcsv(path: &str)-> Result<(), Box<dyn Error>>{ // Box<dyn Error> is a trait object that represents any type that implements the Error trait
    let mut rdr = csv::Reader::from_path(path)?; // ? is a shortcut for try!
    for result in rdr.records(){    // rdr.records() returns an iterator over the records in the CSV file
        let record = result?;// ? is a shortcut for try!
        println!("{:?}", record);  // prints the record
    }
    Ok(()) // Ok is a Result enum variant that indicates success
}
