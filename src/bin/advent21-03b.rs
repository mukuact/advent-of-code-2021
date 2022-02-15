use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let mut oxygen = None;
    {
        let mut first = 0;
        let mut last = counter.len();
        let mut middle = 2usize.pow((len-1).try_into().unwrap());
        for i in (0..len).rev() {
            println!("{:b}, {:b}, {:b}", first, middle, last);
            let zero: isize = counter[first..middle].iter().sum();
            let one: isize = counter[middle..last].iter().sum();
            println!("{},{}", zero, one);
            if zero == 0 && one == 1 {
                let a = counter[middle..last].iter().position(|&x| x == 1);
                oxygen = a.map(|x| middle + x);
                break;
            } else if zero == 1 && one == 0 {
                let a = counter[first..middle].iter().position(|&x| x == 1);
                oxygen = a.map(|x| first + x);
                break;
            } else if zero == 1 && one == 1 {
                let a = counter[middle..last].iter().position(|&x| x == 1);
                oxygen = a.map(|x| middle + x);
                break;
            }
            if zero > one {
                last = middle;
                middle = first + 2usize.pow((i-1).try_into().unwrap());
            } else {
                first = middle;
                middle  = first + 2usize.pow((i-1).try_into().unwrap());
            }
        }
    }
    let mut co2 = None;
    {
        let mut first = 0;
        let mut last = counter.len();
        let mut middle = 2usize.pow((len-1).try_into().unwrap());
        for i in (0..len).rev() {
            println!("{:b}, {:b}, {:b}", first, middle, last);
            let zero: isize = counter[first..middle].iter().sum();
            let one: isize = counter[middle..last].iter().sum();
            println!("{},{}", zero, one);
            if zero == 0 && one == 1 {
                let a = counter[middle..last].iter().position(|&x| x == 1);
                co2 = a.map(|x| middle + x);
                break;
            } else if zero == 1 && one == 0 {
                let a = counter[first..middle].iter().position(|&x| x == 1);
                co2 = a.map(|x| first + x);
                break;
            } else if zero == 1 && one == 1 {
                let a = counter[first..middle].iter().position(|&x| x == 1);
                co2 = a.map(|x| first + x);
                break;
            }
            if zero <= one {
                last = middle;
                middle = first + 2usize.pow((i-1).try_into().unwrap());
            } else {
                first = middle;
                middle  = first + 2usize.pow((i-1).try_into().unwrap());
            }
        }
    }
    println!("{}", oxygen.unwrap());
    println!("{}", co2.unwrap());
    println!("answer: {}", oxygen.unwrap() * co2.unwrap());
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
