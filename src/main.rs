#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32), // <--- using unlabeled field here
    Placeholder,
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
            Media::Podcast(episode_number) => {
                format!("Podcast: {}", episode_number)
            }
            Media::Placeholder => {
                format!("Just a Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media)
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

    let podcast = Media::Podcast(777);

    let placeholder = Media::Placeholder;

    println!("{}", book.description());
    println!("{}", good_movie.description());
    println!("{}", bad_audiobook.description());
    println!("{}", podcast.description());
    println!("{}", placeholder.description());

    let mut catalog = Catalog::new();

    catalog.add(book);
    catalog.add(good_movie);
    catalog.add(bad_audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
}
