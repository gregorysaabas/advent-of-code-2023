mod part1_lib;
use part1_lib::part1;
fn main() {
    println!("Hello, part1!");
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }
}
