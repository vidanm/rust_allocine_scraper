#[cfg(test)]
use crate::Movie;

#[test]
fn title_is_formatted() {
    let _movie_le_sacrifice = Movie::new(
        String::from("Le sacrifice"),
        String::from("Andreï Tarkovski"),
    );
    assert_eq!(_movie_le_sacrifice.search_url, "https://www.allocine.fr/rechercher/?q=Le+sacrifice");
}

