use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut lines = read_lines("./inputs/input7.txt").unwrap();
    let first = lines.next();
    let input: Option<Vec<isize>> = first.as_ref().map(|x| {
        x.as_ref()
            .iter()
            .flat_map(|x| x.split(','))
            .filter_map(|x| x.parse().ok())
            .collect()
    });
    if let Some(v) = input {
        let fuel: Option<isize> = (*v.iter().min().unwrap()..*v.iter().max().unwrap())
            .map(|i| v.iter().map(|x| (x - i).abs()).sum())
            .min();
        println!("{}", fuel.unwrap());

        let fuel2: Option<isize> = (*v.iter().min().unwrap()..*v.iter().max().unwrap())
            .map(|i| {
                v.iter()
                    .map(|x| {
                        let diff = (x - i).abs();
                        diff * (diff + 1) / 2
                    })
                    .sum()
            })
            .min();
        println!("{}", fuel2.unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
