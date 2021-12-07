aoc_harness_macros::aoc_main!(2021 day 1, generator parse_input,
    [solve1] => 1446,
    [solve2] => 1486,
    example part 1 SAMPLE => 7,
    example part 2 SAMPLE => 5,
    // bench
    );

fn parse_input(input: &str) -> Vec<u16> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn solve<const WINDOW: usize>(input: &[u16]) -> u16 {
    let mut count = 0;
    for i in WINDOW..input.len() {
        if input[i - WINDOW] < input[i] {
            count += 1;
        }
    }
    count
}

fn solve1(input: &[u16]) -> u16 {
    solve::<1>(input)
}

fn solve2(input: &[u16]) -> u16 {
    solve::<3>(input)
}

pub const SAMPLE: &str =
"199
200
208
210
200
207
240
269
260
263
";
