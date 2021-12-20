//use std::collections::VecDeque;

aoc_harness::aoc_main!(2021 day 16, generator parse_input,
    part1 [solve1] => 886,
    part2 [solve2] ,
    example part1 SAMPLE0 => 9,
    example part1 SAMPLE1 => 16,
    example part1 SAMPLE2 => 12,
    example part1 SAMPLE3 => 23,
    example part1 SAMPLE4 => 31,

    example part2 SAMPLE5 => 3,
    example part2 SAMPLE6 => 54,
    example part2 SAMPLE7 => 7,
    example part2 SAMPLE8 => 9,
    example part2 SAMPLE9 => 1,
    example part2 SAMPLE10 => 0,
    example part2 SAMPLE11 => 0,
    example part2 SAMPLE12 => 1,
    );

type I = u16;
type Input = Vec<I>;
trait IInput : Iterator<Item=I> {}
impl<T: Iterator<Item=I>> IInput for T {}

fn parse_input(input: &str) -> Input {
    input.trim().chars().flat_map(|x|{
        let n = I::from_str_radix(&x.to_string(),16).unwrap();
        [n>>3,n>>2&1,n>>1&1,n&1]
    }).collect()
}
fn read(input:&mut impl IInput,n: usize) -> Option<I> {
    let mut ret = 0;
    for _ in 0..n {
        ret = ret<<1 | input.next()?
    }
    Some(ret)
}
fn read_lit(input:&mut impl IInput) -> Option<I> {
    let mut s = 0;
    loop {
        let flag = input.next()? == 0;
        s = s <<4 | read(input,4)?;
        if flag {break}
    }
    Some(s)
}

fn solve1(input:&[I]) -> I {
    solve::<false>(&mut input.iter().copied()).unwrap()
}
fn solve2(input:&[I]) -> I {
    solve::<true>(&mut input.iter().copied()).unwrap()
}

fn solve<const P2:bool>(input:&mut impl IInput) -> Option<I> {
    let ver = read(input,3)?;
    let lit;
    let mut packets : Vec<I> = Vec::new();
    let typ = read(input,3)?;
    if typ == 4 {
        lit = read_lit(input)?;
    } else {
        lit = 0;
        if read(input,1)? == 0 {
            let len = read(input,15)?;
            let new_input : Vec<_>= input.take(len as usize).collect();
            let mut new_input_i = new_input.iter().copied();
            packets = (0..).map_while(|_|solve::<P2>(&mut new_input_i)).collect();
        } else {
            packets = (0..read(input,11)?).map_while(|_|solve::<P2>(input)).collect();
        }
    }
    let ret = {
        if P2 {
            match typ {
                0 => packets.iter().sum::<I>(),
                1 => packets.iter().product::<I>(),
                2 => *packets.iter().min()?,
                3 => *packets.iter().max()?,
                4 => lit,
                5 => (packets[0] > packets[1]) as I,
                6 => (packets[0] < packets[1]) as I,
                7 => (packets[0] == packets[1]) as I,
                _ => panic!(),
            }
        } else {ver + packets.iter().sum::<I>()}
    };
    // println!("typ {:?}",typ);
    // println!("packets {:?}",packets.len());
    Some(ret)
}

pub const SAMPLE0: &str = "38006F45291200";
pub const SAMPLE1: &str = "8A004A801A8002F478";
pub const SAMPLE2: &str = "620080001611562C8802118E34";
pub const SAMPLE3: &str = "C0015000016115A2E0802F182340";
pub const SAMPLE4: &str = "A0016C880162017C3686B18A3D4780";
pub const SAMPLE5: &str = "C200B40A82";
pub const SAMPLE6: &str = "04005AC33890";
pub const SAMPLE7: &str = "880086C3E88112";
pub const SAMPLE8: &str = "CE00C43D881120";
pub const SAMPLE9: &str = "D8005AC2A8F0";
pub const SAMPLE10: &str = "F600BC2D8F";
pub const SAMPLE11: &str = "9C005AC2F8F0";
pub const SAMPLE12: &str = "9C0141080250320F1802104A08";
