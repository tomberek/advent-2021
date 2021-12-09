// use text_io::scan;
// use packed_simd::*;
// use slice_deque::*;

aoc_harness_macros::aoc_main!(2021 day 8, generator parse_input,
    [solve1] => 421,
    [solve2] => 986163,
    example part 1 SAMPLE => 26,
    example part 2 SAMPLE => 61229,
    //bench
    );

type Segment = u8;
type Output = u8;
type Line = (Vec<Segment>,Vec<Output>);
type Game = Vec<Line>;

fn parse_input(input: &str) -> Game {
    input.lines().map(|x|{
        let mut positions = x.split("|");
        let line = positions.next().unwrap().split(" ").filter(|&x|x!="").map(|x|
            x.chars().fold(0,|acc,x|acc|(1<<(x as u8 -97)))
        ).collect();
        let results = positions.next().unwrap().split(" ").filter(|&x|x!="").map(|x|
            x.chars().fold(0,|acc,x|acc|(1<<(x as u8 -97)))
        ).collect();
        (line,results)
    }).collect()
}

fn solve1(game: &Game) -> usize {
    // solve(game).map(|x|x.filter(|&i|i == 1 || i == 4 || i == 7 || i == 8).count()).sum::<usize>()
    game.iter().flat_map(|(_,output)|{
        output.iter().filter(|&x|matches!(x.count_ones(),2|3|4|7))
    }).count()
}

fn solve<'a>(game:&Game) -> impl Iterator<Item= impl Iterator<Item= u8> + '_ >{
    game.iter().map(|(input,output)|{
        let i  = |count,f:Box<dyn Fn(u8)->bool>| input.iter()
            .find(|x|x.count_ones()==count && f(**x)).unwrap();

        let one =   i(2,Box::new(|_|true));
        let four =  i(4,Box::new(|_|true));
        let seven = i(3,Box::new(|_|true));
        let eight = i(7,Box::new(|_|true));
        let six =   i(6,Box::new(|x|(x|one) == *eight));
        let five =  i(5,Box::new(|x|(six|x) == *six));
        let seg_e = six - five;
        let zero =  i(6,Box::new(|x|x != *six  && x & seg_e != 0));
        let two =   i(5,Box::new(|x|x != *five && x & seg_e != 0));
        let nine =  i(6,Box::new(|x|x != *six  && x != *zero));
        let three = i(5,Box::new(|x|x != *two  && x != *five));

        output.iter().map(|x|{
            match x {
                x if *x == *zero  => 0,
                x if *x == *one   => 1,
                x if *x == *two   => 2,
                x if *x == *three => 3,
                x if *x == *four  => 4,
                x if *x == *five  => 5,
                x if *x == *six   => 6,
                x if *x == *seven => 7,
                x if *x == *eight => 8,
                x if *x == *nine  => 9,
                _ => panic!(),
            }
        })
    })

}

fn solve2(game:&Game) -> usize {
    solve(game).map(|x|x.fold(0,|acc,i|acc*10+i as usize)).sum::<usize>()
}

pub const SAMPLE: &str =
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
