#![feature(portable_simd)]
// use itertools::Itertools;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use rayon::prelude::*;
// use ndarray::Array2;
use text_io::scan;
use packed_simd::{i16x4, i16x2,Simd};
//use core_simd::*;

aoc_harness_macros::aoc_main!(2021 day 5,
    generator parse_input,
    [part1] => 4745,
    [part2] => 18442,
    example part 1 SAMPLE => 5,
    example part 2 SAMPLE => 12
    );

type Line = [i16;4];

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|x|{
        let x1: i16; let x2: i16; let y1: i16; let y2: i16;
        scan!(x.bytes() => "{},{} -> {},{}", y1,x1,y2,x2);
        [y1,x1,y2,x2]
    }).collect()
}

fn part1(input: &[Line]) -> usize {
    solve(input.iter().filter(|[y1,x1,y2,x2]|
        y1 == y2 || x1 == x2
    ))
}

fn part2(input: &[Line]) -> usize {
    solve(input.iter().filter(|[y1,x1,y2,x2]|{
        let ydiff = (y2 - y1 ).abs();
        let xdiff = (x2 - x1 ).abs();
        y1 == y2 || x1 == x2 || (ydiff == xdiff)
    }))
}

fn solve<'a, I>(input:I) -> usize
    where I: Iterator<Item = &'a Line> {
    let mut board  = [[0 as u8;1000];1000];
    let mut acc = 0;
    input.for_each(|a@[y1,x1,y2,x2]|{
        // let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        // let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        // let a = simd2 - simd1;
        // let ysig = a.extract(0).signum();
        // let xsig = (x2 - x1).signum();
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let a = simd2 - simd1;
        let ssig = unsafe {
            let ysig = a.extract_unchecked(0).signum();
            let xsig = a.extract_unchecked(1).signum();
            i16x2::new(ysig,xsig)
        };
        unsafe {
            let r = a.extract_unchecked(1).abs()
                .max(( a.extract_unchecked(0) ).abs());
        (0..=r).for_each(|i|{
            let a = simd1 + i * ssig;
            // let y = (y1 + i * ysig) as usize;
            // let x = (x1 + i * xsig) as usize;
            // let c = &mut board[y][x];
            let c = &mut board
                [a.extract_unchecked(1) as usize]
                [a.extract_unchecked(0) as usize];
            acc += (*c==1) as usize;
            *c +=1;
        })
        }
    });
    acc
}

const SAMPLE : &str =
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
    use crate::part1;
    use crate::part2;

    #[test]
    pub fn test1() {
        let sample = parse_input(sample);
        assert_eq!(part1(&sample), 5)
    }

    #[test]
    pub fn test2() {
        let sample = parse_input(sample);
        assert_eq!(part2(&sample), 12)
    }

} // }}}
