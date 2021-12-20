// use ndarray::*;
//use ndarray_linalg::*;
// use rulinalg::norm::Euclidean;
use std::collections::{HashSet,HashMap};
use itertools::Itertools;

use rulinalg::matrix::Matrix;
use rulinalg::matrix::BaseMatrix;


aoc_harness::aoc_main!(2021 day 19, generator parse_input,
    part1 [solve1] => 338,
    part2 [solve2] => 9862,
    example part1 SAMPLE => 79,
    example part2 SAMPLE => 3621,
    );

type I = i64;

type Input = Vec<Scan>;
type Scan = Matrix<I>;


fn parse_input(input: &str) -> Input {
    input.split("\n\n").map(|x|{
        let mut scanner = x.trim().lines();
        scanner.next();
        let out = scanner.flat_map(|line|{
            line.split(",").map(|num|{
                num.parse().unwrap()
            }).chain([1.0 as I ].into_iter()).collect::<Vec<I>>()
        }).collect::<Vec<_>>();
        let arr = Matrix::new(out.len()/4,4,out);
        arr
    }).collect()
}

fn solve2(input:&Input) -> I {
    solver(input).1
}
fn solve1(input:&Input) -> I {
    solver(input).0
}
fn solver(input:&Input) -> (I,I) {
    let mut inp = input.iter();
    let mut done : Vec<Matrix<I>> = vec![inp.next().map(|x|x.clone()).unwrap()];
    let mut beacons : Vec<Matrix<I>> = done[0].row_iter().map(|x|x.into_matrix()).collect();

    let mut input : Vec<Matrix<I>> = inp.map(|x|x.clone()).collect::<Vec<_>>();
    let mut locs : Vec<Matrix<I>> = vec![Matrix::zeros(1,4)];

    while !input.is_empty() {
        let old_len = input.len();
        let mut next = Vec::new();
        let l = done.len();
        for scanner in &input {
            let mut flag = false;
            for i in 0..l {
                let first = done[i].clone();
                if let Some(a) = solve(&first,&scanner){

                    locs = locs.iter().map(|b|(b * &a)).collect();
                    let loc = 
                         a.sub_slice([3,0],1,4).into_matrix();
                    locs.push(loc);
                    beacons = beacons.iter().map(|b|b * &a).collect();
                    beacons.extend(scanner.row_iter().map(|x|x.into_matrix()));
                    done = done.iter().map(|b|b * &a).collect();
                    done.push(scanner.clone());
                    flag = true;
                    break;
                }
            }
            if !flag {
                    next.push(scanner.clone());
            }
        }
        input = next;
        if input.len() == 0 { break;}
        //println!("{}",input.len());
        if input.len() >= old_len { panic!("no progress") }
    }
    let mut b :Vec<Vec<I>> = beacons.iter().map(|x|x.iter().copied().collect()).collect();
    b.sort();
    b.dedup();
    let mut max = 0;
    locs.iter().cartesian_product(locs.clone().iter()).for_each(|(a,b)|{
        let t = (a - b).iter().map(|x|x.abs()).sum::<I>();
        if t > max { max = t -1;}
    });
    //println!("max {}",max);
    return (b.len() as I,max as I)
}

fn solve(a:&Scan,b:&Scan) -> Option<Matrix<I>> {
    let d = Matrix::from_fn(a.rows(),a.rows(),|col,row|{
        if col > row {
            let c = a.row(col).into_matrix() - a.row(row).into_matrix();
            c.elemul(&c).sum() as I
        } else {0}
    });
    let mut a_map = HashMap::new();
    d.iter().enumerate().filter(|(_,a)|**a>1).map(|(i,a)|{
        a_map.insert(*a as I,i as I);
    }).for_each(drop);

    let d = Matrix::from_fn(b.rows(),b.rows(),|col,row|{
        if col > row {
            let c = b.row(col).into_matrix() - b.row(row).into_matrix();
            c.elemul(&c).sum() as I
        } else {0}
    });
    let mut b_map = HashMap::new();
    d.iter().enumerate().filter(|(_,a)|**a>1).for_each(|(i,a)|{
        b_map.insert(*a as I,i as I);
    });
    let a_set = a_map.keys().copied().filter(|&x|x>1).collect::<HashSet<_>>();
    let b_set = b_map.keys().copied().filter(|&x|x>1).collect::<HashSet<_>>();
    let res = a_set.intersection(&b_set).copied().collect::<HashSet<_>>();

    let mut res_stable = res.iter().collect::<Vec<_>>();
    res_stable.sort_unstable();
    res_stable.dedup();

    if res_stable.len() < 12 {
        return None
    }
    res_stable = res_stable.iter().rev().take(12).copied().collect();

    let mut tempa2 : Vec<Vec<I>> = Vec::new();
    let mut tempa :  Vec<Vec<I>> = Vec::new();
    let mut tempb2 : Vec<Vec<I>> = Vec::new();
    let mut tempb :  Vec<Vec<I>> = Vec::new();
    res_stable.iter().for_each(|val|{
        let x = a.row(*a_map.get(val).unwrap() as usize %a.rows()).into_matrix();
        let y = a.row(*a_map.get(val).unwrap() as usize /a.rows()).into_matrix();
        tempa2.push(x.iter().copied().collect());
        tempa2.push(y.iter().copied().collect());
        tempa.push((x + &y)
                .iter().copied().collect::<Vec<I>>());

        let x = b.row(*b_map.get(val).unwrap() as usize %b.rows()).into_matrix();
        let y = b.row(*b_map.get(val).unwrap() as usize /b.rows()).into_matrix();
        tempb2.push(x.iter().copied().collect());
        tempb2.push(y.iter().copied().collect());
        tempb.push((x + &y)
                .iter().copied().collect::<Vec<I>>());
    });

    let mat_a = Matrix::new(tempa.len(),4,tempa.into_iter().flatten()
        .map(|x|x as f64 / 2.0)
        .collect::<Vec<_>>());
    let mat_b = Matrix::new(tempb.len(),4,tempb.into_iter().flatten()
        .map(|x|x as f64 / 2.0)
        .collect::<Vec<_>>());

    // The actual work
    let x = mat_a.transpose() * &mat_a;
    let c = x.inverse().unwrap();
    let d = (c.clone() * mat_a.transpose()) * mat_b.clone();

    let x = Matrix::from_fn(d.rows(),d.cols(),|row,col|{
        d[[col,row]].round() as I
    });

    // Check our work
    let mat_a = Matrix::new(tempa2.len(),4,tempa2.into_iter().flatten()
        .collect::<Vec<_>>());
    let mat_b = Matrix::new(tempb2.len(),4,tempb2.into_iter().flatten()
        .collect::<Vec<_>>());
    let ret = mat_a.clone() * x.clone();
    for i in 0..ret.rows() {
        if ret[[i,0]] != mat_b[[i&0xFFFE,0]] && ret[[i,0]] != mat_b[[i&0xFFFE | 1,0]]  {
            return None
        }
    }

    return Some(x)
}

pub const SAMPLE: &str =
"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
