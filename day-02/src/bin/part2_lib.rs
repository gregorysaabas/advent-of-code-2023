use super::part1_lib::{BallInfo, GameInfo, parse_input};
pub fn part2(input:&str)-> i32{
    let games = parse_input(input);
    let game_powers:Vec<i32> = games.values().map(get_power).collect();

    game_powers.iter().sum()
}

fn get_power(game:&GameInfo) -> i32{
    let red_max = get_max_colour(&game.balls, |ball|ball.red);
    let green_max = get_max_colour(&game.balls, |ball|ball.green);
    let blue_max = get_max_colour(&game.balls, |ball|ball.blue);
    red_max*green_max*blue_max
}

fn get_max_colour<F>(balls: &[BallInfo], color_fn: F) -> i32
    where
        F: Fn(&BallInfo) -> usize,
{
    balls.iter()
        .map(|ball| color_fn(ball) as i32)
        .max()
        .unwrap_or(0)
}