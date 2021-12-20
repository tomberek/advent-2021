use rulinalg::matrix::Matrix;
use rulinalg::matrix::BaseMatrix;


aoc_harness::aoc_main!(2021 day 20, generator parse_input,
    part1 [solve1] => 5326,
    part2 [solve2] => 17096,
    example part1 SAMPLE => 35,
    example part2 SAMPLE => 3351,
    );

type Line = Vec<bool>;
type Grid = Matrix<bool>;
type Input = (Line,Grid);

fn parse_input(input: &str) -> Input {
    let mut inp = input.lines();
    let line : Vec<bool> = inp.next().unwrap().chars().map(|x|x=='#').collect();
    inp.next();
    let grid = inp.map(|line|{
        line.chars().map(|x|x=='#').collect()
    }).collect::<Vec<Vec<_>>>();

    return (line,Matrix::new(grid.len(),grid[0].len(),grid.iter().flatten().copied().collect::<Vec<_>>()))
}
fn solve1(input:&Input) -> usize {
    solve(input,2)
}
fn solve2(input:&Input) -> usize {
    solve(input,50)
}
fn solve((line,grid):&Input,iter:usize) -> usize {
    let amount = 2;
    let mut output = grid.clone();
    output = Matrix::from_fn(output.rows()+amount*2,output.cols()+amount*2,|col,row|
        if row < amount || col <amount ||
           row >= output.rows() + amount ||
           col >= output.cols() + amount {
            return false
        } else {
            grid[[row-amount,col-amount]]
        }
    );

    let amount = 1;
    for offset in (1..=iter).step_by(1) {
        output = Matrix::from_fn(output.rows()+amount*2,output.cols()+amount*2,|col,row|{
            let row = row - amount;
            let col = col - amount;
            let array = [
                (row-1 ,col-1 ),
                (row-1 ,col   ),
                (row-1 ,col+1 ),
                (row   ,col-1 ),
                (row   ,col   ),
                (row   ,col+1 ),
                (row+1 ,col-1 ),
                (row+1 ,col   ),
                (row+1 ,col+1 ),
            ].into_iter()
                .map(|(a,b)|{
                    let c = a<2 || b<2 || a>=output.rows() || b>=output.cols();
                    (c && line[0] && offset%2 == 0 && !line[511]) || (!c && output[[a,b]])
                })
                .fold(0,|acc,x|acc<<1 | x as usize );
                line[array]
        });
        //show(&output);
        //println!("{}", &output.iter().filter(|x|**x).count());
    }
    return output.iter().filter(|x|**x).count()
}
fn show(m:&Matrix<bool>){
    m.row_iter().for_each(|row|{
        row.iter().for_each(|x|{
            if *x{
                print!("#");
            } else {
                print!(".");
            }
        });
        println!("");
    });
    println!("");
}

pub const SAMPLE: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";
