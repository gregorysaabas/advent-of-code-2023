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
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 6);
    }
}
