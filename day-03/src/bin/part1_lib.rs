use std::collections::HashSet;

pub fn part1(input: &str) -> i32 {
    let (numbers, symbols) = parse_input(input);
    let symbol_adjacent = get_symbol_adjacent(symbols);
    let adjacent_numbers: Vec<PartNumber> = numbers.into_iter()
        .filter(|number| number.coordinates.iter()
            .any(|coordinate| symbol_adjacent.contains(coordinate)))
        .collect();
    adjacent_numbers.iter()
        .map(|number| number.value)
        .sum()
}

fn get_symbol_adjacent(symbols: Vec<Symbol>) -> HashSet<Coordinate> {
    symbols.iter()
        .map(|symbol| symbol.coordinate)
        .flat_map(get_adjacent)
        .collect()
}

pub fn get_adjacent(coordinate: Coordinate) -> Vec<Coordinate> {
    (-1i32..=1)
        .flat_map(|row| (-1i32..=1)
            .map(move |col| (row, col)))
        .map(|(row, col)| Coordinate {
            row: coordinate.row + row,
            col: coordinate.col + col,
        }).collect()
}

pub fn parse_input(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let (numbers, symbols): (Vec<PartNumber>, Vec<Symbol>) = input
        .lines()
        .enumerate()
        .map(|(row, line)| parse_line(row, line))
        .fold((Vec::new(), Vec::new()), |(mut numbers, mut symbols), (n, s)| {
            numbers.extend(n);
            symbols.extend(s);
            (numbers, symbols)
        });
    (numbers, symbols)
}

struct Accumulator {
    coordinates: Vec<Coordinate>,
    value: i32,
}

impl Accumulator {
    fn new() -> Self {
        Self { coordinates: vec![], value: 0 }
    }
    fn push(&mut self, coordinate: Coordinate, value: u32) {
        self.coordinates.push(coordinate);
        self.value = self.value * 10 + value as i32;
    }
}

fn parse_line(row: usize, line: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let (mut numbers, symbols, mut acc) =
        line.chars().enumerate().fold(
            (Vec::new(), Vec::new(), None),
            |(mut numbers, mut symbols, mut acc), (col, char)| {
                let (row, col) = (row as i32, col as i32);
                handle_number(&mut numbers, &mut acc, char, row, col);
                handle_symbol(&mut symbols, char, row, col);
                (numbers, symbols, acc)
            },
        );
    if acc.is_some() {
        handle_number(&mut numbers, &mut acc, '.', 0, 0);
    }
    (numbers, symbols)
}

fn handle_number(numbers: &mut Vec<PartNumber>, acc: &mut Option<Accumulator>, char: char, row: i32, col: i32) {
    if let Some(digit_value) = char.to_digit(10) {
        let coordinate = Coordinate { row, col };
        acc.get_or_insert(Accumulator::new()).push(coordinate, digit_value);
    } else if let Some(ac) = acc.take() {
        let part_number = PartNumber {
            value: ac.value,
            coordinates: ac.coordinates,
        };
        numbers.push(part_number);
    }
}

fn handle_symbol(symbols: &mut Vec<Symbol>, char: char, row: i32, col: i32) {
    if !char.is_ascii_digit() && char != '.' {
        let coordinate = Coordinate { row, col };
        symbols.push(Symbol { value: char, coordinate });
    }
}

#[derive(Debug, Clone)]
pub struct PartNumber {
    pub value: i32,
    pub coordinates: Vec<Coordinate>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub value: char,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}