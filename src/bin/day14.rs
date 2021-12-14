use text_io::scan;
// use packed_simd::*;
//use std::collections::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
// use ndarray::prelude::*;

aoc_harness_macros::aoc_main!(2021 day 14, generator parse_input,
    part1 [solve1] => 3587,
    part2 [solve2] => 3906445077999,
    example part1 SAMPLE => 1588,
    example part2 SAMPLE => 2188189693529,
    );

type Rules = [u8;26*32];
struct Input {
    rules: Rules,
    start: [isize;26*32],
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.split("\n\n");
    let start = lines.next().unwrap().to_string();
    let mut rules : Rules = [0;26*32];
    lines.next().unwrap().lines().for_each(|x|{
        let from :String;
        let to :String;
        scan!(x.bytes() => "{} -> {}",from,to);
        let f : &[u8;2] = from.as_bytes().try_into().expect("bad size");
        let t : &[u8;1] = to.as_bytes().try_into().expect("bad size");
        rules[((f[1] as usize - 65)*32) + (f[0] as usize - 65)]=t[0] - 65;
    });
    let mut totals : [isize;26*32] = [0;26*32];
    start.chars().tuple_windows().map(|(a,b)|a.to_string()+&b.to_string()).for_each(|key|{
        let f : &[u8;2] = key.as_bytes().try_into().expect("bad size");
        let key = ((f[1] as u16 - 65)*32) + (f[0] as u16 - 65);
        totals[key as usize]+=1;
    });
    Input{rules,start:totals}
}

fn solve1(input:&Input) -> usize {
    solve(input,10)
}
fn solve2(input:&Input) -> usize {
    solve(input,40)
}
fn solve(Input{rules,start}:&Input,iter:usize) -> usize {
    let mut totals = start.clone();
    let mut round: [isize;26*32] = [0;26*32];
    (0..iter).for_each(|_|{
        totals.iter().enumerate().filter(|(_,&v)|v>0).for_each(|(key,val)|{
            let new_key = rules[key as usize];
            round[((key as u16 & 0x001F) + ((new_key as u16)*32)) as usize]+=val;
            round[((new_key as u16) + (key as u16 & 0xFFE0)) as usize]+=val;
            round[key as usize] -= val;
        });
        round.iter().enumerate().filter(|(_,&v)|v!=0).for_each(|(key,val)|{
            totals[key as usize]+=val;
        });
        round.fill(0);
    });
    let mut m = [0;32];
    for (key,val) in totals.iter().enumerate() {
        m[key/32] += val;
        m[key&0x1F] += val;
    }
    let max = m.iter().enumerate().max_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();
    let min = m.iter().enumerate().filter(|(_,&v)|v>0).min_by_key(|(_, v)| *v).map(|(_, v)| v).unwrap();
    // println!("max: {:?}",max);
    // println!("min: {:?}",min);
    return (*max - *min + 1) as usize >>1
}

pub const SAMPLE: &str =
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
