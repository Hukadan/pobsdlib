use crate::utils::split_line;

/* ------------------------- TRAITS --------------------------*/
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

/* ------------------------ FIELD ENUM -----------------------*/
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
/// assert_eq!(field,Field::NewGame(&"Name of the game"));
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
/// assert_eq!(field,Field::SingleItem(&"Engine",&"Engine name"));
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
/// assert_eq!(field,Field::MultipleItems(&"Tags",vec![&"tag1",&"tag2"]));
/// ```
/// Note that while Tags and Genres are coma separated values, Stores are space separated ones.
/// This is handled by the `Field::from` method.
#[derive(PartialEq, Debug)]
pub enum Field<'a> {
    NewGame(&'a str),
    SingleItem(&'a str, &'a str),
    MultipleItems(&'a str, Vec<&'a str>),
}

impl<'a> Field<'a> {
    /// Try to convert a line of the database in a Field enum. Panic if it cannot.
    pub fn from(line: &'a str) -> Self {
        // split the line in a left and right hand sides
        let (left, right) = split_line(line);
        // use the left hand side to discriminate between single and multiple item lines
        match left {
            "Game" => Field::NewGame(right),
            "Cover" | "Engine" | "Setup" | "Runtime" | "Hints" | "Year" | "Dev" | "Pub"
            | "Version" | "Status" => Field::SingleItem(left, right),
            "Store" => {
                let mut items: Vec<&str> = Vec::new();
                for item in right.split(' ') {
                    items.push(item.trim());
                }
                Field::MultipleItems(left, items)
            }
            "Genre" | "Tags" => {
                let mut items: Vec<&str> = Vec::new();
                for item in right.split(',') {
                    items.push(item.trim());
                }
                Field::MultipleItems(left, items)
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
    pub fn as_line(&'a self) -> String {
        match self {
            Field::NewGame(name) => vec!["Game", name].join("\t"),
            Field::SingleItem(left, right) => vec![left.to_owned(), right].join("\t"),
            Field::MultipleItems(left, right) => {
                if left.eq(&"Store") {
                    vec![left.to_owned(), right.join(" ").as_str()].join("\t")
                } else {
                    vec![left.to_owned(), right.join(", ").as_str()].join("\t")
                }
            }
        }
    }
}

/* ------------------------ ITEM STRUCT -------------------------*/
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

impl ItemTraits for &Item {
    /// Sets the id of the item.
    fn set_id(&mut self, id: usize) {
        self.set_id(id);
    }
    /// Returns the name of the item.
    fn get_name(&self) -> &str {
        &self.name
    }
}

/* ------------------------ GAME STRUCT -------------------------*/
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
    pub fn get_field(&self, name: &str) -> Field {
        match name {
            "Game" => Field::NewGame(&self.name),
            "Cover" => Field::SingleItem(&"Cover", &self.cover),
            "Engine" => Field::SingleItem(&"Engine", &self.engine),
            "Setup" => Field::SingleItem(&"Setup", &self.setup),
            "Runtime" => Field::SingleItem(&"Runtime", &self.runtime),
            "Hints" => Field::SingleItem(&"Hints", &self.hints),
            "Year" => Field::SingleItem(&"Year", &self.year),
            "Dev" => Field::SingleItem(&"Dev", &self.dev),
            "Pub" => Field::SingleItem(&"Pub", &self.publi),
            "Version" => Field::SingleItem(&"Version", &self.version),
            "Status" => Field::SingleItem(&"Status", &self.status),
            "Store" => {
                let mut stores: Vec<&str> = Vec::new();
                for store in &self.store {
                    stores.push(store);
                }
                Field::MultipleItems(&"Store", stores)
            }
            "Genre" => {
                let mut genres: Vec<&str> = Vec::new();
                for genre in &self.genres {
                    genres.push(genre);
                }
                Field::MultipleItems(&"Genre", genres)
            }
            "Tags" => {
                let mut tags: Vec<&str> = Vec::new();
                for tag in &self.tags {
                    tags.push(tag);
                }
                Field::MultipleItems(&"Tags", tags)
            }
            _ => panic!("Unkown filed {}", name),
        }
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

impl ItemTraits for &Game {
    /// Sets the id of the game.
    fn set_id(&mut self, id: usize) {
        self.set_id(id);
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
            Field::NewGame(name) => self.name = name.to_string(),
            Field::SingleItem(left, right) => {
                match left {
                    "Cover" => self.cover = right.to_string(),
                    "Engine" => self.engine = right.to_string(),
                    "Setup" => self.setup = right.to_string(),
                    "Runtime" => self.runtime = right.to_string(),
                    "Hints" => self.hints = right.to_string(),
                    "Year" => self.year = right.to_string(),
                    "Dev" => self.dev = right.to_string(),
                    "Pub" => self.publi = right.to_string(),
                    "Version" => self.version = right.to_string(),
                    "Status" => self.status = right.to_string(),
                    _ => panic!("unknown single item field: unable to set"),
                };
            }
            Field::MultipleItems(left, right) => {
                match left {
                    "Store" => {
                        let mut stores: Vec<String> = Vec::new();
                        for store in right {
                            stores.push(store.to_string());
                        }
                        self.store = stores;
                    }
                    "Tags" => {
                        let mut tags: Vec<String> = Vec::new();
                        for tag in right {
                            tags.push(tag.to_string());
                        }
                        self.tags = tags;
                    }
                    "Genre" => {
                        let mut genres: Vec<String> = Vec::new();
                        for genre in right {
                            genres.push(genre.to_string());
                        }
                        self.genres = genres;
                    }
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

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod test_field_methods {
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
    fn from_game_line() {
        let input = "Game\tToto";
        let field = Field::from(&input);
        assert!(Field::NewGame(&"Toto") == field);
    }
    #[test]
    fn from_single_line() {
        let input = "Cover\tToto";
        let field = Field::from(&input);
        assert!(Field::SingleItem(&"Cover", &"Toto") == field);
    }
    #[test]
    fn from_mutilple_line() {
        let input = "Genre\tfirst, second";
        let field = Field::from(&input);
        assert!(Field::MultipleItems(&"Genre", vec![&"first", &"second"]) == field);
    }
    #[test]
    #[should_panic]
    fn from_malformed_line() {
        let input = "Let's panic";
        Field::from(&input);
    }
}

#[cfg(test)]
mod test_item_methods {
    use super::*;
    #[test]
    fn new() {
        let item = Item::new();
        let item_bis = Item::default();
        assert!(item == item_bis);
    }
    #[test]
    fn set_id() {
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
    fn get_name() {
        let item = Item {
            id: 2,
            name: "toto".to_string(),
            games: Vec::new(),
        };
        assert_eq!(item.get_name(), "toto");
    }
}

#[cfg(test)]
mod test_game_methods {
    use super::*;
    #[test]
    fn new() {
        let game = Game::new();
        let game_bis = Game::default();
        assert!(game == game_bis);
    }
    #[test]
    fn get_engine() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Engine", &"Test");
        game.update(field);
        let field = game.get_field("Engine");
        assert_eq!(Field::SingleItem(&"Engine", &"Test"), field);
    }
    #[test]
    fn get_store() {
        let mut game = Game::new();
        let field = Field::MultipleItems(&"Store", vec![&"ST1", &"ST2"]);
        game.update(field);
        let field = game.get_field("Store");
        assert_eq!(Field::MultipleItems(&"Store", vec![&"ST1", &"ST2"]), field);
    }
    #[test]
    fn set_id() {
        let mut game = Game::new();
        game.set_id(2);
        let mut game_bis = Game::new();
        game_bis.id = 2;
        assert!(game == game_bis);
    }
    #[test]
    fn get_name() {
        let mut game = Game::new();
        game.name = "toto".to_string();
        assert_eq!(game.get_name(), "toto");
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
    #[test]
    fn update_from_name() {
        let mut game = Game::new();
        let field = Field::NewGame(&"Test");
        game.update(field);
        assert_eq!(game.name, "Test".to_string());
    }
    #[test]
    fn update_from_cover() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Cover", &"Test");
        game.update(field);
        assert_eq!(game.cover, "Test".to_string());
    }
    #[test]
    fn update_from_engine() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Engine", &"Test");
        game.update(field);
        assert_eq!(game.engine, "Test".to_string());
    }
    #[test]
    fn update_from_setup() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Setup", &"Test");
        game.update(field);
        assert_eq!(game.setup, "Test".to_string());
    }
    #[test]
    fn update_from_runtime() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Runtime", &"Test");
        game.update(field);
        assert_eq!(game.runtime, "Test".to_string());
    }
    #[test]
    fn update_from_hints() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Hints", &"Test");
        game.update(field);
        assert_eq!(game.hints, "Test".to_string());
    }
    #[test]
    fn update_from_year() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Year", &"Test");
        game.update(field);
        assert_eq!(game.year, "Test".to_string());
    }
    #[test]
    fn update_from_dev() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Dev", &"Test");
        game.update(field);
        assert_eq!(game.dev, "Test".to_string());
    }
    #[test]
    fn update_from_publi() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Pub", &"Test");
        game.update(field);
        assert_eq!(game.publi, "Test".to_string());
    }
    #[test]
    fn update_from_version() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Version", &"Test");
        game.update(field);
        assert_eq!(game.version, "Test".to_string());
    }
    #[test]
    fn update_from_status() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Status", &"Test");
        game.update(field);
        assert_eq!(game.status, "Test".to_string());
    }
    #[test]
    #[should_panic]
    fn update_from_malformed_singleitemfield() {
        let mut game = Game::new();
        let field = Field::SingleItem(&"Panic", &"Test");
        game.update(field);
    }
    #[test]
    fn update_from_store() {
        let mut game = Game::new();
        let field = Field::MultipleItems(&"Store", vec![&"ST1", &"ST2"]);
        game.update(field);
        assert_eq!(game.store, vec!["ST1".to_string(), "ST2".to_string()]);
    }
    #[test]
    fn update_from_tags() {
        let mut game = Game::new();
        let field = Field::MultipleItems(&"Tags", vec![&"Tag1", &"Tag2"]);
        game.update(field);
        assert_eq!(game.tags, vec!["Tag1".to_string(), "Tag2".to_string()]);
    }
    #[test]
    fn update_from_genres() {
        let mut game = Game::new();
        let field = Field::MultipleItems(&"Genre", vec![&"Ge1", &"Ge2"]);
        game.update(field);
        assert_eq!(game.genres, vec!["Ge1".to_string(), "Ge2".to_string()]);
    }
    #[test]
    #[should_panic]
    fn update_from_malformed_multipleitemsfield() {
        let mut game = Game::new();
        let field = Field::MultipleItems(&"Panic", vec![&"Ge1", &"Ge2"]);
        game.update(field);
    }
}
