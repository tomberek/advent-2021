// use text_io::scan;
// use packed_simd::*;
// use slice_deque::*;

aoc_harness::aoc_main!(2021 day 6, generator parse_input,
    part1 [solve1] => 346063,
    part2 [solve2a,solve2b] => 1572358335990,
    example part1 SAMPLE => 5934,
    example part2 SAMPLE => 26984457539,
    //bench
    );

type Fish = u8;
fn parse_input(input: &str) -> Vec<Fish> {
    input.lines().next().unwrap().split(",").map(|x|
        x.parse().unwrap()
    ).collect()
}

fn solve1(input: &[Fish]) -> usize {
    solve::<80>(input)
}
fn solve2a(input: &[Fish]) -> usize {
    solve::<256>(input)
}
fn solve2b(input: &[Fish]) -> usize {
    solveb(input,256)
}

fn solveb(input: &[Fish],iter: usize) -> usize {
    let mut pool = [0;9];
    for &f in input {
        pool[f as usize] +=1;
    }
    for i in 0..iter {
        let g = i%9;
        let n = pool[g];
        pool[BUMP[g]] += n;
    }
    pool.iter().sum()
}

const BUMP :[usize;9] = [7,8,0,1,2,3,4,5,6];
fn solve<const ITER: usize>(input: &[Fish]) -> usize {
    let mut pool = [0;9];
    for &f in input {
        pool[f as usize] +=1;
    }
    for _ in 0..(ITER/9) {
        for g in 0..9 {
            pool[BUMP[g]] += pool[g];
        }
    }
    for g in 0..(ITER%9) {
            pool[BUMP[g]] += pool[g];
    }
    pool.iter().sum()
}

pub const SAMPLE: &str =
"3,4,3,1,2";
