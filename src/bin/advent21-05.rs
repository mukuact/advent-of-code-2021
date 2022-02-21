use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Segment {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl FromIterator<isize> for Segment {
    fn from_iter<I: IntoIterator<Item = isize>>(iter: I) -> Self {
        let nums: Vec<isize> = iter.into_iter().take(4).collect();
        Self {
            x1: nums[0],
            y1: nums[1],
            x2: nums[2],
            y2: nums[3],
        }
    }
}

fn make_map(segs: &Vec<Segment>) -> HashMap<(isize, isize), isize> {
    let mut ret = HashMap::new();
    for s in segs {
        if s.x1 == s.x2 {
            for i in min(s.y1, s.y2)..=max(s.y1, s.y2) {
                *ret.entry((s.x1, i)).or_insert(0) += 1;
            }
        } else if s.y1 == s.y2 {
            for i in min(s.x1, s.x2)..=max(s.x1, s.x2) {
                *ret.entry((i, s.y1)).or_insert(0) += 1;
            }
        }
    }
    ret
}

fn make_diagmap(segs: &Vec<Segment>) -> HashMap<(isize, isize), isize> {
    let mut ret = HashMap::new();
    for s in segs {
        if s.x1 == s.x2 {
            for i in min(s.y1, s.y2)..=max(s.y1, s.y2) {
                *ret.entry((s.x1, i)).or_insert(0) += 1;
            }
        } else if s.y1 == s.y2 {
            for i in min(s.x1, s.x2)..=max(s.x1, s.x2) {
                *ret.entry((i, s.y1)).or_insert(0) += 1;
            }
        } else {
            let left = min(s.x1, s.x2);
            let right = max(s.x1, s.x2);
            if left == s.x1 {
                if s.y1 < s.y2 {
                    for i in 0..=(right - left) {
                        *ret.entry((left + i, s.y1 + i)).or_insert(0) += 1;
                    }
                } else if s.y1 > s.y2 {
                    for i in 0..=(right - left) {
                        *ret.entry((left + i, s.y1 - i)).or_insert(0) += 1;
                    }
                }
            } else {
                if s.y1 < s.y2 {
                    for i in 0..=(right - left) {
                        *ret.entry((left + i, s.y2 - i)).or_insert(0) += 1;
                    }
                } else if s.y1 > s.y2 {
                    for i in 0..=(right - left) {
                        *ret.entry((left + i, s.y2 + i)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    ret
}

fn main() {
    let lines = read_lines("./inputs/input5.txt").unwrap();
    let segments: Vec<Segment> = lines
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .flat_map(|x| x.split(','))
                .filter_map(|x| x.parse::<isize>().ok())
                .collect()
        })
        .collect();
    let map = make_map(&segments);
    let n_overlap = map
        .iter()
        .filter_map(|(_, v)| if *v > 1 { Some(v) } else { None })
        .count();
    println!("{}", n_overlap);

    let diagmap = make_diagmap(&segments);
    let n_overlap2 = diagmap
        .iter()
        .filter_map(|(_, v)| if *v > 1 { Some(v) } else { None })
        .count();
    println!("{}", n_overlap2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
