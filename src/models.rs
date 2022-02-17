use crate::utils::split_line;

/// # Trait used on items stored in an ItemCollection
pub trait ItemTraits {
    fn set_id(&mut self, id: usize);
    fn get_name(&self) -> &str;
}

/// #Traits specific to Game
pub trait GameTraits {
    fn update(&mut self, line: Line);
    fn get_tags(&self) -> &Vec<String>;
    fn get_genres(&self) -> &Vec<String>;
}

/// # Represent a line of the game database
/// There is three different variant:
/// * a first variant for Game entries
/// * a second variant for entries related to single items (i.e. Engine)
/// * a third variant for entries related to multiple itmes (i.e Tags)
#[derive(PartialEq)]
pub enum Line {
    NewGame(String),
    SingleItem(String, String),
    MultipleItems(String, Vec<String>),
}

impl Line {
    pub fn from(line: &str) -> Self {
        // split the line in a left and right hand sides
        let (left, right) = split_line(line);
        // use the left hand side to discriminate between single and multiple item lines
        // sould panic if the disctinction cannot be made
        match left {
            "Game" => Line::NewGame(right.to_string()),
            "Cover" | "Engine" | "Setup" | "Runtime" | "Store" | "Hints" | "Year" | "Dev"
            | "Pub" | "Version" | "Status" => Line::SingleItem(left.to_string(), right.to_string()),
            "Genre" | "Tags" => {
                let mut items: Vec<String> = Vec::new();
                for item in right.split(',') {
                    items.push(item.trim().to_string());
                }
                Line::MultipleItems(left.to_string(), items)
            }
            _ => panic!("Unkown filed {}", left),
        }
    }
}

#[cfg(test)]
mod line_tests {
    use super::*;
    #[test]
    fn game_line() {
        let input = "Game\tToto";
        let line = Line::from(&input);
        assert!(Line::NewGame("Toto".to_string()) == line);
    }
    #[test]
    fn single_line() {
        let input = "Cover\tToto";
        let line = Line::from(&input);
        assert!(Line::SingleItem("Cover".to_string(), "Toto".to_string()) == line);
    }
    #[test]
    fn mutilple_line() {
        let input = "Genre\tfirst, second";
        let line = Line::from(&input);
        assert!(
            Line::MultipleItems(
                "Genre".to_string(),
                vec!["first".to_string(), "second".to_string()]
            ) == line
        );
    }
}

/// # Represent an item, at the moment a tag or a genre
#[derive(Default, PartialEq)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub games: Vec<usize>,
}

impl Item {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ItemTraits for Item {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;
    #[test]
    fn item_new() {
        let item = Item::new();
        let item_bis = Item::default();
        assert!(item == item_bis);
    }
    #[test]
    fn item_set_id() {
        let mut item = Item::new();
        item.set_id(2);
        let item_bis = Item {
            id: 2,
            name: "".to_string(),
            games: Vec::new(),
        };
        assert!(item == item_bis);
    }
    #[test]
    fn item_get_name() {
        let item = Item {
            id: 2,
            name: "toto".to_string(),
            games: Vec::new(),
        };
        assert_eq!(item.get_name(), "toto");
    }
}
/// # Represent a game
#[derive(Serialize, Default, PartialEq)]
pub struct Game {
    pub id: usize,
    pub name: String,
    pub cover: String,
    pub engine: String,
    pub setup: String,
    pub runtime: String,
    pub store: String,
    pub hints: String,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub year: String,
    pub dev: String,
    pub publi: String,
    pub version: String,
    pub status: String,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ItemTraits for Game {
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GameTraits for Game {
    fn update(&mut self, line: Line) {
        match line {
            Line::NewGame(name) => self.name = name,
            Line::SingleItem(left, right) => {
                match left.as_str() {
                    "Cover" => self.cover = right,
                    "Engine" => self.engine = right,
                    "Setup" => self.setup = right,
                    "Runtime" => self.runtime = right,
                    "Store" => self.store = right,
                    "Hints" => self.hints = right,
                    "Year" => self.year = right,
                    "Dev" => self.dev = right,
                    "Pub" => self.publi = right,
                    "Version" => self.version = right,
                    "Status" => self.status = right,
                    _ => panic!("unknown single item field: unable to set"),
                };
            }
            Line::MultipleItems(left, right) => {
                match left.as_str() {
                    "Tags" => self.tags = right,
                    "Genre" => self.genres = right,
                    _ => panic!("unknown multiple item field: unable to set"),
                };
            }
        };
    }
    fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    fn get_genres(&self) -> &Vec<String> {
        &self.genres
    }
}
#[cfg(test)]
mod game_tests {
    use super::*;
    #[test]
    fn game_new() {
        let game = Game::new();
        let game_bis = Game::default();
        assert!(game == game_bis);
    }
    #[test]
    fn game_set_id() {
        let mut game = Game::new();
        game.set_id(2);
        let mut game_bis = Game::new();
        game_bis.id = 2;
        assert!(game == game_bis);
    }
    #[test]
    fn game_get_name() {
        let mut game = Game::new();
        game.name = "toto".to_string();
        assert_eq!(game.get_name(), "toto");
    }
    #[test]
    fn game_update_name() {
        let mut game = Game::new();
        let line = Line::NewGame("Test".to_string());
        game.update(line);
        assert_eq!(game.name, "Test".to_string());
    }
    #[test]
    fn game_update_cover() {
        let mut game = Game::new();
        let line = Line::SingleItem("Cover".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.cover, "Test".to_string());
    }
    #[test]
    fn game_update_engine() {
        let mut game = Game::new();
        let line = Line::SingleItem("Engine".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.engine, "Test".to_string());
    }
    #[test]
    fn game_update_setup() {
        let mut game = Game::new();
        let line = Line::SingleItem("Setup".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.setup, "Test".to_string());
    }
    #[test]
    fn game_update_runtime() {
        let mut game = Game::new();
        let line = Line::SingleItem("Runtime".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.runtime, "Test".to_string());
    }
    #[test]
    fn game_update_store() {
        let mut game = Game::new();
        let line = Line::SingleItem("Store".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.store, "Test".to_string());
    }
    #[test]
    fn game_update_hints() {
        let mut game = Game::new();
        let line = Line::SingleItem("Hints".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.hints, "Test".to_string());
    }
    #[test]
    fn game_update_year() {
        let mut game = Game::new();
        let line = Line::SingleItem("Year".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.year, "Test".to_string());
    }
    #[test]
    fn game_update_dev() {
        let mut game = Game::new();
        let line = Line::SingleItem("Dev".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.dev, "Test".to_string());
    }
    #[test]
    fn game_update_publi() {
        let mut game = Game::new();
        let line = Line::SingleItem("Pub".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.publi, "Test".to_string());
    }
    #[test]
    fn game_update_version() {
        let mut game = Game::new();
        let line = Line::SingleItem("Version".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.version, "Test".to_string());
    }
    #[test]
    fn game_update_status() {
        let mut game = Game::new();
        let line = Line::SingleItem("Status".to_string(), "Test".to_string());
        game.update(line);
        assert_eq!(game.status, "Test".to_string());
    }
    #[test]
    #[should_panic]
    fn game_single_panic() {
        let mut game = Game::new();
        let line = Line::SingleItem("Panic".to_string(), "Test".to_string());
        game.update(line);
    }
    #[test]
    fn game_update_tags() {
        let mut game = Game::new();
        let line = Line::MultipleItems(
            "Tags".to_string(),
            vec!["Tag1".to_string(), "Tag2".to_string()],
        );
        game.update(line);
        assert_eq!(game.tags, vec!["Tag1".to_string(), "Tag2".to_string()]);
    }
    #[test]
    fn game_update_genres() {
        let mut game = Game::new();
        let line = Line::MultipleItems(
            "Genre".to_string(),
            vec!["Ge1".to_string(), "Ge2".to_string()],
        );
        game.update(line);
        assert_eq!(game.genres, vec!["Ge1".to_string(), "Ge2".to_string()]);
    }
    #[test]
    #[should_panic]
    fn game_multiple_panic() {
        let mut game = Game::new();
        let line = Line::MultipleItems(
            "Panic".to_string(),
            vec!["Ge1".to_string(), "Ge2".to_string()],
        );
        game.update(line);
    }
    #[test]
    fn get_tags() {
        let mut game = Game::new();
        game.tags = vec!["Tag1".to_string(), "Tag2".to_string()];
        assert!(game.get_tags() == &vec!["Tag1".to_string(), "Tag2".to_string()]);
    }
    #[test]
    fn get_genres() {
        let mut game = Game::new();
        game.genres = vec!["Ge1".to_string(), "Ge2".to_string()];
        assert!(game.get_genres() == &vec!["Ge1".to_string(), "Ge2".to_string()]);
    }
}
