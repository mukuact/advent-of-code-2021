use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), io::Error>{
    let lines = read_lines("inputs/input1-1.txt")?;
    let num = lines
        .flat_map(|n| n.unwrap().parse())
        .collect::<Vec<isize>>()
        .windows(2)
        .filter(|[a, b]| a < b)
        .count();
    println!("{}", num);
    Ok(())
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
