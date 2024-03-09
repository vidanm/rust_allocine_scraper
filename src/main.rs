pub mod errors;
pub mod movie;
pub mod interact;
mod tests;
use crate::movie::Movie;
use interact::AllocineAction;
use errors::AppError;
use rocket::State;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder};

type Result<T> = std::result::Result<T, AppError>;

async fn do_steps(p_client: &Client,p_title : &String,p_director: &String) -> Result<()> {

    let movie = Movie::new(p_title.to_string(),p_director.to_string());
    let cards = AllocineAction::get_all_cards_of_search(p_client, &movie).await?;
    let movie_card = AllocineAction::get_movie_card(&movie, &cards).await?;

    AllocineAction::go_to_screening_page(p_client, &movie_card).await?;

    let date_tabs: Vec<Element> = AllocineAction::get_date_tabs(&p_client).await?;

    for date_tab in date_tabs {
        let theater_cards = AllocineAction::get_theater_cards(&p_client, &date_tab).await?;
        for theater_card in theater_cards {
            let _theater_screenings = AllocineAction::get_theater_screenings(&theater_card).await?;
        }
    }

    Ok(())
}

#[macro_use] extern crate rocket;
use rocket::Shutdown;

#[get("/shutdown")]
async fn shutdown(shutdown: Shutdown,p_client : &State<Client>) -> &'static str {
    shutdown.notify();
    //close client
    // p_client.close().await;
    "Shutting down..."
}

#[get("/<director>/<title>")]
async fn index(director: String, title:String, p_client : &State<Client>) -> &'static str {

    match do_steps(p_client,&title,&director).await {
        Ok(c) => c,
        Err(e) => match e {
            AppError::IllegalArgument => println!("Illegal Argument Error"),
            AppError::ElementNotFound => println!("Element Not Found Error"),
            AppError::CmdError(e) => match e {
                CmdError::Standard(wd) => match wd.error() {
                    "element click intercepted" => {
                        let _ = AllocineAction::accept_paywall(p_client).await;
                        let _ = do_steps(p_client,&title,&director).await;
                    }
                    _e => println!("{}", _e), //todo Revoir
                },
                _e => println!("{}", _e), //todo Revoir
            },
        },
    };

    "Hello, world!"
}

// #[get("/")]
// async fn index(p_client : &State<Client>) -> &'static str {

//     "Hello, world!"
// }

#[launch]
#[tokio::main]
async fn rocket() -> _ {

    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect(
            "Failed to connect to WebDriver. \n 
            Start the WebDriver with 'geckodriver' command",
        );

    //S'occupe d'aller sur les pages et de cliquer sur le paywall quand il se présente

    
    // match client.close().await {
    //     Ok(_) => println!(""), //todo
    //     Err(_) => println!("")
    // }
    
    rocket::build().manage(client).mount("/", routes![index])
}
