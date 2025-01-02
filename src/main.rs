#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        // apply pattern matching
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} directed by {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let book = Media::Book {
        title: String::from("Hobbit"),
        author: String::from("J.R.R.Tolkien"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Great Director"),
    };

    let bad_audiobook = Media::Audiobook {
        title: String::from("Bad audiobook"),
    };

    println!("{}", book.description());
    println!("{}", good_movie.description());
    println!("{}", bad_audiobook.description());
}
