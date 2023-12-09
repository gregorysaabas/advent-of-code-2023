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
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part2(input);
        assert_eq!(result, 5905);
    }
}
