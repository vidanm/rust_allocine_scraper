use crate::movie::Movie;
use crate::errors::AppError;
use crate::movie::screening::Screening;
use std::time::Duration;
use unidecode::unidecode;
use fantoccini::elements::Element;
use fantoccini::{Client, Locator};

pub struct Interact;

type Result<T> = std::result::Result<T, AppError>;

impl Interact {
    pub async fn accept_paywall(p_client: &Client) -> Result<()> {
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

    pub async fn get_all_cards_of_search(p_client: &Client, p_movie: &Movie) -> Result<Vec<Element>> {
        //Find all movie cards in the search page
        p_client.goto(&p_movie.search_url).await?;
        p_client.current_url().await?;
        // let url = p_client.current_url().await?;
        // assert_eq!(url.as_ref(), &p_movie.search_url);
        let cards = p_client.find_all(Locator::Css("li.mdl")).await?;
        Ok(cards)
    }

    pub async fn get_movie_card(p_movie: &Movie, p_cards: &Vec<Element>) -> Result<Element> {
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

    pub async fn go_to_screening_page(p_client: &Client, p_card: &Element) -> Result<()> {
        let link_to_movie_page = p_card.find(Locator::Css("a.meta-title-link")).await?;
        link_to_movie_page.click().await?;

        let screening_tab = p_client
            .find(Locator::Css("#js-showtimes-nav-redir"))
            .await?;

        screening_tab.click().await?;
        p_client
            .wait()
            .at_most(Duration::from_secs(3))
            .for_element(Locator::Css("div#calendar-date-roller"))
            .await?;
        Ok(())
    }

    pub async fn get_date_tabs(p_client: &Client) -> Result<Vec<Element>> {
        let date_tabs = p_client
            .find_all(Locator::Css("span.calendar-date-link:not(.disabled)"))
            .await?;

        Ok(date_tabs)
    }

    pub async fn get_theater_cards(p_client: &Client, p_date_tab: &Element) -> Result<Vec<Element>> {
        p_date_tab.click().await?;
        p_client
            .wait()
            .at_most(Duration::from_secs(3))
            .for_element(Locator::Css("div.theater-card"))
            .await?;

        let result = p_client.find_all(Locator::Css("div.theater-card")).await?;

        Ok(result)
    }

    pub async fn get_theater_screenings(p_theater_card: &Element) -> Result<Vec<Screening>> {
        let mut screenings: Vec<Screening> = Vec::new();

        let v_screening_cards = p_theater_card
            .find_all(Locator::Css("span.showtimes-hour-item-value"))
            .await?;

        for scr_card in v_screening_cards.into_iter() {
            let datetime = scr_card.text().await?;
            screenings.push(Screening::new(
                datetime,
                String::from("UNDEFINED"),
                String::from("UNDEFINED"),
            ));
        }

        println!("{:?}", screenings);
        Ok(screenings)
    }
}
