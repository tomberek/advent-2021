// use text_io::scan;
// use packed_simd::*;
use std::collections::VecDeque;
use ndarray::prelude::*;
//use std::sync::atomic::{AtomicUsize, Ordering};

aoc_harness_macros::aoc_main!(2021 day 11, generator parse_input,
    part1 [solve1] => 1683,
    part2 [solve2] => 788,
    example part1 SAMPLE => 1656,
    example part2 SAMPLE => 195,
    );

type Input = Array2<i8>;

fn parse_input(input: &str) -> Input {
    let out :Vec<Vec<i8>>= input.lines().filter(|x|*x != "").map(|x|{
        x.chars().map(|y|{
            y.to_string().parse().unwrap()
        }).collect()
    }).collect();
    let mut arr = Array2::zeros((out.len(), out[0].len()));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = out[i][j];
        }
    }
    arr
}
fn solve1(input:&Input) -> usize {
    solve::<false>(input)
}
fn solve2(input:&Input) -> usize {
    solve::<true>(input)
}

fn solve<const PART2:bool>(input:&Input) -> usize {
    let mut grid = input.clone();
    let mut count = 0;
    let mut queue = VecDeque::new();
    let mut flashing = 0;
    let size = grid.len();

    let steps = if PART2 {9999} else {100};
    for step in 1..=steps {
        grid += 1;
        grid.indexed_iter_mut().for_each(|(ix,a)|{
            if *a==10 { queue.push_front(ix); }
        });
        while let Some((j,i)) = queue.pop_front() {
            if PART2 {flashing +=1;}
            [ (j.wrapping_sub(1),i),
              (j.wrapping_sub(1),i.wrapping_sub(1)),
              (j.wrapping_sub(1),i+1),
              (j,i.wrapping_sub(1)),
              (j,i+1),
              (j+1,i),
              (j+1,i.wrapping_sub(1)),
              (j+1,i+1),
            ].iter().for_each(|&ix|{
                grid.get_mut(ix).map(|a|{
                    *a +=1;
                    if *a==10 { queue.push_front(ix); }
                });
            });
            if PART2 && flashing == size { return step }
        }
        grid.map_inplace(|a| if *a>9 {
            if !PART2 { count+=1; }
            *a=0;
        });
        queue.clear();
        flashing = 0;
    }
    return count
}

pub const SAMPLE2: &str =
"11111
19991
19191
19991
11111
";

pub const SAMPLE: &str =
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
