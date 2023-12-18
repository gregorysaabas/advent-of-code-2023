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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part2(input);
        assert_eq!(result, 6);
    }
}
