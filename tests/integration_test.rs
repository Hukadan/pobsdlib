extern crate pobsdlib;
use pobsdlib::collections::DataBase;

#[test]
fn test_game_get_by_id() {
    let db_game = DataBase::new("tests/data/test-games.db");
    match db_game.games.get_item_by_id(2) {
        Some(game) => {
            assert_eq!(game.name, "The Adventures of Shuggy".to_string());
            assert_eq!(
                game.store,
                [
                    "https://store.steampowered.com/app/211440/Adventures_of_Shuggy/".to_string(),
                    "https://www.gog.com/game/the_adventures_of_shuggy".to_string()
                ]
            );
        }
        None => panic!("Game not found"),
    }
}
#[test]
fn test_game_get_by_name() {
    let db_game = DataBase::new("tests/data/test-games.db");
    match db_game.games.get_item_by_name("Akane the Kunoichi") {
        Some(game) => {
            assert_eq!(game.id, 6);
            assert_eq!(game.engine, "XNA".to_string());
        }
        None => panic!("Game not found"),
    }
}
#[test]
fn test_game_get_by_tag() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let games = db_game.get_games_by_tag("indie");
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "The Adventures of Shuggy".to_string());
    assert_eq!(games[1].name, "Aeternum".to_string());
}
#[test]
fn test_game_get_by_genre() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let games = db_game.get_games_by_genre("RPG");
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "Aedemphia".to_string());
    assert_eq!(games[1].name, "Always Sometimes Monsters".to_string());
}
