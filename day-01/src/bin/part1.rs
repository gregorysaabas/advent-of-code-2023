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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }
}
