use std::collections::VecDeque;
use itertools::Itertools;

aoc_harness::aoc_main!(2021 day 10, generator parse_input,
    part1 [solve1] => 290691,
    part2 [solve2] => 2768166558,
    example part1 SAMPLE => 26397,
    example part2 SAMPLE => 288957,
    );

type Line = String;
type Input = Vec<Line>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|x|x.to_string()).collect()
}

fn solve2(input:&Input) -> usize {
    let mut q = VecDeque::new();
    let p = input.iter().filter_map(|line|{
        q.clear();
        let mut corrupt = false;
        for c in line.chars() {
            match c {
                '(' => {q.push_front(1);},
                '[' => {q.push_front(2);},
                '{' => {q.push_front(3);},
                '<' => {q.push_front(4);},
                ')' => {q.pop_front().and_then(|p|(p!=1).then(||{corrupt=true;}));},
                ']' => {q.pop_front().and_then(|p|(p!=2).then(||{corrupt=true;}));},
                '}' => {q.pop_front().and_then(|p|(p!=3).then(||{corrupt=true;}));},
                '>' => {q.pop_front().and_then(|p|(p!=4).then(||{corrupt=true;}));},
                _ => unreachable!(),
            }
            if corrupt { break }
        }
        (!corrupt && q.len() > 0).then(|| q.iter().fold(0,|acc,x|acc*5 + x))
    }).collect::<Vec<_>>();
    // faster discovery center value
    // p.sort_unstable();
    // p[(p.len())/2]
    *p.iter().k_smallest(p.len() / 2 + 1).last().unwrap()
}
fn solve1(input:&Input) -> usize {
    let mut q = VecDeque::new();
    input.iter().flat_map(|line|{
        q.clear();
        line.chars().filter_map(|c|{
            match c {
                '{'|'['|'('|'<' => { q.push_front(c); None},
                ')' => q.pop_front().and_then(|prev|(prev!='(').then(||3)),
                ']' => q.pop_front().and_then(|prev|(prev!='[').then(||57)),
                '}' => q.pop_front().and_then(|prev|(prev!='{').then(||1197)),
                '>' => q.pop_front().and_then(|prev|(prev!='<').then(||25137)),
                _ => panic!(),
            }
        }).next()
    }).group_by(|x|*x).into_iter()
        .map(|(x,g)|{ g.into_iter().count() * x })
        .sum()
}

pub const SAMPLE: &str =
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
