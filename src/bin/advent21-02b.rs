use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    let file = read_lines("inputs/input1-2.txt").unwrap();
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;
    for line in file {
        if let Ok(l) = line {
            let mut a = l.split_whitespace();
            let dir = a.next();
            let num: Option<isize> = a.next().and_then(|x| x.parse().ok());
            match dir {
                Some("forward") => {
                    x += num.unwrap();
                    z += aim * num.unwrap();
                },
                Some("down") => aim += num.unwrap(),
                Some("up") => aim -= num.unwrap(),
                _ => ()
            }

        }
    }
    println!("{}", x * z );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
