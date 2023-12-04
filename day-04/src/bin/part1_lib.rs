pub fn part1(input:&str)->i32{
    let games = parse_input(input);
    let game_scores:Vec<i32> = games.iter()
        .map(calculate_points)
        .collect();
    game_scores.iter().sum()
}

fn calculate_points(game:&Game) -> i32{
    let num_matching:u32 = get_num_matching(game);
    if num_matching >0{
        2_i32.pow(num_matching - 1)
    } else{
        0
    }
}

pub fn get_num_matching(game: &Game) -> u32 {
    game.right_numbers.iter()
        .filter(|number| game.left_numbers.contains(number))
        .count() as u32
}

#[derive(Debug)]
pub struct Game {
    pub left_numbers: Vec<u32>,
    pub right_numbers: Vec<u32>,
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        let card_info = parts[1].trim();
        let sides: Vec<&str> = card_info.split('|').collect();

        let left_numbers: Vec<u32> = sides[0]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let right_numbers: Vec<u32> = sides[1]
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        Game {
            left_numbers,
            right_numbers,
        }
    }).collect()
}