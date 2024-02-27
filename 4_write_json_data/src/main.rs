use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph { //create a struct for the paragraph
    name: String
}
#[derive(Serialize, Deserialize)]
struct Article{ //create a struct for the article
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}
fn main() {
    let article: Article = Article { //create an instance of the article struct
        article: "how to work with json in Rust".to_string(),
        author: "elias".to_string(),
        paragraph: vec![
            Paragraph { name: "starting sentences".to_string() },
            Paragraph { name: "body of the paragraph".to_string() },
            Paragraph { name: "end of the paragraph".to_string() }
        ]
    };
    let json = serde_json::to_string(&article).unwrap(); //convert the article struct to a json string
    println!("the json is {}", json); //print the json string
}
