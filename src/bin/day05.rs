// use itertools::Itertools;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use rayon::prelude::*;
// use ndarray::Array2;
use text_io::scan;
use packed_simd::*;
//use core_simd::*;

aoc_harness_macros::aoc_main!(2021 day 5,
    generator parse_input,
    [solve1] => 4745,
    [solve2] => 18442,
    example part 1 SAMPLE => 5,
    example part 2 SAMPLE => 12,
    // bench
    );

type Line = [i16;4];

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|x|{
        let x1: i16; let x2: i16; let y1: i16; let y2: i16;
        scan!(x.bytes() => "{},{} -> {},{}", y1,x1,y2,x2);
        [y1,x1,y2,x2]
    }).collect()
}

fn solve1(input: &[Line]) -> usize {
    solve(input.iter().filter(|[y1,x1,y2,x2]|
        y1 == y2 || x1 == x2
    ))
}

fn solve2(input: &[Line]) -> usize {
    solve(input.iter().filter(|a@[y1,x1,y2,x2]|{
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let adiff = simd2 - simd1;
        let ydiff = adiff.extract(0).abs();
        let xdiff = adiff.extract(1).abs();
        // let mask = adiff>>16 - 1;
        // let absdiff = (adiff ^ mask) - mask;
        // let ydiff = absdiff.extract(0);
        // let xdiff = absdiff.extract(1);
        y1 == y2 || x1 == x2 || (ydiff == xdiff)
    }))
}

fn solve<'a, I>(input:I) -> usize
    where I: Iterator<Item = &'a Line> {
    let mut board  = [[0 as u8;1000];1000];
    let mut acc = 0;
    input.for_each(|a|{
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let a = simd2 - simd1;
        let ysig = a.extract(0).signum();
        let xsig = a.extract(1).signum();
        let ssig = i16x2::new(ysig,xsig);
        let r = a.extract(1).abs() | a.extract(0).abs();
        (0..=r).for_each(|i|{
            let a = simd1 + i * ssig;
            let c = &mut board
                [a.extract(1) as usize]
                [a.extract(0) as usize];
            acc += (*c==1) as usize;
            *c +=1;
        })
    });
    acc
}

pub const SAMPLE : &str =
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[cfg(test)]
mod tests { // {{{
    use crate::parse_input;
    use crate::solve1;
    use crate::solve2;
    use crate::SAMPLE;

    #[test]
    pub fn test1() {
        let sample = parse_input(SAMPLE);
        assert_eq!(solve1(&sample), 5)
    }

    #[test]
    pub fn test2() {
        let sample = parse_input(SAMPLE);
        assert_eq!(solve2(&sample), 12)
    }

} // }}}
