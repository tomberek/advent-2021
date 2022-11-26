use text_io::{try_scan};

aoc_harness::aoc_main!(2021 day 24, generator parse_input,
    part1 [solve1],
    //part2 [part2] => 2073416724,
    //example part1 SAMPLE => 150,
    //example part2 SAMPLE => 900,
    );

type I = i64;

#[derive(Debug)]
enum Instruction {
    Input {reg:char},
    Add {reg:char,val:Result<I,char>},
    Mul {reg:char,val:Result<I,char>},
    Div {reg:char,val:Result<I,char>},
    Mod {reg:char,val:Result<I,char>},
    Eql {reg:char,val:Result<I,char>},
}

// peek character + input !=
// popper if pop then end peek 25 x * 1 + * input adder + x * +
//
// x = (z%26 + character) != input
// z = (z/popper) * (25 * x + 1) + (input + adder)* x
// popper char adder
//
//
// 1 _ 6
// 1 _ 2
// 1 _ 13
// 26 -6 _
// 1 _ 13
// 26 -12 _
// 1 _ 3
// 1 _ 11
// 1 _ 10
// 26 -2 _
// 26 -5 _
// 26 -4 _
// 26 -4 _
// 26 -12 _
//
//
// i[3] = i[2] + 7
// i[5] = i[4] + 1
// i[9] = i[8] + 8
// i[10] = i[7] + 6
// i[11] = i[6] - 1
// i[12] = i[1] - 2
// i[13] = i[0] - 6
// 
//
//
//
//
//
//
// 7
// 3
// 1
// 8
// 1
// 2
// 2
// 1
// 1
// 9
// 7
// 1
// 1
// 1
//
//
// 73181221197111
// 99298993199873
//
fn try3(x:&str)->Result<(String,char,String),text_io::Error> {
    let op: String;
    let reg: char;
    let val: String;
    try_scan!(x.bytes() => "{} {} {}",op,reg,val);
    Ok((op,reg,val))
}
fn try2(x:&str)->Result<Instruction,text_io::Error> {
    let reg: char;
    try_scan!(x.bytes() => "inp {}",reg);
    Ok(Instruction::Input{reg})
}
fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().filter_map(|x| {
        try3(x).and_then(|(op,reg,val)|{
            match op.as_str() {
                "inp" => Ok(Instruction::Input{reg:reg}),
                "add" => Ok(Instruction::Add{reg:reg,val:val.parse().or(Err(val.chars().next().unwrap()))}),
                "mul" => Ok(Instruction::Mul{reg:reg,val:val.parse().or(Err(val.chars().next().unwrap()))}),
                "div" => Ok(Instruction::Div{reg:reg,val:val.parse().or(Err(val.chars().next().unwrap()))}),
                "mod" => Ok(Instruction::Mod{reg:reg,val:val.parse().or(Err(val.chars().next().unwrap()))}),
                "eql" => Ok(Instruction::Eql{reg:reg,val:val.parse().or(Err(val.chars().next().unwrap()))}),
                _ => panic!(),
            }
        })
            .or_else(
            |_|try2(x)).ok()
    }).collect()
}
type Reg = (I,I,I,I);
fn op(op: fn(I,I)->I, reg:char, val:Result<I,char>, (w,x,y,z):Reg) -> Reg {
    let val = match val {
        Ok(val) => val,
        Err(val) => {
            match val {
                'w' => w,
                'x' => x,
                'y' => y,
                'z' => z,
                _ => panic!(),
            }
        }
    };
    match reg {
        'w' => (op(w,val),x,y,z),
        'x' => (w,op(x,val),y,z),
        'y' => (w,x,op(y,val),z),
        'z' => (w,x,y,op(z,val)),
        _ => panic!(),
    }
}
fn solve1(input: &[Instruction]) -> I {
    //for (idx,_) in input.iter().enumerate() {
    //    if idx%18 == 4 ||
    //        idx%18 == 5 ||
    //        idx%18 == 15 {
    //        //println!("{:?}",i);
    //    }
    //    if idx%18 == 0 {
    //        //println!("");
    //    }
    //}
    let mut s = [7, 3, 1, 8, 1, 2, 2, 1, 1, 9, 7, 1, 1, 1].iter();
    //let mut s = [9,9,2,9,8,9,9,3,1,9,9,8,7,3].iter();
    let res = input.iter().fold((0,0,0,0),|(w,x,y,z),instr|{
        //println!("{:?}",(w,x,y,z));
        match instr {
            Instruction::Input{reg} => {
                match reg {
                    'w' => (*s.next().unwrap(),x,y,z),
                    _ => panic!(),
                }
            },
            Instruction::Add{reg,val} => op(|a,b|a+b,*reg,*val,(w,x,y,z)),
            Instruction::Mul{reg,val} => op(|a,b|a*b,*reg,*val,(w,x,y,z)),
            Instruction::Div{reg,val} => op(|a,b|a/b,*reg,*val,(w,x,y,z)),
            Instruction::Mod{reg,val} => op(|a,b|a%b,*reg,*val,(w,x,y,z)),
            Instruction::Eql{reg,val} => op(|a,b|(a==b) as I,*reg,*val,(w,x,y,z)),
        }
    });
    res.3
}

// const SAMPLE: &str =
// "inp w
// add z w
// mod z 2
// div w 2
// add y w
// mod y 2
// div w 2
// add x w
// mod x 2
// div w 2
// mod w 2
// ";
