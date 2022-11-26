use text_io::scan;
// use ndarray::*;
//use ndarray_linalg::*;
// use rulinalg::norm::Euclidean;
use std::collections::{HashMap};

aoc_harness::aoc_main!(2021 day 21, generator parse_input,
    part1 [solve1] => 512442,
    part2 [solve2] => 346642902541848,
    example part1 SAMPLE => 739785,
    example part2 SAMPLE => 444356092776315,
    );

fn parse_input(input: &str) -> Input {
    let out = input.trim().lines().map(|line|{
        let player: I;
        let position: I;
        scan!(line.bytes() => "Player {} starting position: {}",player,position);
        position
    }).collect::<Vec<_>>();
    out
}

type I = u16;
type Players = Vec<I>;
type Input = Players;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
struct Game {
    players: [I;2],
    score: [usize;2],
    turn: usize,
}

const QUANT: [(I, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn play(mut map: &mut HashMap<Game,[usize;2]>,Game{players:players_orig,score:score_orig,turn}:Game) -> [usize;2] {
    if let Some(a)=map.get(
        &Game{players:players_orig,score:score_orig,turn:turn}) {
        return *a;
    }
    let mut wins = [0,0];
    for (dice,ways) in QUANT.iter() {
        let mut players = players_orig.clone();
        let mut score = score_orig.clone();
        players[turn] = (players[turn]+dice)%10;
        score[turn] += 1 + players[turn] as usize;

        if score[turn] >= 21 {
            wins[turn]+=ways;
            continue
        }
        let a = play(&mut map,
            Game{players:players,score:score,turn:1-turn});
        wins[turn]+= a[turn]*ways;
        wins[1-turn]+= a[1-turn]*ways;
    }
    map.insert(
        Game{players:players_orig,score:score_orig,turn:turn},
        wins);
    wins
}

fn solve2(input:&Input) -> usize {
    let mut map : HashMap<Game,[usize;2]>= HashMap::new();
    let a = play(&mut map,Game{players:[input[0]-1,input[1]-1],score:[0,0],turn:0});
    return *a.iter().max().unwrap();
}

fn solve1(input:&Input) -> usize {
    let mut players = input.clone();
    let mut score : [usize;2] = [0,0];
    let mut state : usize= 0;
    let winner;
    'outer: loop {
        for turn in 0..input.len(){
            let mut dice : u16 = 0;
            state +=1;
            dice += (state%100) as u16;
            state +=1;
            dice += (state%100) as u16;
            state +=1;
            dice += (state%100) as u16;

            players[turn] = (players[turn]+dice -1)%10 +1;
            score[turn] += players[turn] as usize;
            if score[turn] >= 1000 {
                winner = turn;
                break 'outer;
            }
        }
    }
    state * score[1-winner]
}

pub const SAMPLE: &str =
"Player 1 starting position: 4
Player 2 starting position: 8
";
