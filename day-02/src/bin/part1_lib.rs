use std::collections::HashMap;

pub fn part1(input:&str)->i32{
    let games = parse_input(input);

    let games_possible:Vec<i32> = games.iter().filter_map(|(game_id, game)|
        if is_possible_game(game) {
            Some(*game_id as i32)
        } else {
            None
        }).collect();
    games_possible.iter().sum()
}

fn is_possible_game(game:&GameInfo) -> bool{
    let is_possible = game.balls.iter().all(is_possible_balls);
    is_possible
}

fn is_possible_balls(ball_info: &BallInfo) -> bool{
    let is_impossible = ball_info.red > 12
    || ball_info.green > 13
    || ball_info.blue > 14;
    !is_impossible
}

#[derive(Debug)]
pub struct BallInfo {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

#[derive(Debug)]
pub struct GameInfo {
    pub balls: Vec<BallInfo>,
}

pub fn parse_input(input: &str) -> HashMap<usize, GameInfo> {
    input.lines().filter_map(parse_game).collect()
}

fn parse_game(line: &str) -> Option<(usize, GameInfo)> {
    let mut parts = line.splitn(2, ": ");
    let (game_number, balls_str) = (parts.next()?, parts.next()?);

    let game_number = game_number.trim().strip_prefix("Game ")?;
    let game_number: usize = game_number.parse().ok()?;

    let balls = parse_balls(balls_str);
    Some((game_number, GameInfo { balls }))
}

fn parse_balls(balls_str: &str) -> Vec<BallInfo> {
    balls_str
        .split(';')
        .map(|game_balls| {
            game_balls
                .split(',')
                .map(|ball| {
                    let mut parts = ball.trim().splitn(2, ' ');
                    let count: usize = parts.next().unwrap_or_default().parse().unwrap_or_default();
                    let color = parts.next().unwrap_or_default();
                    let color = color.trim();

                    match color {
                        "red" => BallInfo { red: count, green: 0, blue: 0 },
                        "green" => BallInfo { red: 0, green: count, blue: 0 },
                        "blue" => BallInfo { red: 0, green: 0, blue: count },
                        _ => BallInfo { red: 0, green: 0, blue: 0 },
                    }
                })
                .fold(BallInfo { red: 0, green: 0, blue: 0 }, |acc, ball| {
                    BallInfo {
                        red: acc.red + ball.red,
                        green: acc.green + ball.green,
                        blue: acc.blue + ball.blue,
                    }
                })
        })
        .collect()
}