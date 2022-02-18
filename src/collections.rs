use crate::models::{GameTraits, ItemTraits};

#[derive(Serialize, Default)]
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
    pub fn add_item(&mut self, item: T) {
        self.count += 1;
        self.items.push(item);
    }
    pub fn display(&self) {
        println!("to be implemented");
    }
    pub fn get_item_by_id(&self, id: usize) -> Option<&T> {
        match self.items.get(id - 1) {
            Some(item) => Some(item),
            None => None,
        }
    }
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
        assert!(collection.count == 1);
    }
    #[test]
    fn collection_add_item() {
        let items: Vec<Item> = Vec::new();
        let mut collection = ItemCollection::new(items);
        let item = Item::new();
        collection.add_item(item);
        assert!(collection.count == 1);
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
        if let Some(item_check) = collection.get_item_by_name("item 2") {
            assert!(item2_bis == *item_check);
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
        if let Some(item_check) = collection.get_item_by_id(2) {
            assert!(item2_bis == *item_check);
        }
    }
}

impl<T: GameTraits> ItemCollection<T> {
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
