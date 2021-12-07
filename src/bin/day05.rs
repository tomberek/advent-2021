// use itertools::Itertools;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use rayon::prelude::*;
// use ndarray::Array2;
use text_io::scan;
use packed_simd::*;

aoc_harness_macros::aoc_main!(2021 day 5,
    generator parse_input,
    [solve1] => 4745,
    [solve2a,solve2b] => 18442,
    // [solve2b] => 18442,
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
fn solve2a(input: &[Line]) -> usize {
    solve(input.iter().filter(|a@[y1,x1,y2,x2]|{
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let adiff = simd2 - simd1;
        let ydiff = adiff.extract(0).abs();
        let xdiff = adiff.extract(1).abs();
        y1 == y2 || x1 == x2 || (ydiff == xdiff)
    }))
}

fn solve2b(input: &[Line]) -> usize {
    solve2simd(input.iter().filter(|a@[y1,x1,y2,x2]|{
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let adiff = simd2 - simd1;
        let ydiff = adiff.extract(0).abs();
        let xdiff = adiff.extract(1).abs();
        y1 == y2 || x1 == x2 || (ydiff == xdiff)
    }))
}

const MIN2 : i16x2 = i16x2::splat(-1);
const MAX2 : i16x2 = i16x2::splat(1);

fn solve<'a, I>(input:I) -> usize
    where I: Iterator<Item = &'a Line> {
    let mut board  = [[0 as u8;1000];1000];
    let mut acc = 0;
    let mut input = input.copied().collect::<Vec<Line>>();
    input.sort_unstable();
    for a in input {
    //for a in input {
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let a = simd2 - simd1;
        let ssig = a.max(MIN2).min(MAX2);
        let r = a.extract(1).abs() | a.extract(0).abs();
        (0..=r).for_each(|i|{
            let a = simd1 + i * ssig;
            let c = &mut board
                [a.extract(1) as usize]
                [a.extract(0) as usize];
            acc += (*c==1) as usize;
            *c +=1;
        })
    }
    acc
}

fn solve2simd<'a, I>(input:I) -> usize
    where I: Iterator<Item = &'a Line> {
    let mut board  = [[0 as u8;1000];1000];
    let mut acc = 0;
    let mut input = input.copied().collect::<Vec<Line>>();
    input.sort_unstable();
    const MIN32 : i16x32 = i16x32::splat(-1);
    const MAX32 : i16x32 = i16x32::splat(1);
    const MIN2 : i16x2 = i16x2::splat(-1);
    const MAX2 : i16x2 = i16x2::splat(1);
    let chunks = input.chunks_exact(16);
    let remainder = chunks.remainder();

    let shuffle : Simd<[u16;32]> = u16x32::from_slice_unaligned(&[1,0,3,2,5,4,7,6,9,8,11,10,13,12,15,14,17,16,19,18,21,20,23,22,25,24,27,26,29,28,31,30]);

    for a in chunks {
        let simd1 = i16x32::from_slice_unaligned
            (&a.iter().flat_map(|x|[x[0],x[1]]).collect::<Vec<i16>>()[..]);
        let simd2 = i16x32::from_slice_unaligned
            (&a.iter().flat_map(|x|[x[2],x[3]]).collect::<Vec<i16>>()[..]);
        //let simd2 = i16x16::from_slice_unaligned(&a[2..]);
        let a = simd2 - simd1;
        let ssig = a.max(MIN32).min(MAX32);
        // println!("{:?}",simd1);
        // println!("{:?}",simd2);
        // println!("{:?}",ssig);
        let mask = a>>16 - 1;
        let aabs = (a ^ mask) - mask;
        let r = aabs.shuffle1_dyn(shuffle) | aabs;
        for idx in 0..16 {
            let r = r.extract(idx*2);
            let chunks2vec = (0..=r).collect::<Vec<i16>>();
            let chunks2 = chunks2vec.chunks_exact(8);
            let rem2 = chunks2.remainder();

            let simd1y = simd1.extract(idx*2);
            let simd1x = simd1.extract(idx*2+1);
            let ssigy = ssig.extract(idx*2);
            let ssigx = ssig.extract(idx*2+1);

            let simd1y16 =  i16x8::splat(simd1y);
            let simd1x16 =  i16x8::splat(simd1x);
            let ssigy16 =  i16x8::splat(ssigy);
            let ssigx16 =  i16x8::splat(ssigx);
            for i in chunks2 {
                let i = i16x8::from_slice_unaligned(i);
                //println!("{:?}",i);
                let ay = simd1y16 + i * ssigy16;
                let ax = simd1x16 + i * ssigx16;
                for ii in 0..8 {
                    //println!("{:?}",a);
                    let c = &mut board
                        [ax.extract(ii) as usize]
                        [ay.extract(ii) as usize];
                    acc += (*c==1) as usize;
                    *c +=1;
                }
            }
            let simd1 = i16x2::new(simd1y,simd1x);
            let ssig = i16x2::new(ssigy,ssigx);
            for i in rem2 {
                //println!("{:?}",i);
                let i = i16x2::new(*i,*i);
                let a = simd1 + i * ssig;
                let c = &mut board
                    [a.extract(1) as usize]
                    [a.extract(0) as usize];
                    // [a.extract(idx*2+1) as usize]
                    // [a.extract(idx*2) as usize];
                acc += (*c==1) as usize;
                *c +=1;
            }
        }
    }
    for a in remainder {
    //for a in input {
        let simd1 = i16x2::from_slice_unaligned(&a[0..]);
        let simd2 = i16x2::from_slice_unaligned(&a[2..]);
        let a = simd2 - simd1;
        let ssig = a.max(MIN2).min(MAX2);
        let r = a.extract(1).abs() | a.extract(0).abs();
        (0..=r).for_each(|i|{
            let a = simd1 + i * ssig;
            let c = &mut board
                [a.extract(1) as usize]
                [a.extract(0) as usize];
            acc += (*c==1) as usize;
            *c +=1;
        })
    }
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
