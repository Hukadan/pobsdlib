//! # pobsdlib
//! A library to in interact with the PlayOnBSD database
//! The database can be found at `https://github.com/playonbsd/OpenBSD-Games-Database`
//!
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
/// Models to represent a line of the database, an item (e.g. Engine) or a game.
pub mod models;
mod utils;
// public api
mod collections;

use crate::utils::load_database;
use collections::ItemCollection;
use models::{Game, Item};

/// # DataBase
/// Store the game database in three different collection:
/// - a games collection
/// - a tags collection
/// - a genres collection
///
/// Each collection stores items (being games, tags or genres) using the
/// follwing struct:
/// ```
/// pub struct ItemCollection<T> {
///     pub count: usize,
///     pub items: Vec<T>,
/// }
/// ```
///
/// The games collection also stores a vector of games, each game
/// being described using the following struct:
/// ```
/// pub struct Game {
///     pub id: usize,
///     pub name: String,
///     pub cover: String,
///     pub engine: String,
///     pub setup: String,
///     pub runtime: String,
///     pub store: String,
///     pub hints: String,
///     pub genres: Vec<String>,
///     pub tags: Vec<String>,
///     pub year: String,
///     pub dev: String,
///     pub publi: String,
///     pub version: String,
///     pub status: String,
/// }
/// ```
///
/// The tags/genres collection also stores a vector of tags/genres, each
/// tag/genre being described by the following struct:
/// ```
/// pub struct Item {
///     pub id: usize,
///     pub name: String,
///     pub games: Vec<usize>,
/// }
/// ```
///
pub struct DataBase {
    /// Store the games collection (see above for details).
    pub games: ItemCollection<Game>,
    /// Store the tags collection (see above for details).
    pub tags: ItemCollection<Item>,
    /// Store the genres collection (see above for details).
    pub genres: ItemCollection<Item>,
}

/// Public API
impl DataBase {
    /// Create a database from a file
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
