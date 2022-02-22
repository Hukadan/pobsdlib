use crate::collections::ItemCollection;
use crate::models::{Game, Item, GameTraits, ItemTraits, Field};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn split_line(line: &str) -> (&str, &str) {
    let split_line: Vec<&str> = line.split('\t').collect();
    let left: &str;
    let right: &str;
    // split the line in a left and right hand sides
    match split_line.len() {
        1 => {
            left = split_line[0];
            right = "";
        }
        2 => {
            left = split_line[0];
            right = split_line[1];
        }
        _ => {
            left = split_line[0];
            right = split_line[1];
            eprintln!(
                "WARNING: ignoring {}. Check tabs number in your database",
                split_line[2]
            );
        }
    };
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let test_str = "";
        assert_eq!(("", ""), split_line(&test_str));
    }
    #[test]
    fn test_no_tab() {
        let test_str = "notab";
        assert_eq!(("notab", ""), split_line(&test_str));
    }
    #[test]
    fn test_no_tab_space() {
        let test_str = "no tab";
        assert_eq!(("no tab", ""), split_line(&test_str));
    }
    #[test]
    fn test_one_tab() {
        let test_str = "one\ttab";
        assert_eq!(("one", "tab"), split_line(&test_str));
    }
    #[test]
    fn test_two_tab() {
        let test_str = "one\ttab\tanother";
        assert_eq!(("one", "tab"), split_line(&test_str));
    }
}

pub fn game_dispatch(field: Field, games: &mut ItemCollection<Game>) {
    match field {
        Field::NewGame(_) => {
            let mut game = Game::default();
            game.set_id(games.count + 1);
            game.update(field);
            games.add_item(game);
        }
        Field::SingleItem(_, _) | Field::MultipleItems(_, _) => {
            if let Some(game) = games.items.last_mut() {
                game.update(field)
            };
        }
    };
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn load_database(filename: &str, games: &mut ItemCollection<Game>) {
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            game_dispatch(Field::from(&line), games);
        }
    }
}

pub fn load_tags_from_games(tags: &mut ItemCollection<Item>, games: &ItemCollection<Game>) {
    for game in &games.items {
        if game.tags.len()> 0 {
            for tag in &game.tags {
                match tags.get_item_by_name_mut(&tag) {
                    Some(tag_item) => tag_item.games.push(game.id),
                    None => {
                        let mut newtag = Item::new();
                        newtag.name = tag.to_string();
                        newtag.games.push(game.id);
                        tags.items.push(newtag);
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests_load_tags {
    use super::*;
    #[test]
    fn test_load_tags() {
        let tags: Vec<Item> = Vec::new(); 
        let games: Vec<Game> = Vec::new(); 
        let mut tagcollection = ItemCollection::new(tags);
        let mut gamecollection = ItemCollection::new(games);
        let mut game = Game::new();
        game.tags = vec!["tag1".to_string(), "tag2".to_string()];
        gamecollection.add_item(game);
        let mut game = Game::new();
        game.tags = vec!["tag1".to_string(), "tag3".to_string()];
        gamecollection.add_item(game);
        let mut game = Game::new();
        game.tags = vec!["tag1".to_string()];
        gamecollection.add_item(game);
        load_tags_from_games(&mut tagcollection, &gamecollection);
        // Check if the number of tags is correct
        assert_eq!(tagcollection.items.len(),3);
        // Check if the tag names are correct
        assert_eq!(tagcollection.items[0].name,"tag1".to_string());
        assert_eq!(tagcollection.items[1].name,"tag2".to_string());
        assert_eq!(tagcollection.items[2].name,"tag3".to_string());
        // Check if the game ids associated to the tags are correct
        assert_eq!(tagcollection.items[0].games, vec![1, 2, 3]);
        assert_eq!(tagcollection.items[1].games, vec![1]);
        assert_eq!(tagcollection.items[2].games, vec![2]);
    }
}

pub fn load_genres_from_games(genres: &mut ItemCollection<Item>, games: &ItemCollection<Game>) {
    for game in &games.items {
        if game.genres.len()> 0 {
            for genre in &game.genres {
                match genres.get_item_by_name_mut(&genre) {
                    Some(genre_item) => genre_item.games.push(game.id),
                    None => {
                        let mut newgenre = Item::new();
                        newgenre.name = genre.to_string();
                        newgenre.games.push(game.id);
                        genres.items.push(newgenre);
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests_load_genres {
    use super::*;
    #[test]
    fn test_load_genres() {
        let genres: Vec<Item> = Vec::new(); 
        let games: Vec<Game> = Vec::new(); 
        let mut genrecollection = ItemCollection::new(genres);
        let mut gamecollection = ItemCollection::new(games);
        let mut game = Game::new();
        game.genres = vec!["gen1".to_string(), "gen2".to_string()];
        gamecollection.add_item(game);
        let mut game = Game::new();
        game.genres = vec!["gen1".to_string(), "gen3".to_string()];
        gamecollection.add_item(game);
        let mut game = Game::new();
        game.genres = vec!["gen1".to_string()];
        gamecollection.add_item(game);
        load_genres_from_games(&mut genrecollection, &gamecollection);
        // Check if the number of tags is correct
        assert_eq!(genrecollection.items.len(),3);
        // Check if the tag names are correct
        assert_eq!(genrecollection.items[0].name,"gen1".to_string());
        assert_eq!(genrecollection.items[1].name,"gen2".to_string());
        assert_eq!(genrecollection.items[2].name,"gen3".to_string());
        // Check if the game ids associated to the tags are correct
        assert_eq!(genrecollection.items[0].games, vec![1, 2, 3]);
        assert_eq!(genrecollection.items[1].games, vec![1]);
        assert_eq!(genrecollection.items[2].games, vec![2]);
    }
}
