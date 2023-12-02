mod part2_lib;
use part2_lib::part2;

fn main() {
    println!("Hello, part2!");
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
