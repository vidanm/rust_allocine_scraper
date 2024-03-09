pub mod errors;
pub mod interact;
pub mod movie;
mod tests;
use std::error::Error;

use crate::movie::Movie;
use errors::AppError;
use fantoccini::elements::Element;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder};
use interact::AllocineAction;
use itertools::Itertools;
use rocket::http::hyper::body::HttpBody;
use rocket::State;

type Result<T> = std::result::Result<T, AppError>;

#[macro_use]
extern crate rocket;
use rocket::Shutdown;

#[get("/shutdown")]
async fn shutdown(shutdown: Shutdown, p_client: &State<Client>) -> &'static str {
    shutdown.notify();
    //close client
    // p_client.close().await;
    "Shutting down..."
}

#[get("/<director>/<title>")]
async fn index(director: String, title: String, p_client: &State<Client>) -> String {
    let p_movie = Movie::new(title.to_string(), director.to_string());
    let out = match AllocineAction::get_all_screenings_for_movie(p_client, &p_movie).await {
        Ok(c) => {
            let x: Vec<String> = c
                .into_iter()
                .map(|x| {
                    let mut s = x.cinema;
                    s.push_str("\n");
                    s.push_str(&x.location);
                    s.push_str(&x.version);
                    s.push_str(&x.date_time);
                    s.push_str("\n");
                    s.replace("Réserver","")
                })
                .collect();
            x.join("\n").to_owned()
        }
        Err(e) => match e {
            AppError::IllegalArgument => String::from("Illegal Argument Error"), //println!("Illegal Argument Error"),
            AppError::ElementNotFound => String::from("Element Not Found Error"), //println!("Element Not Found Error"),
            AppError::CmdError(e) => match e {
                CmdError::Standard(wd) => match wd.error() {
                    "element click intercepted" => {
                        let _ = AllocineAction::accept_paywall(p_client).await;
                        let _ =
                            AllocineAction::get_all_screenings_for_movie(p_client, &p_movie).await;
                        String::from("Element click intercepted")
                    }
                    _e => {
                        println!("{}", _e);
                        String::from("Error occured see console")
                    } //todo Revoir //todo Revoir
                },
                _e => {
                    println!("{}", _e);
                    String::from("Error occured see console")
                } //todo Revoir
            },
        },
    };
    out
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
