use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
fn main() -> Result<()> {
    let mut response = reqwest::blocking::get("https://httpbin.org/get")?; //get the response from the url
    let mut body = String::new(); //create a new string
    response.read_to_string(&mut body)?; //read the response to the string
    println!("Status: {}", response.status()); //print the status
    println!("Headers:\n{:#?}", response.headers()); //print the headers
    println!("Body:\n{}", body);    //print the body

    Ok(())
}
