aoc_harness_macros::aoc_main!(2021 day 4, generator parse_input,
    part1 [part1] => 58838,
    part2 [part2] => 6256,
    example part1 SAMPLE => 4512,
    example part2 SAMPLE => 1924,
);

#[derive(Debug,Clone)]
pub struct Board {
    id: usize,
    board: [[i8;5];5],
    board_t: [[i8;5];5],
    winning: bool,
}
type Draw = Vec<usize>;
type Index = Vec<[Option<(usize,usize)>;100]>;
pub struct Game {
    d: Draw,
    boards: Vec<Board>,
    index: Index,
}

fn parse_input(input: &str) -> Game {
    let mut arr = input.lines().clone();
    let draw = arr.next().unwrap().split(",").map(|x|x.parse().unwrap()).collect();
    let mut boards : Vec<Board> = Vec::new();
    let mut index : Index = Vec::new();
    let mut bnum = 0;
    loop {
       if let None = arr.next(){
           break
       }
       let mut b : Board = Board{
           id: bnum,
           board: [[0;5];5],
           board_t: [[0;5];5],
           winning: false,
       };
       for row in 0..5 {
           let line = arr.next().unwrap();
           index.push([None;100]);
           b.board[row]
               .copy_from_slice(&line.split_whitespace()
               .enumerate()
               .map(|(column,x)|{
                   let num = x.parse().unwrap();
                   index[bnum as usize][num as usize]=Some((row,column));
                   num
               }).collect::<Vec<i8>>()[..]);

           b.board_t.iter_mut().enumerate().for_each(|(r,row)|{
               row.iter_mut().enumerate().for_each(|(c,item)|
                   *item = b.board[c][r]
           )});
       }
       boards.push(b);
       bnum +=1;
    }
    return Game {
        d: draw,
        boards: boards,
        index: index,
    }
}

fn helper(Game {d,boards,index}: &Game,scorer: &dyn Fn(&mut Vec<Board>) -> Option<u32>) -> u32 {
    let mut boards = boards.clone();
    for d in d.iter(){
        boards.iter_mut().for_each(|b|{
            if let Some((row,col)) = index[b.id][*d] {
                b.board[row][col] = -1;
                b.board_t[col][row] = -1;
                if winning(&mut b.board,row) || winning(&mut b.board_t,col){
                    b.winning = true;
                }
            }
        });
        if let Some(s) = scorer(&mut boards) {
            return s * (*d as u32)
        }
    }
    return 0
}
fn score1(boards: &mut Vec<Board>) -> Option<u32> {
    if let Some(b) = boards.iter().find(|b|b.winning){
        return Some(score(&b.board))
    }
    None
}
fn score2(boards: &mut Vec<Board>) -> Option<u32> {
    if boards.len()!=1 {
        boards.retain(|b|!(b.winning));
        return None
    }
    score1(boards)
}
fn part1(input: &Game) -> u32 {
    helper(input,&score1)
}
fn part2(input: &Game) -> u32 {
    helper(input,&score2)
}
fn score(board:&[[i8;5];5]) -> u32{
    board.into_iter().flatten()
        .filter(|x|**x>0).map(|&x|x as u32).sum::<u32>()
}
// Only check in the row in which a value changed
fn winning(board: &[[i8;5];5],row: usize) -> bool {
    board[row].iter().all(|&b|b<0)
}

const SAMPLE: &str = 
"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
