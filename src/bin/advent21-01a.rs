use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        let count = count_incrased(lines);
        println!("{}", count.unwrap());
    }
}

fn count_incrased(lines: io::Lines<io::BufReader<File>>) -> Result<isize, io::Error> {
    let mut prev_num = isize::MAX;
    let mut count = 0;
    for line in lines {
        if let Ok(l) = line {
            if let Ok(num) = l.parse::<isize>() {
                if prev_num < num {
                    count += 1;
                }
                prev_num = num;
            }
        }
    }
    Ok(count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
