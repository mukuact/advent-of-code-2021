use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("inputs/input1-1.txt"){
        let mut num_z = isize::MAX;
        let mut num_zz = isize::MAX;
        let mut num_zzz = isize::MAX;
        let mut count = 0;
        for line in lines {
            if let Ok(l) = line {
                if let Ok(num) = l.parse::<isize>() {
                    if num_zzz < num {
                        count += 1;
                    }
                    num_zzz = num_zz;
                    num_zz = num_z;
                    num_z = num;
                }
            }
        }
        println!("{}", count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
