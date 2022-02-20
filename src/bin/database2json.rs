extern crate pobsdlib;
extern crate serde_json;
use pobsdlib::collections::DataBase;
use std::{env, path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }
    if args.len() > 2 {
        eprintln!("Too many arguments");
        process::exit(1);
    }
    let path = path::Path::new(&args[1]);
    if path.is_file() {
        let db_game = DataBase::new(&args[1]);
        let json_games = serde_json::to_string_pretty(&db_game.games).unwrap();
        println!("{}", json_games);
    } else {
        eprintln!("This is not a file");
    }
}
