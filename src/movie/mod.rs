pub mod screening;
use scraper::{Html, Selector};
use screening::Screening;
use unidecode::unidecode;

pub struct Movie {
    _card_url: Option<String>,
    pub title: String,
    pub director: String,
}

impl Movie {
    pub fn new(title: String, director: String) -> Self {
        let formatted_title = title.replace(" ", "+").to_ascii_uppercase();

        Self {
            _card_url: Self::get_movie_url(&formatted_title, &director),
            title: formatted_title,
            director: director.to_ascii_uppercase(),
        }
    }

    fn get_movie_url(title: &String, _direction: &String) -> Option<String> {
        let url_start: String = String::from("https://www.allocine.fr/rechercher/?q=");
        let url = format!("{}{}", url_start, title);
        let response = reqwest::blocking::get(url).unwrap().text().unwrap();
        let document = Html::parse_document(&response);
        let card_selector = Selector::parse("div.entity-card>.meta").unwrap();
        let direction_selector = Selector::parse("div.meta-body-direction>span").unwrap();
        let title_selector = Selector::parse("div.meta-title>span").unwrap();

        //todo itérer sur toutes les cards
        // et comparer les noms real et titres
        let card = document.select(&card_selector).next().unwrap();
        let found_title: String = card
            .select(&title_selector)
            .map(|x| unidecode(&x.inner_html()).to_ascii_uppercase())
            .collect();

        let found_direction: String = card
            .select(&direction_selector)
            .map(|x| unidecode(&x.inner_html()).to_ascii_uppercase())
            .filter(|x| !x.eq("DE"))
            .collect();

        let found_url: String = card
            .select(&title_selector)
            .map(|x| x.value().attr("href").unwrap())
            .collect();

        println!(
            "Trouvé sur la page : {} {} {}",
            found_direction, found_title, found_url
        );
        Some(found_url)
    }

    pub fn get_screenings(&self) -> Screening {
        Screening {
            date_time: String::from(""),
            location: String::from(""),
            cinema: String::from(""),
        }
    }
}