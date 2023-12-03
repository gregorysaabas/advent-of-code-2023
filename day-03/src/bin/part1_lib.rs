use std::collections::HashSet;

pub fn part1(input:&str) -> i32{
    let (numbers, symbols) = parse_input(input);
    let symbol_adjacent = get_symbol_adjacent(symbols);
    let adjacent_numbers :Vec<PartNumber> = numbers.into_iter()
        .filter(|number| number.coordinates.iter()
            .any(|coordinate| symbol_adjacent.contains(coordinate)) )
        .collect();
    adjacent_numbers.iter()
        .map(|number| number.value)
        .sum()
}

fn get_symbol_adjacent( symbols:Vec<Symbol>) -> HashSet<Coordinate>{
    symbols.iter()
        .map(|symbol| symbol.coordinate)
        .flat_map(get_adjacent)
        .collect()
}

pub fn get_adjacent(coordinate: Coordinate) -> Vec<Coordinate>{
    (-1i32..=1)
        .flat_map(|row| (-1i32..=1)
            .map(move |col| (row,col)))
        .map(|(row,col)| Coordinate{
            row: coordinate.row+row,
            col: coordinate.col+col,
        }).collect()
}

pub fn parse_input(input:&str) -> (Vec<PartNumber>, Vec<Symbol>){
    let (numbers, symbols): (Vec<PartNumber>, Vec<Symbol>) = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().fold(
                (Vec::new(), Vec::new(), None, 0),
                |(mut numbers, mut symbols, mut current_coordinates, mut current_value), (col, char)| {
                    let (row, col) = (row as i32, col as i32);
                    if let Some(digit_value) = char.to_digit(10) {
                        let coordinate = Coordinate { row, col };
                        current_coordinates.get_or_insert(Vec::new()).push(coordinate);
                        current_value = current_value*10+ digit_value as i32;
                    } else if let Some(coordinates) = current_coordinates.take() {
                        let value = current_value;
                        let part_number = PartNumber {
                            value,
                            coordinates,
                        };
                        current_coordinates = None;
                        current_value = 0;
                        numbers.push(part_number);
                    }

                    if !char.is_digit(10) && char != '.' {
                        let coordinate = Coordinate { row, col };
                        symbols.push(Symbol { value:char, coordinate });
                    }

                    (numbers, symbols, current_coordinates, current_value)
                },
            )
        })
        .map(|(mut n, s, cc, value)| {
            if let Some(coordinates) = cc {
                n.push(PartNumber{value,coordinates});}
            (n,s)})
        .fold((Vec::new(), Vec::new()), |(mut numbers, mut symbols), (n, s)| {
            numbers.extend(n);
            symbols.extend(s);
            (numbers, symbols)
        });

    (numbers, symbols)
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