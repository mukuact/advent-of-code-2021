use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_life_support_rating<F>(
    comp: F,
    v: &Vec<usize>,
    first: usize,
    last: usize,
    i: u32,
) -> Option<usize>
where
    F: Fn(usize, usize) -> bool,
{
    let middle = first + 2usize.pow(i);
    let zero = &v[first..middle];
    let one = &v[middle..last];

    let n_zero: usize = zero.iter().sum();
    let n_one: usize = one.iter().sum();
    println!("{:b}, {:b}, {:b}", first, middle, last);
    println!("{},{}", n_zero, n_one);

    if n_zero == 0 && n_one == 1 {
        let a = one.iter().position(|&x| x == 1);
        return a.map(|x| middle + x);
    } else if n_zero == 1 && n_one == 0 {
        let a = zero.iter().position(|&x| x == 1);
        return a.map(|x| first + x);
    }
    // special treatment for (n_zero == n_one == 1)
    if comp(n_zero, n_one) {
        if i == 0 {
            get_life_support_rating(comp, v, first, middle, 0)
        } else {
            get_life_support_rating(comp, v, first, middle, i - 1)
        }
    } else {
        if i == 0 {
            get_life_support_rating(comp, v, middle, last, 0)
        } else {
            get_life_support_rating(comp, v, middle, last, i - 1)
        }
    }
}

fn main() {
    let file = read_lines("inputs/input3.txt").unwrap();
    let mut iter = file.peekable();
    let len = iter.peek().unwrap().as_ref().unwrap().chars().count();
    let mut counter = vec![0; 2usize.pow(len.try_into().unwrap())];
    for line in iter {
        let line = line.unwrap();
        let intval = usize::from_str_radix(&line, 2).unwrap();
        counter[intval] += 1;
    }

    let co2 = get_life_support_rating(
        |x, y| x > y,
        &counter,
        0,
        counter.len(),
        (len - 1).try_into().unwrap(),
    );
    let oxygen = get_life_support_rating(
        |x, y| x <= y,
        &counter,
        0,
        counter.len(),
        (len - 1).try_into().unwrap(),
    );
    println!("{}", oxygen.unwrap());
    println!("{}", co2.unwrap());
    println!("answer: {}", oxygen.unwrap() * co2.unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
