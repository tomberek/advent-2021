#![feature(explicit_generic_args_with_impl_trait)]
use text_io::*;
use rayon::prelude::*;
//use std::collections::VecDeque;

aoc_harness::aoc_main!(2021 day 17, generator parse_input,
    part1 [solve1] => 4950,
    part2 [solve2] => 1477,
    example part1 SAMPLE0 => 45,
    example part2 SAMPLE0 => 112,
    );

type I = i32;
type Point = [I;2];
type Vel = Point;
type Area = [I;4];
type Particle = Area;

fn parse_input(input: &str) -> Area {
    input.lines().map(|x|{
        let px :I;
        let py :I;
        let vx :I;
        let vy :I;
        scan!(x.bytes() => "target area: x={}..{}, y={}..{}",px,vx,py,vy);
        //((px,py),(vx,vy))
        [px,py,vx,vy]

    }).collect::<Vec<_>>()[0]
}
//const PX :usize = 0;
const PY :usize = 1;
// const VX :usize = 2;
// const VY :usize = 3;
// const AX :usize = 0;
const AY :usize = 1;
const BX :usize = 2;
// const BY :usize = 3;
fn step( [px,py,vx,vy]: &Particle) -> Particle {
    //( (px + vx,py + vy) , ( vx - vx.signum(), vy - 1))
    [px + vx,py + vy , vx - vx.signum(), vy - 1]
}
fn check( [px,py,_,_]: &Particle, [ax,ay,bx,by]: &Area) -> bool {
    px >= ax && px <= bx && py >= ay && py <= by
}
fn check_valid( [px,py,vx,_]: &Particle, [ax,ay,bx,_]: &Area) -> bool {
    py >= ay && px <= bx && (*vx !=0 || ( px >= ax && px <= bx))
}
fn solve1(area:&Area) -> I {
    solve(&area).into_iter().max().unwrap()
}
fn solve2(area:&Area) -> I {
    solve(&area).iter().count() as I
}
fn solve(area:&Area) -> Vec<I> {
    (1..area[BX]+1).map(move |vx|{
        (area[AY]..-1 * area[AY]+1).filter_map(move |vy|{
            let mut input = [0,0,vx,vy];
            let mut highest = input[PY];
            while check_valid(&input,&area) {
                input = step(&input);
                if input[PY] > highest {
                    highest = input[PY];
                }
                if check(&input,area) {
                    //println!("in box {} {}",vx,vy);
                    return Some(highest)
                }
            }
            //println!("failed {:?}",input);
            return None
        })
    }).flatten().collect()
}

pub const SAMPLE0: &str = "target area: x=20..30, y=-10..-5";
