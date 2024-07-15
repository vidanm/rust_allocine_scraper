pub mod errors;
pub mod interact;
pub mod movie;
mod tests;
use crate::movie::Movie;
use errors::AppError;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder};
use interact::AllocineAction;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::Shutdown;
use rocket::{Request, Response};

use serde_json;
#[macro_use]
extern crate rocket;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/shutdown")]
async fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}

async fn get_screenings(director: &str, title: &str) -> String {
    let p_client: Client = ClientBuilder::rustls()
        //.connect("http://geckodriver:4444")
        .connect("http://geckodriver:4444")
        .await
        .expect(
            "Failed to connect to WebDriver. \n 
            Start the WebDriver with 'geckodriver' command",
        );

    let p_movie = Movie::new(title.to_string(), director.to_string());
    let _ = AllocineAction::accept_paywall(&p_client).await;
    let _ = AllocineAction::close_interstitial(&p_client).await;

    let out = match AllocineAction::get_all_screenings_for_movie(&p_client, &p_movie).await {
        Ok(mut c) => {
            c.sort_unstable_by_key(|screening| {
                (screening.cinema.clone(), screening.version.clone())
            });

            let x: Vec<_> = c
                .chunk_by(|a, b| a.cinema.clone() == b.cinema.clone())
                .into_iter()
                .collect();

            match serde_json::to_string(&x) {
                Ok(x) => x,
                Err(e) => e.to_string(),
            }
        }
        Err(e) => match e {
            AppError::IllegalArgument => String::from("Illegal Argument Error"),
            AppError::ElementNotFound => String::from("Element Not Found Error"),
            AppError::CmdError(e) => match e {
                CmdError::Standard(wd) => match wd.error() {
                    "element click intercepted" => {
                        format!("{}", "element click intercepted").to_string()
                    }
                    _e => {
                        println!("{}", _e);
                        format!("{}", _e).to_string()
                    }
                },
                _e => {
                    println!("{}", _e);
                    format!("{}", _e).to_string()
                }
            },
        },
    };
    out
}

#[get("/<director>/<title>")]
async fn index(director: &str, title: &str) -> String {
    get_screenings(director, title).await
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, shutdown])
}
