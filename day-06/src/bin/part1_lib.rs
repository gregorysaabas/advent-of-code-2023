use std::ops::RangeInclusive;

pub fn part1(input:&str) ->u64{
    let races = parse_input(input);
    let winning_moves_per_race:Vec<u64> = races.iter()
        .map(find_winning_range)
        .map(|r| r.count() as u64)
        .collect();
    winning_moves_per_race.iter().product()
}

pub fn find_winning_range(race:&Race)-> RangeInclusive<u64>{
    let start = (0..=race.time)
        .find(|t| get_distance(&race.time, t) > race.distance)
        .unwrap();

    let end = (0..=race.time).rev()
        .find(|t| get_distance(&race.time, t) > race.distance)
        .unwrap();

    start..=end
}

fn get_distance(time: &u64, t: &u64) -> u64 {
    t * (time - t)
}

pub struct Race{
    pub time:u64,
    pub distance:u64,
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    if lines.len() == 2 {
        let time_values: Vec<u64> = lines[0]
            .split_whitespace()
            .skip(1) // Skip the first element (the label "Time:")
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        let distance_values: Vec<u64> = lines[1]
            .split_whitespace()
            .skip(1) // Skip the first element (the label "Distance:")
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        for (&time, &distance) in time_values.iter().zip(distance_values.iter()) {
            races.push(Race { time, distance });
        }
    }

    races
}