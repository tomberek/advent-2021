aoc_harness::aoc_main!(2021 day 7, generator parse_input,
    part1 [solve1] => 337488,
    part2 [solve2] => 89647695,
    example part1 SAMPLE => 37,
    example part2 SAMPLE => 168,
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
fn median(numbers: &mut [Crab]) -> Crab {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn solve1(input: &[Crab]) -> usize {
    let i : &mut [Crab]= &mut input.to_vec();
    let pos = median(i);
    (pos..pos+2).map(|pos|{
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
