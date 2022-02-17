extern crate pobsdlib;
extern crate serde_json;

use pobsdlib::collections::ItemCollection;
use pobsdlib::models::{Game, Line};
use pobsdlib::utils::{game_dispatch, read_lines};

fn main() {
    let games: Vec<Game> = Vec::new();
    let mut game_collection = ItemCollection::new(games);
    if let Ok(lines) = read_lines("/home/hukadan/Projects/Rust/Learning/pobsdlib/games.db") {
        for line in lines.flatten() {
            game_dispatch(Line::from(&line), &mut game_collection);
        }
    }
    let json_games = serde_json::to_string_pretty(&game_collection).unwrap();
    println!("{}", json_games);
}
