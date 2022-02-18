extern crate pobsdlib;
use pobsdlib::DataBase;

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
