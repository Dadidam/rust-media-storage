mod content;

use content::media::Media;
use content::catalog::Catalog;

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

    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("Item: {:#?}", value);
        }
        Option::None => {
            println!("Nothing found at the passed index");
        }
    }

    match catalog.get_by_index(10) {
        Option::Some(value) => {
            println!("{:#?}", value);
        }
        Option::None => {
            println!("No value here!")
        }
    }
}
