pub mod movie;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder, Locator};
use fantoccini::elements::Element;
use movie::Movie;
use unidecode::unidecode;

async fn search_for_movie(client:&Client, p_movie : &Movie) -> Result<Vec<Element>,CmdError> {
    client.goto(&p_movie.search_url).await?;
    let url = client.current_url().await?;
    assert_eq!(url.as_ref(), &p_movie.search_url);
    //Click the paywall if there is one
    client.find(Locator::Css("button.jad_cmp_paywall_button-cookies")).await?.click().await?;
    //Find all movie cards
    let cards = client.find_all(Locator::Css("li.mdl")).await?;
    Ok(cards)
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect(
            "Failed to connect to WebDriver. \n 
            Start the WebDriver with 'geckodriver' command",
        );

    let test_movie = Movie::new(
        String::from("Ledwajilj sacrifice"),
        String::from("Andreï Tarkovski"),
    );

    let cards = search_for_movie(&client,&test_movie).await;
    // Gérer une valeur de retour s'il n'y a pas le film selectionné
    // match cards.await {
    //     Ok(i) => Ok(i),
    //     Err(e) => Err(e)
    // };

    let card_iter = cards.unwrap().into_iter();
    for card in card_iter {
        let director = unidecode(
            &card
                .find(Locator::Css(r#"div.meta-body-direction>a"#))
                .await?
                .prop("text")
                .await?
                .unwrap()
                .to_ascii_uppercase(),
        );

        let movie = unidecode(
            &card
                .find(Locator::Css("a.meta-title-link"))
                .await?
                .prop("text")
                .await?
                .unwrap()
                .to_ascii_uppercase(),
        );

        println!("{} : {}", director, movie);

        if test_movie.director == director && movie == test_movie.title {
            let x = card
            .find(Locator::Css("a.meta-title-link"))
            .await?.click().await?;
        }
    }

    client.close().await
}

#[cfg(test)]
mod tests {
    use crate::Movie;

    #[test]
    fn title_is_formatted() {
        let _movie_le_sacrifice = Movie::new(
            String::from("Le sacrifice"),
            String::from("Andreï Tarkovski"),
        );
        assert_eq!(_movie_le_sacrifice.title, "LE+SACRIFICE");
    }
}
