// use packed_simd::*;
//use std::collections::HashMap;
//use rayon::prelude::*;
// use packed_simd::*;
use ndarray::prelude::*;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Position = (usize,usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
    actual_cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
aoc_harness::aoc_main!(2021 day 15, generator parse_input,
    part1 [solve1] => 537,
    part2 [solve2],
    example part1 SAMPLE => 40,
    example part2 SAMPLE => 315,
    );

//type Rules = [u8;26*32];
type I = u8;
type Input = Array2<I>;

fn parse_input(input: &str) -> Input {
    let out :Vec<Vec<_>>= input.lines().filter(|x|*x != "").map(|x|{
        x.chars().map(|y|{
            y.to_string().parse().unwrap()
        }).collect()
    }).collect();
    let mut arr = Array2::zeros((out.len(), out[0].len()));
    for (j, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (i, col) in row.iter_mut().enumerate() {
            *col = out[j][i];
        }
    }
    arr
}

fn solve1(input:&Input) -> usize {
    let size = input.raw_dim();
    shortest_path(input,(0,0),(size[0]-1,size[1]-1)).unwrap()
}
fn solve2(input:&Input) -> usize {
    let size = input.raw_dim();
    let mut arr = Array2::zeros((size[0]*5, size[1]*5));
    for (j, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (i, col) in row.iter_mut().enumerate() {
            let big_y = j / size[0];
            let little_y = j %size[0];
            let big_x = i / size[1];
            let little_x = i %size[1];
            *col = 1 + (input[[little_y,little_x]] + ((big_y+big_x) as u8) -1)%9;
        }
    }
    shortest_path(&arr,(0,0),(size[0]*5-1,size[1]*5-1)).unwrap()
}
fn neighbors(input: &Input, pos: Position) -> impl Iterator<Item=Position> {
    let size = input.raw_dim();
    let (j,i) = pos;
    [
      (j,i+1),
      (j+1,i),
      (j.wrapping_sub(1),i),
      (j,i.wrapping_sub(1)),
    ].into_iter()
    .filter(move |x| x.0 < size[0] && x.1 < size[1])
}

fn l(input: &Input,pos: Position) -> usize{
    let size = input.dim();
    0
    // ((size.0-pos.0) + (size.1-pos.1))
}

fn shortest_path(input: &Input, start: Position, goal: Position) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let size = input.dim();

    let mut heap = BinaryHeap::new();
    let mut heap_back = BinaryHeap::new();

    // We're at `start`, with a zero cost
    let mult = 6;
    let high_cost = (size.0+size.1) * mult;
    //let mut dist: Array2<usize> = Array2::from_elem(input.dim(),high_cost);
    let mut dist: Array2<usize> = Array2::zeros(input.dim());
    for (j, mut row) in dist.axis_iter_mut(Axis(0)).enumerate() {
        for (i, col) in row.iter_mut().enumerate() {
            *col = (j+i)*9;
        }
    }

    dist[start] = 0;
    heap.push(State { cost:0, position: start, actual_cost:0 });

    dist[start] = high_cost;
    heap_back.push(State { cost:high_cost, position: goal, actual_cost:0 });

    // Examine the frontier with lower cost nodes first (min-heap)
    loop {
        let mut flag = true;
        if let Some(State { cost:_, position, actual_cost }) = heap.pop() {
            // Alternatively we could have continued to find all shortest paths
            if position == goal { return Some(actual_cost); }

            // Important as we may have already found a better way
            if actual_cost <= dist[position] {
                // For each node we can reach, see if we can find a way with
                // a lower cost going through this node
                heap.extend(
                    neighbors(input,position).filter_map(|edge|{
                    let actual_cost = actual_cost + input[edge] as usize;
                    if actual_cost < dist[edge] {
                        dist[edge] = actual_cost;
                        Some(State {
                            cost: actual_cost + l(&input,edge),
                            position: edge,
                            actual_cost: actual_cost,
                        })
                    }else {None}

                }));
            }
            flag = false;
        }
        //if let Some(State { cost:_, position, actual_cost }) = heap_back.pop() {
        //    if position == start { return Some(actual_cost); }
        //    //println!("{:?} {} :{}<{}",position, actual_cost, (position.0+position.1)*9+actual_cost, dist[position]);
        //    if (position.0+position.1)*mult + actual_cost <= dist[position] {
        //        heap_back.extend(
        //            neighbors(input,position).filter_map(|edge|{
        //            let actual_cost = actual_cost + input[edge] as usize;
        //            if (edge.0+edge.1)*mult + actual_cost < dist[edge] {
        //                //println!("is good: {:?} {:?}",edge, actual_cost);
        //                dist[edge] = (edge.0+edge.1)*mult + actual_cost;
        //                Some(State {
        //                    cost: (actual_cost + l(&input,edge)),
        //                    position: edge,
        //                    actual_cost: actual_cost,
        //                })
        //            }else {None}

        //        }));
        //    }
        //    flag = false;
        //}
        if flag { break;}
    }

    // Goal not reachable
    None
}

pub const SAMPLE: &str =
"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
