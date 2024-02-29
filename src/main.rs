pub mod movie;
mod tests;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder, Locator};
use movie::Movie;
use unidecode::unidecode;

async fn accept_paywall(p_client: &Client) -> Result<(), CmdError> {
    //Click the paywall if there is one
    p_client.goto("https://www.allocine.fr").await?;
    let v_button = p_client
        .find(Locator::Css("button.jad_cmp_paywall_button-cookies"))
        .await;

    match v_button {
        Ok(i) => i.click().await,
        Err(_) => Ok(()) // Skip if there is no paywall
    }
}

async fn get_all_cards_of_search(
    p_client: &Client,
    p_movie: &Movie,
) -> Result<Vec<Element>, CmdError> {
    //Find all movie cards in the search page
    p_client.goto(&p_movie.search_url).await?;
    p_client.current_url().await?;
    // let url = p_client.current_url().await?;
    // assert_eq!(url.as_ref(), &p_movie.search_url);
    let cards = p_client.find_all(Locator::Css("li.mdl")).await?;
    Ok(cards)
}

async fn get_movie_card(
    p_movie: &Movie,
    p_cards: &Vec<Element>,
) -> Result<Option<Element>, CmdError> {
    let card_iter = p_cards.into_iter();
    for card in card_iter {
        let director = unidecode(
            &card
                .find(Locator::Css(r#"div.meta-body-direction>a"#))
                .await?
                .prop("text")
                .await?
                .unwrap(),
        )
        .to_ascii_uppercase();

        let movie_title = unidecode(
            &card
                .find(Locator::Css("a.meta-title-link"))
                .await?
                .prop("text")
                .await?
                .unwrap(),
        )
        .to_ascii_uppercase();

        let card_of_movie: Option<Element> = match director.as_str() {
            ref x if x == &p_movie.director => match movie_title.as_str() {
                ref y if y == &p_movie.title => Some(card.clone()), //todo : clone nécessaire ?
                _ => None,
            },
            _ => None,
        };

        return Ok(card_of_movie);
    }
    Err(CmdError::InvalidArgument(
        String::from(""),
        String::from(""),
    )) //todo
}

async fn go_to_movie_page(p_card: &Element) -> Result<(), CmdError> {
    p_card.click().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CmdError> {
    let test_movie = Movie::new(
        String::from("Dune : Deuxième Partie"),
        String::from("Denis Villeneuve"),
    );

    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect(
            "Failed to connect to WebDriver. \n 
            Start the WebDriver with 'geckodriver' command",
        );

    //Gérer l'erreur ?
    accept_paywall(&client).await?;
    let cards = get_all_cards_of_search(&client, &test_movie).await?;
    let movie_card = get_movie_card(&test_movie, &cards).await?;
    match movie_card {
        Some(i) => go_to_movie_page(&i).await?,
        _ => println!(
            "No card found for this movie : {} {}",
            test_movie.director, test_movie.title
        ),
    };

    client.close().await
}