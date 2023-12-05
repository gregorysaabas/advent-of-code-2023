use crate::part1_lib::{convert_seeds_to_locations, parse_almanac};
pub fn part2(input:&str) -> u64{
    let almanac = parse_almanac(input);

    convert_seeds_to_locations(&almanac.maps, &almanac.seeds)
}