use crate::utils::split_line;

/// This trait is needed if you use ItemCollection with Item or Game struct.
pub trait ItemTraits {
    fn set_id(&mut self, id: usize);
    fn get_name(&self) -> &str;
}

/// This trait is needed if you use ItemCollection with Game struct.
pub trait GameTraits {
    fn update(&mut self, field: Field);
    fn get_tags(&self) -> &Vec<String>;
    fn get_genres(&self) -> &Vec<String>;
}

/// # Represent a field generated form a line of the game database
/// There is three different variants:
/// * a first variant for Game entries;
/// * a second variant for entries related to single items (i.e. Engine);
/// * a third variant for entries related to multiple items (i.e Tags).
///
///
/// ## Field::NewGame
/// A line corresponding to a Game field will produce a Field::NewGame
/// storing the name of the game.
/// ```
/// use pobsdlib::models::Field;
///
/// let line_str = "Game\tName of the game";
/// let field = Field::from(line_str);
///
/// assert_eq!(field,Field::NewGame("Name of the game".to_string()));
/// ```
///
/// ## Field::SingleItem
/// A line corresponding to a single item field (e.g. Engine) will produce
/// a Field::SingleItem storing the kind of item and its name.
/// ```
/// use pobsdlib::models::Field;
///
/// let line_str = "Engine\tEngine name";
/// let field = Field::from(line_str);
///
/// assert_eq!(field,Field::SingleItem("Engine".to_string(),"Engine name".to_string()));
/// ```
///
/// ## Field::MultipleItems
/// A line corresponding to a multiples items field (e.g. Tags) will produce
/// a Field::MultipleItems storing the kind of item and the items.
/// ```
/// use pobsdlib::models::Field;
///
/// let line_str = "Tags\ttag1,tag2";
/// let field = Field::from(line_str);
///
/// assert_eq!(field,Field::MultipleItems("Tags".to_string(),vec!["tag1".to_string(),"tag2".to_string()]));
/// ```
/// Note that while Tags and Genres are coma separated values, Stores are space separated ones.
/// This is handled by the `Field::from` method.
#[derive(PartialEq, Debug)]
pub enum Field {
    NewGame(String),
    SingleItem(String, String),
    MultipleItems(String, Vec<String>),
}

impl Field {
    /// Try to convert a line of the database in a Field enum. Panic if it cannot.
    pub fn from(line: &str) -> Self {
        // split the line in a left and right hand sides
        let (left, right) = split_line(line);
        // use the left hand side to discriminate between single and multiple item lines
        match left {
            "Game" => Field::NewGame(right.to_string()),
            "Cover" | "Engine" | "Setup" | "Runtime" | "Hints" | "Year" | "Dev" | "Pub"
            | "Version" | "Status" => Field::SingleItem(left.to_string(), right.to_string()),
            "Store" => {
                let mut items: Vec<String> = Vec::new();
                for item in right.split(' ') {
                    items.push(item.trim().to_string());
                }
                Field::MultipleItems(left.to_string(), items)
            }
            "Genre" | "Tags" => {
                let mut items: Vec<String> = Vec::new();
                for item in right.split(',') {
                    items.push(item.trim().to_string());
                }
                Field::MultipleItems(left.to_string(), items)
            }
            _ => panic!("Unkown filed {}", left),
        }
    }
    /// Returns the string corresponding to the line in the database
    /// ```
    /// use pobsdlib::models::Field;
    /// let input = "Engine\tSuper engine";
    /// let field = Field::from(&input);
    /// assert_eq!(field.as_line(), input.to_string());
    /// let input = "Genre\tGe1, Ge2";
    /// let field = Field::from(&input);
    /// assert_eq!(field.as_line(), input.to_string());
    /// ```
    pub fn as_line(&self) -> String {
        match self {
            Field::NewGame(name) => vec!["Game", name].join("\t"),
            Field::SingleItem(left, right) => vec![left.to_owned(), right.to_string()].join("\t"),
            Field::MultipleItems(left, right) => {
                if left.eq(&"Store".to_string()) {
                    vec![left.to_owned(), right.join(" ").to_string()].join("\t")
                } else {
                    vec![left.to_owned(), right.join(", ").to_string()].join("\t")
                }
            }
        }
    }
}

#[cfg(test)]
mod field_tests {
    use super::*;
    #[test]
    fn as_line_game() {
        let input = "Game\tToto";
        let field = Field::from(&input);
        assert_eq!(field.as_line(), input.to_string());
    }
    #[test]
    fn as_line_engine() {
        let input = "Engine\tToto";
        let field = Field::from(&input);
        assert_eq!(field.as_line(), input.to_string());
    }
    #[test]
    fn as_line_tags() {
        let input = "Tags\ttag1, tag2";
        let field = Field::from(&input);
        assert_eq!(field.as_line(), input.to_string());
    }
    #[test]
    fn as_line_stores() {
        let input = "Tags\turl1 url2";
        let field = Field::from(&input);
        assert_eq!(field.as_line(), input.to_string());
    }
    #[test]
    fn game_line() {
        let input = "Game\tToto";
        let field = Field::from(&input);
        assert!(Field::NewGame("Toto".to_string()) == field);
    }
    #[test]
    fn single_line() {
        let input = "Cover\tToto";
        let field = Field::from(&input);
        assert!(Field::SingleItem("Cover".to_string(), "Toto".to_string()) == field);
    }
    #[test]
    fn mutilple_line() {
        let input = "Genre\tfirst, second";
        let field = Field::from(&input);
        assert!(
            Field::MultipleItems(
                "Genre".to_string(),
                vec!["first".to_string(), "second".to_string()]
            ) == field
        );
    }
    #[test]
    #[should_panic]
    fn panic_line() {
        let input = "Let's panic";
        Field::from(&input);
    }
}

/// # Represent an item.
/// At the moment, only tags and genres are represented this way.
#[derive(Default, PartialEq)]
pub struct Item {
    /// The id of the tag.
    pub id: usize,
    /// The name of the tag.
    pub name: String,
    /// A vector of ids of the games with such a tag.
    pub games: Vec<usize>,
}

impl Item {
    /// Is equivalent to Item::Default()
    pub fn new() -> Self {
        Self::default()
    }
}

impl ItemTraits for Item {
    /// Sets the id of the item.
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    /// Returns the name of the item.
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
    /// The id of the game.
    pub id: usize,
    /// The name of the game.
    pub name: String,
    /// The cover of the game.
    pub cover: String,
    /// The engine used by the game.
    pub engine: String,
    /// Step(s) to setup the game.
    pub setup: String,
    /// The executable in the package.
    pub runtime: String,
    /// A vector with store urls.
    pub store: Vec<String>,
    /// Hints (as the name imply).
    pub hints: String,
    /// A vector of genres associated with the game.
    pub genres: Vec<String>,
    /// A vector of tags associated with the game.
    pub tags: Vec<String>,
    /// Released year.
    pub year: String,
    /// Developer (as the name imply).
    pub dev: String,
    /// Publisher.
    pub publi: String,
    /// Version of the game.
    pub version: String,
    /// When tested on -current.
    pub status: String,
}

impl Game {
    /// Is equivalent to Game::Defautl().
    pub fn new() -> Self {
        Self::default()
    }
}

impl ItemTraits for Game {
    /// Sets the id of the game.
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    /// Returns the name of the game.
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GameTraits for Game {
    /// Sets one attribute of the game according to the Field enum given.
    /// ```
    /// use pobsdlib::models::{Field,Game,GameTraits};
    ///
    /// let line_str = "Game\tName of the game";
    /// let field = Field::from(line_str);
    /// let mut game = Game::new();
    /// game.update(field);
    /// assert_eq!(game.name,"Name of the game".to_string());
    /// ```
    /// The id cannot be set this way and the `set_id` method must be used.
    fn update(&mut self, field: Field) {
        match field {
            Field::NewGame(name) => self.name = name,
            Field::SingleItem(left, right) => {
                match left.as_str() {
                    "Cover" => self.cover = right,
                    "Engine" => self.engine = right,
                    "Setup" => self.setup = right,
                    "Runtime" => self.runtime = right,
                    "Hints" => self.hints = right,
                    "Year" => self.year = right,
                    "Dev" => self.dev = right,
                    "Pub" => self.publi = right,
                    "Version" => self.version = right,
                    "Status" => self.status = right,
                    _ => panic!("unknown single item field: unable to set"),
                };
            }
            Field::MultipleItems(left, right) => {
                match left.as_str() {
                    "Store" => self.store = right,
                    "Tags" => self.tags = right,
                    "Genre" => self.genres = right,
                    _ => panic!("unknown multiple item field: unable to set"),
                };
            }
        };
    }
    /// Returns the tag vector of the game.
    fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    /// Returns the tag vector of the game.
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
        let field = Field::NewGame("Test".to_string());
        game.update(field);
        assert_eq!(game.name, "Test".to_string());
    }
    #[test]
    fn game_update_cover() {
        let mut game = Game::new();
        let field = Field::SingleItem("Cover".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.cover, "Test".to_string());
    }
    #[test]
    fn game_update_engine() {
        let mut game = Game::new();
        let field = Field::SingleItem("Engine".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.engine, "Test".to_string());
    }
    #[test]
    fn game_update_setup() {
        let mut game = Game::new();
        let field = Field::SingleItem("Setup".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.setup, "Test".to_string());
    }
    #[test]
    fn game_update_runtime() {
        let mut game = Game::new();
        let field = Field::SingleItem("Runtime".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.runtime, "Test".to_string());
    }
    #[test]
    fn game_update_hints() {
        let mut game = Game::new();
        let field = Field::SingleItem("Hints".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.hints, "Test".to_string());
    }
    #[test]
    fn game_update_year() {
        let mut game = Game::new();
        let field = Field::SingleItem("Year".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.year, "Test".to_string());
    }
    #[test]
    fn game_update_dev() {
        let mut game = Game::new();
        let field = Field::SingleItem("Dev".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.dev, "Test".to_string());
    }
    #[test]
    fn game_update_publi() {
        let mut game = Game::new();
        let field = Field::SingleItem("Pub".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.publi, "Test".to_string());
    }
    #[test]
    fn game_update_version() {
        let mut game = Game::new();
        let field = Field::SingleItem("Version".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.version, "Test".to_string());
    }
    #[test]
    fn game_update_status() {
        let mut game = Game::new();
        let field = Field::SingleItem("Status".to_string(), "Test".to_string());
        game.update(field);
        assert_eq!(game.status, "Test".to_string());
    }
    #[test]
    #[should_panic]
    fn game_single_panic() {
        let mut game = Game::new();
        let field = Field::SingleItem("Panic".to_string(), "Test".to_string());
        game.update(field);
    }
    #[test]
    fn game_update_store() {
        let mut game = Game::new();
        let field = Field::MultipleItems(
            "Store".to_string(),
            vec!["ST1".to_string(), "ST2".to_string()],
        );
        game.update(field);
        assert_eq!(game.store, vec!["ST1".to_string(), "ST2".to_string()]);
    }
    #[test]
    fn game_update_tags() {
        let mut game = Game::new();
        let field = Field::MultipleItems(
            "Tags".to_string(),
            vec!["Tag1".to_string(), "Tag2".to_string()],
        );
        game.update(field);
        assert_eq!(game.tags, vec!["Tag1".to_string(), "Tag2".to_string()]);
    }
    #[test]
    fn game_update_genres() {
        let mut game = Game::new();
        let field = Field::MultipleItems(
            "Genre".to_string(),
            vec!["Ge1".to_string(), "Ge2".to_string()],
        );
        game.update(field);
        assert_eq!(game.genres, vec!["Ge1".to_string(), "Ge2".to_string()]);
    }
    #[test]
    #[should_panic]
    fn game_multiple_panic() {
        let mut game = Game::new();
        let field = Field::MultipleItems(
            "Panic".to_string(),
            vec!["Ge1".to_string(), "Ge2".to_string()],
        );
        game.update(field);
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
