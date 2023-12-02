use std::collections::HashMap;

pub fn part2(input: &str) -> i32 {
    let string_to_i32_str_map: HashMap<&str, i32> = HashMap::from(
        [
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);

    let get_line_value_with_map = |line| get_line_value(line, &string_to_i32_str_map);

    let line_values: Vec<i32> = input.lines()
        .map(get_line_value_with_map)
        .collect();

    line_values.iter().sum()
}

fn get_line_value(line: &str, map: &HashMap<&str, i32>) -> i32 {
    let (first_str, last_str) = find_first_occurrence(line, map.keys().map(|x| x.to_string()).collect());

    let first = map.get(&*first_str).unwrap();
    let last = map.get(&*last_str).unwrap();
    first * 10 + last
}

fn find_first_occurrence(search_string: &str, substrings: Vec<String>) -> (String, String) {
    let substring_with_occurrences: Vec<(String, Option<usize>, Option<usize>)> = substrings
        .iter()
        .map(|substring| (substring.clone(), search_string.find(substring), search_string.rfind(substring)))
        .collect();
    let (min_str, _, _): (String, _, _) = substring_with_occurrences.iter()
        .filter(|(_, min_index, _)| min_index.is_some())
        .min_by_key(|(_, min_index, _)| min_index.unwrap()).unwrap().clone();
    let (max_str, _, _) = substring_with_occurrences.iter()
        .filter(|(_, _, max_index)| max_index.is_some())
        .max_by_key(|(_, _, max_index)| max_index.unwrap()).unwrap().clone();
    (min_str.clone(), max_str.clone())
}