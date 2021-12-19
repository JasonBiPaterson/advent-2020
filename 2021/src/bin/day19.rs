use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Beacon = ndarray::Array<i32, ndarray::Dim<[usize; 1]>>;

fn parse_input(input: &str) -> Vec<Vec<Beacon>> {
    input
        .trim()
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|line| {
                    line.trim()
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<i32>>()
                })
                .map(|v| ndarray::arr1(&v))
                .collect()
        })
        .collect()
}

fn beacon_diff(full_scan: &mut HashSet<Beacon>, scan2: &[Beacon], n: usize) -> Option<Beacon> {
    let mut diff_count = HashMap::new();
    for beacon1 in full_scan.iter() {
        for beacon2 in scan2 {
            let diff = beacon1 - beacon2;
            let count = diff_count.entry(diff.clone()).or_insert(0);
            *count += 1;
            if *count >= n {
                return Some(diff);
            }
        }
    }
    None
}

fn scan_rotations(scan: &[Beacon]) -> Vec<Vec<Beacon>> {
    let matrices = &[
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
        [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
        [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
        [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
        [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
        [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
        [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
        [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
        [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
        [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    ]
    .iter()
    .map(|m| ndarray::arr2(m));

    matrices
        .clone()
        .into_iter()
        .map(|m| {
            scan.to_owned()
                .into_iter()
                .map(|b| m.dot(&b))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn any_common_rotation(full_scan: &mut HashSet<Beacon>, scan2: &[Beacon]) -> Option<Beacon> {
    for s2 in scan_rotations(scan2) {
        if let Some(diff) = beacon_diff(full_scan, &s2, 12) {
            for beacon in s2 {
                full_scan.insert(beacon + diff.to_owned());
            }
            return Some(diff);
        }
    }
    None
}

fn solve(parsed_input: &[Vec<Beacon>]) -> (usize, i32) {
    let mut beacons = HashSet::new();
    for beacon in &parsed_input[0] {
        beacons.insert(beacon.to_owned());
    }

    let mut scans = HashSet::new();
    for scan in &parsed_input[1..] {
        scans.insert(scan.clone());
    }

    let mut scan_pos = vec![];

    while !scans.is_empty() {
        let mut next_scans = scans.clone();
        for scan in &scans {
            if let Some(pos) = any_common_rotation(&mut beacons, scan) {
                next_scans.remove(scan);
                scan_pos.push(pos);
            }
        }
        scans = next_scans;
    }

    let part1 = beacons.len();

    let part2 = scan_pos
        .clone()
        .iter()
        .flat_map(|pos1| {
            scan_pos
                .clone()
                .iter()
                .map(move |pos2| (pos1 - pos2).iter().map(|&n| n.abs()).sum())
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();

    (part1, part2)
}

fn main() {
    let input: String = fs::read_to_string("input/day19").expect("Failed to read input");

    let parsed_input = parse_input(&input);

    let (part1, part2) = solve(&parsed_input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
--- scanner 0 ---
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

    #[test]
    fn test_part1() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(solve(&parsed_input).0, 79);
    }

    #[test]
    fn test_part2() {
        let parsed_input = parse_input(INPUT);
        assert_eq!(solve(&parsed_input).1, 3621);
    }
}
