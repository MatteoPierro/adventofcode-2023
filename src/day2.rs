use regex::{Error, Regex};
use crate::input_reader::read_lines;

#[derive(Debug, Clone, PartialEq)]
struct Game {
    id: usize,
    extractions: Vec<Extraction>
}

impl Game {
    fn parse_games(raw_games: Vec<String>) -> Vec<Game> {
        raw_games.iter().map(|raw_game| Game::parse_game(raw_game)).collect()
    }
    fn parse_game(raw_game: &str) -> Game {
        let parts: Vec<&str> = raw_game.split(": ").collect();
        let game_with_id = *parts.first().unwrap();
        let id = parse_game_id(game_with_id);
        let raw_extractions = *parts.last().unwrap();
        let extractions = Extraction::parse_extractions(raw_extractions);
        Game { id, extractions }
    }

    fn is_possible(&self) -> bool {
        self.extractions.iter().all(|e| e.is_possible())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Extraction {
    red: Option<usize>,
    blue: Option<usize>,
    green: Option<usize>,
}

impl Extraction {
    fn parse_extraction(raw_extraction: &str) -> Self {
        let blue = extract_color(Regex::new(r"(\d+) blue"), raw_extraction);
        let green = extract_color(Regex::new(r"(\d+) green"), raw_extraction);
        let red = extract_color(Regex::new(r"(\d+) red"), raw_extraction);
        Extraction { red, blue, green }
    }

    fn parse_extractions(raw_extractions: &str) -> Vec<Self> {
        raw_extractions.split("; ")
            .collect::<Vec<_>>().iter()
            .map(|e| Extraction::parse_extraction(e))
            .collect()
    }

    fn is_possible(&self) -> bool {
        self.is_possible_blue() && self.is_possible_green() && self.is_possible_red()
    }

    fn is_possible_blue(&self) -> bool{
        is_possible_color(self.blue, 14)
    }

    fn is_possible_green(&self) -> bool{
        is_possible_color(self.green, 13)
    }

    fn is_possible_red(&self) -> bool{
        is_possible_color(self.red, 12)
    }
}

fn is_possible_color(color: Option<usize>, expected: usize) -> bool {
    if let Some(v) = color {
        return v <= expected;
    }

    true
}

fn parse_game_id(game_with_id: &str) -> usize {
    Regex::new(r"Game (\d+)").unwrap()
        .captures(game_with_id)
        .map(|c| c[1].parse::<usize>().unwrap())
        .unwrap()
}

fn extract_color(color_regex: Result<Regex, Error>, raw_extraction: &str) -> Option<usize> {
    color_regex.unwrap().captures(raw_extraction).map(|c| c[1].parse::<usize>().unwrap())
}

fn calculate_possible_games_ids_sum(lines: &str) -> usize {
    Game::parse_games(read_lines(lines)).iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day2::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day02.txt");
        assert_eq!(2528, calculate_possible_games_ids_sum(&input));
    }

    #[test]
    fn it_calculates_possible_games_ids_sum() {
        let input = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

        assert_eq!(8, calculate_possible_games_ids_sum(input));
    }

    #[test]
    fn it_finds_if_a_game_is_possible() {
        assert!(Game::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").is_possible());
        assert!(Game::parse_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").is_possible());
        assert_eq!(false, Game::parse_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").is_possible());
        assert_eq!(false, Game::parse_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").is_possible());
        assert!(Game::parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").is_possible());
    }

    #[test]
    fn it_parses_a_game() {
        assert_eq!(
            Game {
                id: 1,
                extractions: vec![
                    Extraction { red: Some(4), green: None, blue: Some(3) },
                    Extraction { red: Some(1), blue: Some(6), green: Some(2) },
                    Extraction { red: None, green: Some(2), blue: None },
                ]
            }, Game::parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
    }
}