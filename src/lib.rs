/*A library that returns back a random movie name froom the list of the world top 10 best movies */

use rand::Rng;

//create an const array of the top 10 best movies around the world
pub const MOVIES: [&str; 10] = [
    "Afterlives by Abdulrazak Gurnah (Riverhead)",
    "An Immense World by Ed Yong (Random House) ",
    "Bad Mexicans by Kelly Lytle Hernández (Norton)",
    "The Book of Goose by Yiyun Li (Farrar, Straus & Giroux) ",
    "The Books of Jacob by Olga Tokarczuk, translated from the Polish by Jennifer Croft (Riverhead) ",
    "Checkout 19 by Claire-Louise Bennett (Riverhead) ",
    "Chilean Poet by Alejandro Zambra, translated from the Spanish by Megan McDowell (Viking) ",
    "Constructing a Nervous System by Margo Jefferson (Pantheon) ",
    "Continuous Creation by Les Murray (Farrar, Straus & Giroux) ",
    "Customs by Solmaz Sharif (Graywolf)",
];

//create a function that returns a random movie in the list above
pub fn random_movie() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..MOVIES.len());
    MOVIES[random_index]
}
