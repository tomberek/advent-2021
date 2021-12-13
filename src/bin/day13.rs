use text_io::scan;
use ndarray::*;

aoc_harness_macros::aoc_main!(2021 day 13, generator parse_input,
    part1 [solve1] => 788,
    part2 [solve2] ,
    example part1 SAMPLE => 17,
    example part2 SAMPLE => 16,
    );

type Grid = Vec<[usize;2]>;
type Folds = Vec<(String,usize)>;
type Input = (Grid,Folds);

fn parse_input(input: &str) -> Input {
    let mut lines = input.split("\n\n");
    let points = lines.next().unwrap().lines().map(|a|{
        let x :usize;
        let y :usize;
        scan!(a.bytes() => "{},{}",x,y);
        [x,y]
    }).collect::<Vec<_>>();
    let folds = lines.next().unwrap().lines().map(|x|{
        let dir :String;
        let line :usize;
        scan!(x.bytes() => "fold along {}={}",dir,line);
        (dir,line)
    }).collect();
    (points,folds)
}
fn solve1(input:&Input) -> usize {
    solve(input,1)
}
fn solve2(input:&Input) -> usize {
    solve(input,input.1.len())
}

#[allow(dead_code)]
fn print(points:&Grid) {
    let xmax = points.iter().map(|a|a[0]).max().unwrap()+1;
    let ymax = points.iter().map(|a|a[1]).max().unwrap()+1;
    let mut arr = Array2::zeros((ymax,xmax));
    points.iter().for_each(|a|{arr[[a[1],a[0]]]=1;});
    arr.outer_iter().for_each(|row|{
        row.iter()
            .map(|&a|
                if a==0 {'.'}else{'#'})
            .for_each(|c|{
            print!("{}",c);
        });
        println!("");
    });
}

fn solve((arr,folds):&Input,iter:usize) -> usize {
    let mut arr = arr.clone();
    (0..iter).for_each(|n|{
        let (dir,line) = &folds[n];
        let (original,folded) : (Grid,Grid) = match dir.as_str() {
            "y" => {
                let a : (Grid,Grid) = arr.iter().partition(|a|a[1]<*line);
                let b = a.1.iter().map(|[i,j]|[*i,2*line-j]).collect();
                (a.0,b)
            },
            "x" => {
                let a : (Grid,Grid) = arr.iter().partition(|a|a[0]<*line);
                let b = a.1.iter().map(|[i,j]|[2*line-i,*j]).collect();
                (a.0,b)
            },
            _ => panic!(),
        };
        arr = original.iter().chain(folded.iter()).copied().collect();
    });
    arr.sort();
    arr.dedup();
    //print(&arr);
    return arr.len();
}

pub const SAMPLE: &str =
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
