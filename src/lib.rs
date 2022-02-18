#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod models;
mod utils;
// public api
mod collections;

use crate::utils::{game_dispatch, read_lines};
use collections::ItemCollection;
use models::{Game, Item, Line};

pub struct DataBase {
    pub games: ItemCollection<Game>,
    pub tags: ItemCollection<Item>,
    pub genres: ItemCollection<Item>,
}

impl DataBase {
    pub fn new(filename: &str) -> Self {
        let mut games: ItemCollection<Game> = ItemCollection::default();
        let tags: ItemCollection<Item> = ItemCollection::default();
        let genres: ItemCollection<Item> = ItemCollection::default();
        if let Ok(lines) = read_lines(filename) {
            for line in lines.flatten() {
                game_dispatch(Line::from(&line), &mut games);
            }
        }
        Self {
            games,
            tags,
            genres,
        }
    }
    // Game methods
    pub fn get_games_count(&self) -> &usize {
        &self.games.count
    }
    // Move the function below to item collection
    // Tag methods
    pub fn get_tags_count(&self) -> &usize {
        &self.tags.count
    }
    // Genre methods
    pub fn get_genres_count(&self) -> &usize {
        &self.genres.count
    }
}
