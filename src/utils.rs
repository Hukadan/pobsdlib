use crate::collections::ItemCollection;
use crate::models::{Game, GameTraits, ItemTraits, Line};
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

pub fn game_dispatch<T: Default + ItemTraits + GameTraits>(
    line: Line,
    games: &mut ItemCollection<T>,
) {
    match line {
        Line::NewGame(_) => {
            let mut game = T::default();
            game.set_id(games.count + 1);
            game.update(line);
            games.add_item(game);
        }
        Line::SingleItem(_, _) | Line::MultipleItems(_, _) => {
            if let Some(game) = games.items.last_mut() {
                game.update(line)
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
            game_dispatch(Line::from(&line), games);
        }
    }
}
