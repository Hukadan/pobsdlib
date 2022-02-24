extern crate pobsdlib;
extern crate serde_json;
use pobsdlib::collections::DataBase;
use std::{env, path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }
    if args.len() > 4 {
        eprintln!("Too many arguments");
        process::exit(1);
    }
    let path = path::Path::new(&args[1]);
    if path.is_file() {
        let db_game = DataBase::new(&args[1]);
        let field_name = &args[2];
        let field_value = &args[3];
        //let json_games = serde_json::to_string_pretty(&db_game.get_games_by_tag(tag)).unwrap();
        let json_games = serde_json::to_string_pretty(
            &db_game.games.get_item_with_field(field_name, field_value),
        )
        .unwrap();
        println!("{}", json_games);
    } else {
        eprintln!("This is not a file");
    }
}
