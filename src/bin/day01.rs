// use itertools::Itertools;
// use std::collections::HashSet;
// use rayon::prelude::*;

aoc_harness_macros::aoc_main!(2021 day 1, generator parse_input, [part1], [part2]);

fn parse_input(input: &str) -> Vec<u16> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn solve<const WINDOW: usize>(input: &[u16]) -> u16 {
    input
        .windows(WINDOW)
        .map(|s|s.iter().sum())
        .fold( (u16::MAX,0), |(acc,a),i|
                 (i,a + (i > acc) as u16)
         ).1
}

fn part1(input: &[u16]) -> u16 {
    solve::<1>(input)
}

fn part2(input: &[u16]) -> u16 {
    solve::<3>(input)
}

#[cfg(test)]
mod tests {
    const SAMPLE: [u16; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE.to_vec()), 7)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE.to_vec()), 5)
    }
}
