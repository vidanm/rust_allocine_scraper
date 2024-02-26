// use models::movie::movie::Movie;
pub mod movie;
use movie::Movie;

fn main() {
    let _movie_le_sacrifice = Movie::new(String::from("Le sacrifice"), String::from("Tarkovski"));
    // let screening_le_sacrifice = movie_le_sacrifice.get_screenings();
    // println!("{}", screening_le_sacrifice.cinema);

    // let directors = get_directors(&movie_le_sacrifice.title);
    // directors.into_iter().for_each(|item| println!("{}", item));
}

#[cfg(test)]
mod tests{
    use crate::Movie;

    #[test]
    fn title_is_formatted(){
        let _movie_le_sacrifice = Movie::new(String::from("Le sacrifice"), String::from("Tarkovski"));
        assert_eq!(_movie_le_sacrifice.title,"LE+SACRIFICE");
    }
}