pub mod errors;
pub mod movie;
mod tests;
use errors::AppError;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder, Locator};
use movie::screening::Screening;
use movie::Movie;
use unidecode::unidecode;

type Result<T> = std::result::Result<T, AppError>;

async fn accept_paywall(p_client: &Client) -> Result<()> {
    //Click the paywall if there is one
    p_client.goto("https://www.allocine.fr").await?;
    let v_button = p_client
        .find(Locator::Css("button.jad_cmp_paywall_button-cookies"))
        .await;


    if let Ok(i) = v_button {
        i.click().await?
    }

    Ok(()) //Skip if there is no paywall
}

async fn get_all_cards_of_search(p_client: &Client, p_movie: &Movie) -> Result<Vec<Element>> {
    //Find all movie cards in the search page
    p_client.goto(&p_movie.search_url).await?;
    p_client.current_url().await?;
    // let url = p_client.current_url().await?;
    // assert_eq!(url.as_ref(), &p_movie.search_url);
    let cards = p_client.find_all(Locator::Css("li.mdl")).await?;
    Ok(cards)
}

async fn get_movie_card(p_movie: &Movie, p_cards: &Vec<Element>) -> Result<Element> {
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

        // println!("{} {}", &movie_title, &director);

        if movie_title == p_movie.title && director == p_movie.director {
            return Ok(card.clone());
        }
    }

    Err(AppError::ElementNotFound)
}

async fn go_to_screening_page(p_client: &Client, p_card: &Element) -> Result<()> {
    let link_to_movie_page = p_card.find(Locator::Css("a.meta-title-link")).await?;

    link_to_movie_page.click().await?;

    let screening_tab = p_client
        .find(Locator::Css("#js-showtimes-nav-redir"))
        .await?;

    screening_tab.click().await?;
    Ok(())
}

async fn get_date_tabs(p_client: &Client) -> Result<Vec<Element>> {

    let date_tabs = p_client
        .find_all(Locator::Css("span.calendar-date-link:not(.disabled)"))
        .await?;

    Ok(date_tabs)
}

async fn get_theater_cards(p_date_tab: &Element) -> Result<Vec<Element>> {
    println!("B");

    p_date_tab.click().await?;
    let result = p_date_tab
        .find_all(Locator::Css("div.theater-card"))
        .await?;
    println!("B");

    Ok(result)
}

async fn get_theater_screenings(p_theater_card: &Element) -> Result<Vec<Screening>> {
    let mut screenings: Vec<Screening> = Vec::new();

    println!("A");

    let v_screening_cards = p_theater_card
        .find_all(Locator::Css("span.showtimes-hour-item-value"))
        .await?;

        println!("A");

    for scr_card in v_screening_cards.into_iter() {
        screenings.push(Screening::new(
            scr_card.prop("text").await?.unwrap(),
            String::from("UNDEFINED"),
            String::from("UNDEFINED"),
        ));
    }
    println!("A");
    println!("{:?}", screenings);
    Ok(screenings)
}

async fn do_steps(p_client: &Client) -> Result<()> {
    let test_movie = Movie::new(String::from("Dune"), String::from("Denis Villeneuve"));

    let cards = get_all_cards_of_search(p_client, &test_movie).await?;
    let movie_card = get_movie_card(&test_movie, &cards).await?;

    go_to_screening_page(p_client, &movie_card).await?;

    let date_tabs: Vec<Element> = get_date_tabs(&p_client).await?;
    let date_tab = date_tabs.first().ok_or_else(|| AppError::ElementNotFound)?;
    let theater_cards: Vec<Element> = get_theater_cards(&date_tab).await?;

    let theater_card = theater_cards
        .first()
        .ok_or_else(|| AppError::ElementNotFound)?;

    let _theater_screenings = get_theater_screenings(&theater_card).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect(
            "Failed to connect to WebDriver. \n 
            Start the WebDriver with 'geckodriver' command",
        );

    //todo : add cookie for paywall ?

    //S'occupe d'aller sur les pages et de cliquer sur le paywall quand il se présente
    match do_steps(&client).await {
        Ok(c) => c,
        Err(e) => match e {
            AppError::IllegalArgument => println!("Illegal Argument Error"),
            AppError::ElementNotFound => println!("Element Not Found Error"),
            AppError::CmdError(e) => match e {
                CmdError::Standard(wd) => match wd.error() {
                    "element click intercepted" => {
                        let _ = accept_paywall(&client).await;
                        let _ = do_steps(&client).await;
                    },
                    _s => println!("{}",_s) //todo Revoir
                },
                _e => println!("{}",_e) //todo Revoir
            },
        },
    };

    let out = client.close().await?;
    Ok(out)
}
