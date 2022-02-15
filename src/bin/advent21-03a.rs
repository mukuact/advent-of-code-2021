use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main(){
    let file = read_lines("inputs/input3.txt").unwrap();
    let mut iter = file.peekable();
    let len = iter.peek().unwrap().as_ref().unwrap().chars().count();
    let mut counter = vec![0; len];
    let mut linenum = 0;
    for line in iter {
        if let Ok(l) = line {
            for (i, c) in l.chars().enumerate() {
                if c == '1' {
                    counter[i] += 1;
                }
            }
            linenum += 1;
        }
    }
    let mut gamma = 0;
    for (i, disit) in counter.iter().enumerate() {
        if *disit >= linenum - *disit {
            gamma += 2i64.pow((len - i - 1).try_into().unwrap());
        }
    }
    let epsilon = 2i64.pow(len.try_into().unwrap()) - 1 - gamma;
    println!("{:b}", gamma);
    println!("{:b}", epsilon);
    println!("{}", gamma*epsilon);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
