use crate::utils::load_database;
use crate::models::{GameTraits, ItemTraits, Game, Item};

/// This collection can store items or games.
/// When used with items, ItemTraits are also needed.
/// When used with games, both ItemTraits and GameTraits are needed.
#[derive(Serialize, Default, Debug)]
pub struct ItemCollection<T> {
    pub count: usize,
    pub items: Vec<T>,
}

impl<T: ItemTraits> ItemCollection<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            count: items.len(),
            items,
        }
    }
    /// Adds an item and returns the item id.
    pub fn add_item(&mut self, mut item: T) -> usize {
        self.count += 1;
        item.set_id(self.count);
        self.items.push(item);
        self.count
    }
    /// Returns a refrence the item corresponding to the id if it exists, None otherwise.
    pub fn get_item_by_id(&self, id: usize) -> Option<&T> {
        match self.items.get(id - 1) {
            Some(item) => Some(item),
            None => None,
        }
    }
    /// Returns a reference the item corresponding to the name if it exists, None otherwise.
    pub fn get_item_by_name(&self, name: &str) -> Option<&T> {
        // assumre there is only one element with a given name
        match self.items.iter().find(|&item| item.get_name() == name) {
            Some(item) => Some(item),
            None => None,
        }
    }
}
#[cfg(test)]
mod collection_items_test {
    use super::*;
    use models::Item;
    #[test]
    fn collection_new_empty() {
        let items: Vec<Item> = Vec::new();
        let collection = ItemCollection::new(items);
        assert!(collection.count == 0);
    }
    #[test]
    fn collection_new_with_one() {
        let item = Item::new();
        let items = vec![item];
        let collection = ItemCollection::new(items);
        assert_eq!(collection.get_count(), 1);
    }
    #[test]
    fn collection_add_item_count() {
        let items: Vec<Item> = Vec::new();
        let mut collection = ItemCollection::new(items);
        let item = Item::new();
        collection.add_item(item);
        assert_eq!(collection.count, 1);
    }
    #[test]
    fn collection_add_item() {
        let items: Vec<Item> = Vec::new();
        let mut collection = ItemCollection::new(items);
        let item = Item::new();
        let id = collection.add_item(item);
        assert_eq!(id, collection.items[0].id);
    }
    #[test]
    fn collection_get_by_name() {
        let mut item1 = Item::new();
        item1.name = "item 1".to_string();
        let mut item2 = Item::new();
        item2.name = "item 2".to_string();
        let mut item2_bis = Item::new();
        item2_bis.name = "item 2".to_string();
        let items = vec![item1, item2];
        let collection = ItemCollection::new(items);
        match collection.get_item_by_name("item 2") {
            Some(item_check) => assert!(item2_bis == *item_check),
            None => panic!("Should have found item"),
        }
    }
    #[test]
    fn collection_get_by_id() {
        let mut item1 = Item::new();
        item1.id = 1;
        let mut item2 = Item::new();
        item2.id = 2;
        let mut item2_bis = Item::new();
        item2_bis.id = 2;
        let items = vec![item1, item2];
        let collection = ItemCollection::new(items);
        match collection.get_item_by_id(2) {
            Some(item_check) => assert!(item2_bis == *item_check),
            None => panic!("Should have found item"),
        }
    }
}

impl<T: GameTraits> ItemCollection<T> {
    /// Returns a vector of references to items corresponding to the tag.
    pub fn get_item_with_tag(&self, tag_name: &str) -> Vec<&T> {
        let gs = self
            .items
            .iter()
            .filter(|&item| item.get_tags().contains(&tag_name.to_string()));
        let mut games: Vec<&T> = Vec::new();
        for game in gs {
            games.push(game);
        }
        games
    }
    /// Returns a vector of references to items corresponding to the genre.
    pub fn get_item_with_genre(&self, genre_name: &str) -> Vec<&T> {
        let gs = self
            .items
            .iter()
            .filter(|&item| item.get_genres().contains(&genre_name.to_string()));
        let mut games: Vec<&T> = Vec::new();
        for game in gs {
            games.push(game);
        }
        games
    }
}

#[cfg(test)]
mod collection_games_test {
    use super::*;
    use models::Game;
    #[test]
    fn collection_get_by_tag() {
        let mut games: Vec<Game> = Vec::new();
        let mut g1 = Game::new();
        g1.name = "to be found".to_string();
        g1.tags = vec!["tag1".to_string()];
        games.push(g1);
        let mut g2 = Game::new();
        g2.name = "not to be found".to_string();
        g2.tags = vec!["tag2".to_string()];
        games.push(g2);
        let collection = ItemCollection::new(games);
        let g1_test = collection.get_item_with_tag("tag1");
        assert_eq!(g1_test[0].name, "to be found".to_string());
        assert_eq!(g1_test.len(), 1);
    }
    #[test]
    fn collection_get_by_genre() {
        let mut games: Vec<Game> = Vec::new();
        let mut g1 = Game::new();
        g1.name = "to be found".to_string();
        g1.genres = vec!["ge1".to_string()];
        games.push(g1);
        let mut g2 = Game::new();
        g2.name = "not to be found".to_string();
        g2.genres = vec!["ge2".to_string()];
        games.push(g2);
        let collection = ItemCollection::new(games);
        let g1_test = collection.get_item_with_genre("ge1");
        assert_eq!(g1_test[0].name, "to be found".to_string());
        assert_eq!(g1_test.len(), 1);
    }
}
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
