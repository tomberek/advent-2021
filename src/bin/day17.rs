use text_io::*;
// use rayon::prelude::*;
//use std::collections::VecDeque;

aoc_harness::aoc_main!(2021 day 17, generator parse_input,
    part1 [solve1] => 4950,
    part2 [solve2] => 1477,
    example part1 SAMPLE0 => 45,
    example part2 SAMPLE0 => 112,
    );

type I = i32;
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
// const PY :usize = 1;
// const VX :usize = 2;
// const VY :usize = 3;
const AX :usize = 0;
const AY :usize = 1;
const BX :usize = 2;
const BY :usize = 3;

fn step( input: &mut Particle) {
    let [px,py,vx,vy]=input;
    *input = [*px + *vx,*py + *vy , *vx - vx.signum(), *vy - 1];
}
fn check( [px,py,_,_]: &Particle, [ax,_,_,by]: &Area) -> bool {
    // we already checked px <= bx && py >=ay in "check_valid"
    px >= ax && py <= by
}
fn check_valid( [px,py,vx,_]: &Particle, [ax,ay,bx,_]: &Area) -> bool {
    py >= ay && px <= bx && (*vx !=0 || ( px >= ax && px <= bx))
}
fn solve(area:&Area) -> usize {
    let mut input : [I;4] = [0,0,0,0];
    // calculate lower bound for x-velocity to reach target
    let quad = ((1.0 + (1.0 + 8.0*area[AX] as f64).sqrt())/2.0) as I;
    (quad..=area[BX])
        .filter_map(|orig_vx|{
            let mut steps = area[AX]/orig_vx;
            let mut vx = orig_vx - steps;
            //let mut x = orig_vx*(orig_vx+1)/2 - vx*(vx+1)/2;
            let mut x = steps*(2*orig_vx-steps+1)/2;
            while vx >= 0 {
                if x >= area[AX] && x <= area[BX] {
                    return Some((steps,x,vx))
                }
                steps += 1;
                x += vx;
                vx -= 1;
            }
            None
        })
        .flat_map(|(steps,x,vx)|{
            let min_vy= (area[AY]*2/steps+steps -1)/2;
            let max_vy= area[AY].abs();
            (min_vy..=max_vy).filter(move|vy|{
                input[0]=x;
                input[1]=steps*(2*vy-steps+1)/2;
                input[2]=vx;
                input[3]=vy-steps;
                loop {
                    if check(&input,area) { return true }
                    step(&mut input);
                    if !check_valid(&input,&area){ break }
                }
                return false
            })
        })
        .count()
}
fn solve1(area:&Area) -> I {
    area[AY] * (area[AY]+1) /2
    // solve(&area).into_iter().max().unwrap()
}
fn solve2(area:&Area) -> I {
    solve(&area) as I
}

pub const SAMPLE0: &str = "target area: x=20..30, y=-10..-5";
