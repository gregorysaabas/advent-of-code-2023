use crate::part1_lib::{find_winning_range, Race};

pub fn part2(input:&str) ->u64{
    let race = parse_input(input);
    let winning_range = find_winning_range(&race);
    winning_range.count() as u64
}

fn parse_input(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();

    let time = lines[0]
        .chars()
        .filter(|c| !c.is_whitespace())
        .skip_while(|c| *c != ':')
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let distance = lines[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .skip_while(|c| *c != ':')
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    Race { time, distance }
}