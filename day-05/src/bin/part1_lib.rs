pub fn part1(input: &str) -> u64 {
    let almanac = parse_almanac(input);
    let seeds: Vec<SeedRange> = almanac.seeds.iter()
        .flat_map(|seed| [SeedRange { start: seed.start, length: 1 }, SeedRange { start: seed.length, length: 1 }])
        .collect();
    convert_seeds_to_locations(&almanac.maps, &seeds)
}

pub fn convert_seeds_to_locations(maps: &[Vec<MapRangeDefinition>], seeds: &[SeedRange]) -> u64 {
    seeds.iter().enumerate()
        .flat_map(|(rindex, seed_range)| {
            println!("SeedRange {}/{}", rindex + 1, seeds.len());
            (seed_range.start..seed_range.start + seed_range.length).enumerate()
                .map(move |(sindex, seed)| {
                    if sindex%1000000 ==0{println!("SeedRange {}/{} Seed {}mil/{}mil", rindex+1, seeds.len(), sindex/1000000`+1, seed_range.length/1000000);};
                    maps.iter()
                    .fold(seed, |acc, cur| process_map(acc, cur))})
        })
        .min()
        .unwrap()
}

pub fn process_map(input: u64, map_definition: &[MapRangeDefinition]) -> u64 {
    map_definition.iter()
        .find(|range| range.source_start <= input && input < range.source_start + range.length)
        .map_or(input, |range| input - range.source_start + range.destination_start)
}

#[derive(Clone)]
pub struct SeedRange {
    pub start: u64,
    pub length: u64,
}

#[derive(Clone)]
pub struct MapRangeDefinition {
    pub destination_start: u64,
    pub source_start: u64,
    pub length: u64,
}

#[derive(Clone)]
pub struct Almanac {
    pub seeds: Vec<SeedRange>,
    pub maps: Vec<Vec<MapRangeDefinition>>,
}


pub fn parse_almanac(input: &str) -> Almanac {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds: Vec<u64> = seeds_line
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seeds_structs: Vec<SeedRange> = seeds
        .chunks_exact(2)
        .map(|chunk| SeedRange {
            start: chunk[0],
            length: chunk[1],
        })
        .collect();

    let mut maps = Vec::new();
    while let Some(map_name_line) = lines.next() {
        if map_name_line.contains("map") {
            let map_name = map_name_line.trim_matches(':').trim();
            let map_definition: Vec<MapRangeDefinition> = lines
                .by_ref()
                .take_while(|line| !line.trim().is_empty())
                .map(|line| {
                    let values: Vec<u64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                    MapRangeDefinition {
                        destination_start: values[0],
                        source_start: values[1],
                        length: values[2],
                    }
                })
                .collect();
            maps.push(map_definition);
        }
    }

    Almanac {
        seeds: seeds_structs,
        maps,
    }
}