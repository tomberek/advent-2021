use itertools::Itertools;
use rayon::prelude::*;
//use std::collections::VecDeque;

aoc_harness::aoc_main!(2021 day 18, generator parse_input,
    part1 [solve1] => 3935,
    part2 [solve2] ,
    example part1 SAMPLE => 143,
    example part1 SAMPLE1 => 1384,
    // example part2 SAMPLE0 => 112,
    example part1 SAMPLE15 => 4140,

    example part2 SAMPLE15 => 3993,
    );

type I = isize;

#[derive(Debug,Clone,Copy)]
struct Fish {
    val: I,
    depth: I,
}
type Line = Vec<Fish>;
type Input = Vec<Line>;

fn parse_input(input: &str) -> Input {
    let mut depth = 0;
    let mut output = Vec::new();
    input.trim().lines().for_each(|l|{
        let mut fish = Vec::new();
        l.chars().for_each(|c|{
            match c {
                '[' => depth +=1,
                ']' => depth -=1,
                ',' => {},
                _ => {
                    fish.push(Fish{val:c as I - 48,depth});
                }
            }
        });
        output.push(fish);
    });
    return output
}
fn step( line: &mut Line ) -> Option<Line> {
    if let Some((i,&Fish{val,depth})) = line.iter().enumerate().find(|(_,Fish{val:_,depth})|*depth >4) {
        //println!("explode {:?}",i);
        if i > 0 {
            line[i-1]=Fish{val:line[i-1].val + val,depth:line[i-1].depth};
        }
        if i+2 < line.len() {
            line[i+2]=Fish{val:line[i+2].val+line[i+1].val,depth:line[i+2].depth};
        }
        line[i]=Fish{val:0,depth:depth-1};
        line[i+1]=Fish{val:0,depth:-1};
        return Some(line.iter().filter(|Fish{val:_,depth}|*depth >=0).copied().collect())
    }
    if let Some((i,&Fish{val,depth})) = line.iter().enumerate().find(|(_,Fish{val,depth:_})|*val >=10) {
        //println!("split {:?}",i);
        let post = line.split_off(i);
        line.extend(
            [Fish{val:val/2,depth:depth+1},
            Fish{val:(val+1)/2,depth:depth+1}]
        );
        let mut l = post.iter();
        l.next();
        line.extend(l);
        return Some(line.to_vec())
    }
    None
}
fn add(input: &Line,input2: &Line) -> Line {
    input.iter().chain(input2.iter()).map(|&Fish{val,depth}|Fish{val:val,depth:depth+1}).collect()
}

fn solve2(input:&Input) -> I {
    let it = input.iter().cartesian_product(input.iter()).collect::<Vec<_>>();
    it.par_iter().map(|(a,b)|{
        let mut accum = add(a,b);
        loop {
            //println!("{:?}\n",accum);
            if let Some(l) = step(&mut accum) {
                accum = l;
            } else { break; }
        }
        score(&mut accum)
    }).max().unwrap()
}
fn solve1(input:&Input) -> I {
    let input = input.clone();
    let mut fin = input.into_iter().reduce(|mut accum,item|{
        accum = add(&mut accum,&item);
        loop {
            //println!("{:?}\n",accum);
            if let Some(l) = step(&mut accum) {
                accum = l;
            } else { break; }
        }
        accum
    }).unwrap();
    return score(&mut fin)
}
fn score(line:&mut [Fish]) -> isize {
    for depth in 0..4 {
        let depth = 4 - depth;
        for left in 0..line.len(){
            if line[left].depth == depth {
                let mut right = left + 1;
                while line[right].depth == 0 {
                    right += 1;
                }
                line[left].val = line[left].val *3 + line[right].val *2;
                line[left].depth -= 1;
                line[right].depth = 0;

            }
        }
    }
    line[0].val
}

pub const SAMPLE: &str = "
[[1,2],[[3,4],5]]
";
pub const SAMPLE1: &str = "
[[[[0,7],4],[[7,8],[6,0]]],[8,1]]
";
pub const SAMPLE10: &str = "
[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]
";
pub const SAMPLE11: &str = "[[[[[9,8],1],2],3],4]";

pub const SAMPLE12: &str = 
"[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

pub const SAMPLE15: &str =
"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
