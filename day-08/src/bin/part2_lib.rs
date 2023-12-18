use std::collections::{HashMap, HashSet};
use std::iter::successors;
use std::ops::Range;
use num::integer::lcm;
use crate::part1_lib::{DesertMap, Direction, Node, parse_input};

pub fn part2(input: &str) -> u32 {
    let desert_map = parse_input(input);

    let start_nodes: Vec<String> = desert_map.nodes.keys()
        .filter_map(|name| if name.ends_with("A") { Some(name.to_owned()) } else { None })
        .collect();
    let loops: Vec<DesertLoop> = start_nodes.iter()
        .map(|node_name| find_loop(&desert_map, node_name))
        .collect();
    let max_start = loops.iter().map(|l| l.loop_start).max().unwrap();
    let max_loop_len = loops.iter().map(|l| l.loop_len())
        .reduce(lcm)
        .unwrap();
    // let pre_winner = get_winner(&loops, 0, max_start);
    // let post_winner = get_winner(&loops, max_start,max_start + max_loop_len);
    // let winner =
    //     pre_winner
    //         .or(post_winner)
    //         .unwrap();
    let winner = dumb_get_winner(&loops, max_start, max_start+ max_loop_len).unwrap();
    winner as u32
}
fn dumb_get_winner(loops: &Vec<DesertLoop>, start:usize, end:usize) -> Option<usize> {
    let subwinners:Vec<usize> = loops.iter().map(|l| get_subwinners(start, start + l.loop_len(), l).into_iter().next().unwrap())
        .collect();
    subwinners.iter().cloned().reduce(lcm)
}


fn get_winner(loops: &Vec<DesertLoop>, start:usize, end:usize) -> Option<usize> {
    let winners: Vec<(&DesertLoop,HashSet<usize>)> = loops.iter()
        .map(|l|{//if start < l.loop_start {get_subwinners(start, end, l)} else {
            let mut subwinners = get_subwinners(start, start+l.loop_len(), l);
            // let times_to_repeat = (end-start)/l.loop_len();
            // let acc:HashSet<usize> = HashSet::new();
            // (1..=times_to_repeat).fold(acc,|mut acc,ttr| {
            //the next line is wrong
            //         let copy: Vec<usize> =subwinners.iter().map(|v| *v *ttr).collect();
            //     acc.extend(copy);
            //     acc
            //     }
            // )
            (l,subwinners)
        }
)
        .collect();
    // let common_winners: HashSet<usize> = winners.into_iter()
    //     .reduce(|acc, cur| acc.intersection(&cur).cloned().collect::<HashSet<usize>>())
    //     .unwrap_or(HashSet::new()).iter()
    //     .cloned()
    //     .collect();
    // common_winners.iter()
    //     .cloned()
    //     .min()
    None
}

fn get_subwinners(start: usize, end: usize, l: &DesertLoop) -> HashSet<usize> {
    (start..end)
        .filter(|i| l.ends_in_z(*i))
        .collect::<HashSet<usize>>()
}

#[derive(Debug, Clone)]
struct DesertLoop {
    seq: Vec<String>,
    loop_start: usize,
}

impl DesertLoop {
    fn loop_len(&self) -> usize {
        self.seq.len() - self.loop_start
    }

    fn get_node(&self, index: usize) -> String {
        if index < self.seq.len() {
            self.seq[index].to_owned()
        } else {
            let extra = index - self.seq.len();
            let new_index = self.loop_start + (extra % self.loop_len());
            self.seq[new_index].to_owned()
        }
    }

    fn ends_in_z(&self, index: usize) -> bool {
        if index < self.seq.len() {
            self.seq[index].ends_with("Z")
        } else {
            let extra = index - self.seq.len();
            let new_index = self.loop_start + (extra % self.loop_len());
            self.seq[new_index].ends_with("Z")
        }
    }

}

#[derive(Debug, Clone)]
struct Accumulator {
    cur_node: String,
    cur_index: usize,
    seen: HashSet<(usize, String)>,
    seq: Vec<(usize, String)>,
}

impl Accumulator {
    fn new(start_node: &str) -> Self {
        Self {
            cur_node: start_node.to_owned(),
            cur_index: 0,
            seen: HashSet::new(),
            seq: Vec::new(),
        }
    }

    fn push(mut self, desert_map: &DesertMap) -> Self {
        self.seen.insert((self.cur_index, self.cur_node.clone()));
        self.seq.push((self.cur_index, self.cur_node.clone()));
        let node = &desert_map.nodes[&*self.cur_node];
        self.cur_node = get_next_node(desert_map, &*self.cur_node, &desert_map.instructions[self.cur_index]);
        self.cur_index = (self.cur_index + 1) % desert_map.instructions.len();
        self
    }

    fn is_looped(&self) -> bool {
        self.seen.contains(&(self.cur_index, self.cur_node.to_owned()))
    }

    fn get_desert_loop(&self) -> DesertLoop {
        let loop_start = self.seq.iter().enumerate()
            .find_map(|(i, (index, node))|
                if *index == self.cur_index && *node == self.cur_node { Some(i) } else { None })
            .unwrap();

        DesertLoop {
            seq: self.seq.iter()
                .map(|(_, node)| node)
                .cloned()
                .collect(),
            loop_start,
        }
    }
}

fn get_next_node(desert_map: &DesertMap, cur_node_name: &str, dir: &Direction) -> String {
    let cur_node = desert_map.nodes[cur_node_name].clone();
    match dir {
        Direction::Left => cur_node.left,
        Direction::Right => cur_node.right,
    }
}

fn find_loop(desert_map: &DesertMap, start: &str) -> DesertLoop {
    let mut seq: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut current = start.to_owned();

    while !seen.contains(&*current) {
        seen.insert(current.to_owned());
        current = desert_map.instructions.iter()
            .fold(current, |acc, dir| {
                    seq.push(acc.to_owned());
                get_next_node(desert_map, &acc, dir)
                }
            );
    };
    let zero_alignment_loop_start =
        (0..seq.len())
            .step_by(desert_map.instructions.len())
            .map(|i| (i, &seq[i]))
            .find(|(_,node)| node == &&current)
            .map(|(i,_)| i )
            .unwrap();
    let zero_ahead_by = (0..zero_alignment_loop_start)
        .find(|i| seq[seq.len() - 1 - i] != seq[zero_alignment_loop_start - 1 - i])
        .unwrap_or(0);
    let loop_start = zero_alignment_loop_start- zero_ahead_by;
    seq.truncate(seq.len()-zero_ahead_by);

    DesertLoop{seq,loop_start}
}