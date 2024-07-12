pub mod errors;
pub mod interact;
pub mod movie;
mod tests;
use crate::movie::Movie;
use errors::AppError;
use fantoccini::error::CmdError;
use fantoccini::{Client, ClientBuilder};
use interact::AllocineAction;
use rocket::State;
use serde_json;

#[macro_use]
extern crate rocket;
//use rocket::Shutdown;
//
//#[get("/shutdown")]
//async fn shutdown(shutdown: Shutdown) -> &'static str {
//    shutdown.notify();
//    //close client
//    // p_client.close().await;
//    "Shutting down..."
//}

#[get("/<director>/<title>")]
async fn index(director: &str, title: &str, p_client: &State<Client>) -> String {
    let p_movie = Movie::new(title.to_string(), director.to_string());
    let out = match AllocineAction::get_all_screenings_for_movie(p_client, &p_movie).await {
        Ok(c) => match serde_json::to_string(&c) {
            Ok(x) => x,
            Err(e) => e.to_string(),
        },
        Err(e) => match e {
            AppError::IllegalArgument => String::from("Illegal Argument Error"),
            AppError::ElementNotFound => String::from("Element Not Found Error"),
            AppError::CmdError(e) => match e {
                CmdError::Standard(wd) => match wd.error() {
                    "element click intercepted" => {
                        let _ = AllocineAction::accept_paywall(p_client).await;
                        format!("{}", "element click intercepted").to_string()
                        // A implémenter : récursion
                        // index(director,title,p_client).await
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

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    //let _client: &'static Client =
    // rocket::custom(config).mount("/",routes![index])
    //_client.close().await.expect("Can't close the client");
    //rocket::build().mount("/", routes![index])
    //rocket::build().manage(&_client).mount("/", routes![index])
    rocket::build()
        .manage(
            ClientBuilder::native()
                .connect("http://127.0.0.1:4444")
                .await
                .expect(
                    "Failed to connect to WebDriver. \n 
            Start the WebDriver with 'geckodriver' command",
                ),
        )
        .mount("/", routes![index])
}
// #[rocket::async_trait]
// impl Fairing for Client {

// }
