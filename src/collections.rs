use crate::models::{ItemTraits, GameTraits};

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
}



pub struct DataBase<G, T, K> {
    pub games: ItemCollection<G>,
    pub tags: ItemCollection<T>,
    pub genres: ItemCollection<K>,
}

impl<G, T, K> DataBase<G, T, K> {
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
