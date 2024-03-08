pub mod errors;
pub mod movie;
pub mod interact;
mod tests;
use crate::movie::Movie;
use interact::AllocineAction;
use errors::AppError;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder};

type Result<T> = std::result::Result<T, AppError>;

async fn do_steps(p_client: &Client) -> Result<()> {
    let test_movie = Movie::new(String::from("Dune"), String::from("Denis Villeneuve"));

    let cards = AllocineAction::get_all_cards_of_search(p_client, &test_movie).await?;
    let movie_card = AllocineAction::get_movie_card(&test_movie, &cards).await?;

    AllocineAction::go_to_screening_page(p_client, &movie_card).await?;

    let date_tabs: Vec<Element> = AllocineAction::get_date_tabs(&p_client).await?;
    let date_tab = date_tabs.first().ok_or_else(|| AppError::ElementNotFound)?;
    let theater_cards: Vec<Element> = AllocineAction::get_theater_cards(&p_client, &date_tab).await?;

    let theater_card = theater_cards
        .first()
        .ok_or_else(|| AppError::ElementNotFound)?;

    let _theater_screenings = AllocineAction::get_theater_screenings(&theater_card).await?;

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

    //S'occupe d'aller sur les pages et de cliquer sur le paywall quand il se présente
    match do_steps(&client).await {
        Ok(c) => c,
        Err(e) => match e {
            AppError::IllegalArgument => println!("Illegal Argument Error"),
            AppError::ElementNotFound => println!("Element Not Found Error"),
            AppError::CmdError(e) => match e {
                CmdError::Standard(wd) => match wd.error() {
                    "element click intercepted" => {
                        let _ = AllocineAction::accept_paywall(&client).await;
                        let _ = do_steps(&client).await;
                    }
                    _s => println!("{}", _s), //todo Revoir
                },
                _e => println!("{}", _e), //todo Revoir
            },
        },
    };

    let out = client.close().await?;
    Ok(out)
}
