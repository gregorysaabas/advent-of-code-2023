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
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(input);
        assert_eq!(result, 71503);
    }
}
