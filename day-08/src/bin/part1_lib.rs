pub fn part1(input:&str) -> u32{
    let desert_map = parse_input(input);

    let mut cur_node = &desert_map.nodes["AAA"];
    let mut steps = 0u32;
    while cur_node.name.ne("ZZZ"){
        let dir = &desert_map.instructions[(steps as usize) % desert_map.instructions.len()];
        cur_node = match dir {
            Direction::Left=> &desert_map.nodes[&cur_node.left],
            Direction::Right=> &desert_map.nodes[&cur_node.right],
        };
        steps+=1;
    }
    steps

}
use std::collections::HashMap;
use scan_fmt::scan_fmt;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug)]
pub struct DesertMap {
    pub instructions: Vec<Direction>,
    pub nodes: HashMap<String, Node>,
}

pub fn parse_input(input: &str) -> DesertMap {
    let mut lines = input.lines();

    let instructions_line = lines.next().unwrap();
    let instructions:Vec<Direction> = instructions_line
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        })
        .collect();

    let nodes:HashMap<String, Node> =lines.skip(1)
        .map(|line|{
            let (name, left, right) = scan_fmt!(line, "{} = ({}, {})", String, String, String).ok().unwrap();
            let node = Node {
                name:name.clone(),
                left,
                right,
            };
            (name, node)
        })
        .collect();

    DesertMap{
        instructions,
        nodes
    }
}