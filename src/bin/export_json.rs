extern crate pobsdlib;
extern crate serde_json;
use pobsdlib::DataBase;

fn main() {
    let db_game = DataBase::new("/home/hukadan/Projects/Rust/Learning/pobsdlib/games.db");
    let json_games = serde_json::to_string_pretty(&db_game.games).unwrap();
    println!("{}", json_games);
}
