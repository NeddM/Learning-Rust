use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article = Article {
        article: String::from("Nombre artículo"),
        author: String::from("Pablo Neruda"),
        paragraph: {
            vec![
                Paragraph {
                    name: String::from("parrafo 1"),
                },
                Paragraph {
                    name: String::from("parrafo 2"),
                },
                Paragraph {
                    name: String::from("parrafo 3"),
                },
            ]
        },
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json);
}
