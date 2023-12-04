use crate::part1_lib::{get_num_matching, parse_input};

pub fn part2(input: &str) -> i32 {
    let games = parse_input(input);
    let scores: Vec<i32> = games.iter().map(get_num_matching).map(|x| x as i32).collect();
    let card_instances: Vec<i32> = (1..scores.len())
        .fold(vec![1],
              |mut acc, index| {
                  let num_copies = scores[..index].iter().enumerate()
                      .filter(|(idx, score)| is_copied(index, *idx, **score))
                      .map(|(idx, _)| acc[idx])
                      .sum::<i32>();
                  acc.push(1 + num_copies);
                  acc
              });
    card_instances.iter().sum::<i32>()
}

fn is_copied(index: usize, idx: usize, score: i32) -> bool {
    score >= (index - idx) as i32
}
