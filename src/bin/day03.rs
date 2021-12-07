// use itertools::Itertools;
// use std::collections::HashSet;
// use rayon::prelude::*;
// use bitvec::prelude::*;

aoc_harness_macros::aoc_main!(2021 day 3, generator parse_input,
    [part1] => 4001724,
    [part2] => 587895);

type V = (Vec<u32>,usize);
fn parse_input(input: &str) -> V {
    let arr = input.lines().map(|x| {
        x.chars().fold(0,|acc,y|{
            match y {
                '0' => acc<<1 ,
                '1' => (acc<<1) + 1,
                _ => unreachable!(),
            }
        })
    }).collect::<Vec<u32>>();
    (arr,input.lines().next().unwrap().len())
}

fn part1(input: &V) -> u32 {
    let gamma = (0..input.1).fold(0,|acc,ix|{
        let shift = input.1-1-ix;
        let state: u32 = input.0
                .iter()
                .map(|x|x>>shift & 0b1).sum();
        let cmp = 2*state >= input.0.len() as u32;
        acc | ((cmp as u32)<<shift)
    });
    let epsilon = !gamma & ((1<<input.1) - 1 );
    return gamma * epsilon
}

fn f(input: &V,ones: bool) -> u32 {
    let inp = &mut input.0.clone();
    for ix in 0..input.0.len() {
        if inp.len() <= 1 { break }
        let shift = input.1-1-ix;
        let state: u32 = inp
                .iter()
                .map(|x|x>>shift & 0b1).sum();
        let cmp = 2*state >= inp.len() as u32;
        inp.retain(|x|cmp ^ ((x>>shift & 0b1) != 0 ) ^ ones);
    }
    inp[0]
}

fn part2(input: &V) -> u32 {
    let oxy = f(input,true);
    let co2 = f(input,false);
    oxy * co2
}

#[cfg(test)]
mod tests { // {{{
    use crate::parse_input;
    use crate::part1;
    use crate::part2;
type V = (Vec<u32>,usize);
    #[test]
    pub fn test1() {
    let sample: V = parse_input(
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
");

        assert_eq!(part1(&sample), 198)
    }

    #[test]
    pub fn test2() {
    let sample: V = parse_input(
"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
");
        assert_eq!(part2(&sample), 230)
    }
} // }}}
