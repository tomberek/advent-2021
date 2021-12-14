// use text_io::scan;
// use packed_simd::*;
use std::collections::HashMap;
// use ndarray::prelude::*;

aoc_harness::aoc_main!(2021 day 12, generator parse_input,
    part1 [solve1] => 3856,
    part2 [solve2] => 116692,
    example part1 SAMPLE1 => 10,
    example part1 SAMPLE2 => 19,
    example part2 SAMPLE1 => 36,
    );

type Graph = Vec<Vec<u8>>;
//type Mapping = HashMap<String,usize>;
struct Input {
    graph: Graph,
    //mapping: Mapping,
    start: u8,
    end: u8,
}

fn parse_input(input: &str) -> Input {
    let mut graph = Vec::new();
    let mut mapping = HashMap::new();
    let mut counter = 0;
    input.lines().filter(|x|*x != "").for_each(|x|{
        let mut edge = x.split("-");
        let input = edge.next().unwrap();
        let output = edge.next().unwrap();

        let input_key;
        let output_key;
        if let Some(key) = mapping.get(input){
            input_key = *key;
        } else {
            input_key = counter;
            mapping.insert(input.to_string(),counter);
            graph.push(Vec::new());
            counter += 1;
        }
        if let Some(key) = mapping.get(output){
            output_key = *key;
        } else {
            output_key = counter;
            mapping.insert(output.to_string(),counter);
            graph.push(Vec::new());
            counter += 1;
        }
        graph[input_key].push(output_key as u8);
        graph[output_key].push(input_key as u8);
    });
    let big = mapping.iter()
        .filter(|(k,_)|k.chars().next().unwrap().is_ascii_uppercase())
        .map(|a|*a.1 as u8)
        .collect::<Vec<_>>();

    let mut new_graph : Vec<Vec<_>> = vec![Vec::new();graph.len()];

    let start = mapping.iter().find(|&(k,_)|k=="start").map(|(_,v)|*v as u8)
        .unwrap();
    let end = mapping.iter().find(|&(k,_)|k=="end").map(|(_,v)|*v as u8)
        .unwrap();

    (0..mapping.len()).for_each(|ix|{
        let node = &graph[ix];
        let bigs = node.iter().copied().filter(|a|big.contains(a)).collect::<Vec<_>>();
        let extra = bigs.into_iter().flat_map(|k|graph[k as usize].clone().into_iter().collect::<Vec<_>>()
            ).collect::<Vec<_>>();
        let node = &mut graph[ix];
        node.retain(|a|!big.contains(a));
        node.extend(extra);
        new_graph[ix]=node.to_vec();

    });
    Input{graph:new_graph,start:start as u8,end:end as u8}
}

type Visited = Vec<u8>;

struct Game {
    graph: Graph,
    start: u8,
    end: u8,
    revisit_limit: u8,
}
#[derive(Hash,PartialEq,Eq,Clone)]
struct State {
    visit: Visited,
    position: u8,
    revisits: u8,
}

impl Game {
    fn search(& self, state: State, memo: &mut HashMap<State,usize> ) -> usize {
        memo.get(&state).copied().unwrap_or_else(||{
            let mut paths = 0;
            for &n in self.graph[state.position as usize].iter() {
                let rev = &mut state.revisits.clone();
                if state.visit[n as usize]>0 {
                    if *rev >= self.revisit_limit || n == self.start{
                        continue
                    }
                    *rev += 1;
                } else if n == self.end {
                    paths += 1;
                    continue
                }
                let mut visit = state.visit.clone();
                visit[n as usize] += 1;
                paths += self.search(State{visit: visit.clone(),position:n,revisits:*rev},memo);
                visit[n as usize] -= 1;
            }
            memo.insert(state, paths);
            paths
        })
    }
}

fn solve1(Input{graph,start,end}:&Input) -> usize {
    let state = Game{
        graph:graph.clone(),
        start:*start,
        end:*end,
        revisit_limit:0};
    let mut visit = vec![0;graph.len()];
    visit[*start as usize] += 1;
    return state.search(State{visit: visit,position:*start,revisits:0},&mut HashMap::new())
}
fn solve2(Input{graph,start,end}:&Input) -> usize {
    let state = Game{
        graph:graph.clone(),
        start:*start,
        end:*end,
        revisit_limit:1};
    let mut visit = vec![0;graph.len()];
    visit[*start as usize] += 2;
    return state.search(State{visit: visit,position:*start,revisits:0},&mut HashMap::new())
}

pub const SAMPLE1: &str =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

pub const SAMPLE2: &str =
"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
