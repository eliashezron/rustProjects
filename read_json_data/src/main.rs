use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph { //create a struct for the paragraph
    name: String
}
#[derive(Serialize, Deserialize)]
struct Article { //create a struct for the article
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() { //create a json string
    let json = r#" 
{
  "article": "how to work with json in Rust",
  "author": "elias",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
"#;
    let parsed: Article = read_json_typed(json); //parse the json string

    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[2].name); //print the name of the first paragraph
}
fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
    
}
