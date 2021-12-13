// use text_io::scan;
// use packed_simd::*;
// use slice_deque::*;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use ndarray::prelude::*;

aoc_harness_macros::aoc_main!(2021 day 9, generator parse_input,
    part1 [solve1] => 436,
    part2 [solve2] => 1317792,
    example part1 SAMPLE => 15,
    example part2 SAMPLE => 1134,
    );

type Line = Vec<i8>;
type Input = Array2<i8>;

fn parse_input(input: &str) -> Input {
    let out :Vec<Line>= input.lines().filter(|x|*x != "").map(|x|{
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

fn solve2(input:&Input) -> usize {
    let mut basins = BinaryHeap::new();
    let mut visited = input.clone();
    let mut queue = VecDeque::new();

    input.indexed_iter().filter_map(|(item@(j,i),_)|{
            check(i as isize,j as isize,&input)
                .and_then(|_|Some(item))
        })
        .for_each(|item|{
            let mut b = 0;
            queue.push_front(item);
            visited[item]=-1;
            while let Some((j,i)) = queue.pop_front() {
                b +=1;
                [ (j.wrapping_sub(1),i),
                  (j,i.wrapping_sub(1)),
                  (j,i+1),
                  (j+1,i),
                ].iter()
                    .for_each(|&item|{
                    if let Some(&p) = visited.get(item){
                        if p < 0 || p ==9 {
                            return
                        }
                        visited[item]=-1;
                        queue.push_front(item);
                    }
                });
            }
            basins.push(b);
            queue.clear();
    });
    (0..3).flat_map(|_| basins.pop()).product()
}

fn solve1(input:&Input) -> usize {
    input.indexed_iter().filter_map(|((j,i),_)|{
            check(i as isize,j as isize,input).map(|x|x as usize +1)
    }).sum()
}

fn check(i: isize,j:isize,input: &Input) -> Option<i8> {
    let pos = input[[j as usize,i as usize]];
    if pos == 9 {
        return None
    }
    [ (j.wrapping_sub(1),i),
      (j,i.wrapping_sub(1)),
      (j,i+1),
      (j+1,i),
    ].iter()
        .map(|&(y,x)|pos < *input.get((y as usize,x as usize)).unwrap_or(&9))
        .all(|x|x).then(||pos)
}

pub const SAMPLE: &str =
"2199943210
3987894921
9856789892
8767896789
9899965678
";
