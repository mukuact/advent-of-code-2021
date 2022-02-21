use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mid_day = 80;
    let last_day = 256;
    let mut lines = read_lines("./inputs/input6.txt").unwrap();
    let mut fishes = vec![0; 7];
    let mut new_fishes = vec![0; 3];
    let input: Vec<usize> = lines
        .next()
        .as_ref()
        .unwrap()
        .iter()
        .flat_map(|x| x.split(','))
        .filter_map(|x| x.parse().ok())
        .collect();
    for i in input.iter() {
        fishes[*i] += 1;
    }

    let six_idx_from_eight_idx = &[1, 2, 0];
    let six_idx_from_zero_idx = &[6, 0, 1, 2, 3, 4, 5];
    let mut day = 0;
    let mut will_born = 0;
    for (zero_idx, eight_idx) in (0..7).cycle().zip((0..3).cycle()) {
        new_fishes[eight_idx] = will_born;
        will_born = fishes[zero_idx];
        fishes[six_idx_from_zero_idx[zero_idx]] += new_fishes[six_idx_from_eight_idx[eight_idx]];
        new_fishes[six_idx_from_eight_idx[eight_idx]] = 0;
        print!("day {}:", day);
        print!("{:?}", fishes);
        println!("{:?}", new_fishes);
        if day == last_day {
            break;
        } else if day == mid_day {
            println!(
                "{}",
                fishes.iter().sum::<usize>() + new_fishes.iter().sum::<usize>()
            );
        }
        day += 1;
    }
    println!(
        "{}",
        fishes.iter().sum::<usize>() + new_fishes.iter().sum::<usize>()
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
