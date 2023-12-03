mod part1_lib;
mod part2_lib;

use part2_lib::part2;

fn main() {
    println!("Hello, part2!");
    let input = include_str!("./input1.txt");
    let output = part2(input);
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
        let result = part2(input);
        assert_eq!(result, 467835);
    }
}
