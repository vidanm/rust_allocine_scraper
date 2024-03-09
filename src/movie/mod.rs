pub mod screening;
use screening::Screening;
use unidecode::unidecode;
use core::fmt::{Result,Formatter,Display};

#[derive(Debug)]
pub struct Movie {
    pub search_url : String,
    pub title: String,
    pub director: String,
}

impl Display for Movie {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} : {}", self.title, self.director)
    }
}

impl Movie {
    pub fn new(title: String, director: String) -> Self {
        let formatted_title = title.replace(" ", "+");

        Self {
            // _card_url: Self::get_movie_url(&formatted_title, &director),
            search_url: String::from(format!("https://www.allocine.fr/rechercher/?q={}",formatted_title)),
            title: unidecode(&title).to_ascii_uppercase(),
            director: unidecode(&director).to_ascii_uppercase(),
        }
    }
}
