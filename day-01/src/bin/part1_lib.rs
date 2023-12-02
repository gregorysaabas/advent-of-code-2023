pub fn part1(input: &str) -> i32 {
    let line_values: Vec<i32> = input.lines()
        .map(get_line_value)
        .collect();
    line_values.iter().sum()
}

pub fn get_line_value(line: &str) -> i32 {
    let numbers: Vec<i32> = line.chars()
        .filter_map(|c| c.to_digit(10).map(|digit| digit as i32))
        .collect();
    numbers.first().unwrap() * 10 + numbers.last().unwrap()
}

