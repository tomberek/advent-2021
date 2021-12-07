use text_io::scan;

aoc_harness_macros::aoc_main!(2021 day 2, generator parse_input,
    [part1] => 2117664,
    [part2] => 2073416724);

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

fn parse_input(input: &str) -> Vec<(Direction,u32)> {
    input.lines().map(|x| {
        let dir: String; let n: u32;
        scan!(x.bytes() => "{} {}",dir,n);
        let dir = match dir.as_str() {
            "forward" => Direction::Forward,
            "down"    => Direction::Down,
            "up"      => Direction::Up,
            _         => panic!(),
        };
        (dir,n)
    }).collect()
}

fn part1(input: &[(Direction,u32)]) -> u32 {
    let res = input.iter().fold((0,0),|acc,(dir,n)|{
        match dir {
            Direction::Forward => (acc.0+n,acc.1),
            Direction::Down    => (acc.0  ,acc.1+n),
            Direction::Up      => (acc.0  ,acc.1-n),
        }
    });
    res.0 * res.1
}

fn part2(input: &[(Direction,u32)]) -> u32 {
    let res = input.iter().fold((0,0,0),|acc,(dir,n)|{
        match dir {
            Direction::Forward => (acc.0+n,acc.1+acc.2*n,acc.2),
            Direction::Down    => (acc.0  ,acc.1        ,acc.2+n),
            Direction::Up      => (acc.0  ,acc.1        ,acc.2-n),
        }
    });
    res.0 * res.1
}

#[cfg(test)]
mod tests { // {{{
    use crate::parse_input;
    use crate::part1;
    use crate::part2;
    const SAMPLE: Vec<(Direction,u32)> = &parse_input(
"forward 5
down 5
forward 8
up 3
down 8
forward 2
");

    #[test]
    pub fn test1() {
        assert_eq!(part1(SAMPLE), 150)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(SAMPLE), 900)
    }
} // }}}
