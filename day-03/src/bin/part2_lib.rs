use super::part1_lib::{get_adjacent, parse_input, PartNumber, Symbol};

pub fn part2(input: &str) -> i32 {
    let (numbers, symbols) = parse_input(input);

    let gear_ratios: Vec<i32> = symbols.into_iter()
        .filter(|symbol| symbol.value.eq(&'*'))
        .map(|symbol| get_adjacent_numbers(symbol, &numbers))
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers.iter()
            .map(|number| number.value)
            .product::<i32>())
        .collect();

    gear_ratios.iter().sum()
}

fn get_adjacent_numbers(symbol: Symbol, numbers: &[PartNumber]) -> Vec<PartNumber> {
    let adjacent = get_adjacent(symbol.coordinate);
    numbers.iter()
        .filter(|number| number.coordinates.iter().any(|coordinate| adjacent.contains(coordinate)))
        .cloned()
        .collect()
}