use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://httpbin.org/get").await?; //get the response from the url
    println!("Status: {}", response.status()); //print the status
    println!("Headers:\n{:#?}", response.headers()); //print the headers

    let body = response.text().await?; //get the body
    println!("Body:\n{}", body); //print the body
    Ok(())
}
