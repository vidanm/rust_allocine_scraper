#[cfg(test)]
use fantoccini::error::CmdError;
#[cfg(test)]
use fantoccini::ClientBuilder;
#[cfg(test)]
use crate::Movie;

#[tokio::test]
async fn client_is_connected() -> Result<(),CmdError>{
    let client = ClientBuilder::native()
    .connect("http://localhost:4444")
    .await
    .expect(
        "Failed to connect to WebDriver. \n 
        Start the WebDriver with 'geckodriver' command",
    );

    client.close().await
}

#[test]
fn movie_url_is_built() {
    let _movie_le_sacrifice = Movie::new(
        String::from("Le sacrifice"),
        String::from("Andreï Tarkovski"),
    );
    assert_eq!(_movie_le_sacrifice.search_url, "https://www.allocine.fr/rechercher/?q=Le+sacrifice");
    //Ok(())
}