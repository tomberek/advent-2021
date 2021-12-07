// use text_io::scan;
// use packed_simd::*;
// use slice_deque::*;

aoc_harness_macros::aoc_main!(2021 day 7, generator parse_input,
    [solve1] => 337488,
    [solve2] => 89647695,
    example part 1 SAMPLE => 37,
    example part 2 SAMPLE => 168,
    //bench
    );

type Crab = usize;
fn parse_input(input: &str) -> Vec<Crab> {
    let positions : Vec<Crab> = input.lines().next().unwrap().split(",").map(|x|
        x.parse().unwrap()
    ).collect();
    positions
    // let mut crabs = vec![0;*size+1];
    // positions.iter().for_each(|&x|crabs[x] += 1);
    // crabs
}

fn solve1(input: &[Crab]) -> usize {
    (0..input.len()).map(|pos|{
        input.iter().map(|&i| (i as isize - pos as isize ).abs() as usize).sum::<usize>()
    }).min().unwrap()
}

#[inline(never)]
fn solve2(input: &[Crab]) -> usize {
    let pos = input.iter().sum::<usize>() / input.len();
    (pos..pos+2).map(|pos|{
        input.iter().map(|&i| {
            let dist = (i as isize - pos as isize ).abs() as usize;
            dist * (dist+1) / 2
        }).sum::<usize>()
    }).min().unwrap()
}

pub const SAMPLE: &str =
"16,1,2,0,4,2,7,1,2,14";
