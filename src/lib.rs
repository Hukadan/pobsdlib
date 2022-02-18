//! # pobsdlib
//! A library to in interact with the PlayOnBSD database
//! The database can be found at `https://github.com/playonbsd/OpenBSD-Games-Database`
//!
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod models;
mod utils;
// public api
mod collections;

use crate::utils::load_database;
use collections::ItemCollection;
use models::{Game, Item};

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
        load_database(filename, &mut games);
        Self {
            games,
            tags,
            genres,
        }
    }
    /// Return the number of games in the database
    pub fn get_games_count(&self) -> usize {
        self.games.get_count()
    }
    /// Return the number of tags in the database
    pub fn get_tags_count(&self) -> usize {
        self.tags.get_count()
    }
    /// Return the number of genres in the database
    pub fn get_genres_count(&self) -> usize {
        self.genres.get_count()
    }
}
