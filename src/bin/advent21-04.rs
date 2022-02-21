use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct BingoCard {
    numbers: Vec<usize>,
    marked: [bool; 25],
}

struct BingoColumnIter<'a> {
    bingo: &'a BingoCard,
    column: usize,
    i: usize,
}

impl Iterator for BingoColumnIter<'_> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.bingo.marked.get(self.column + 5 * self.i).copied();
        self.i += 1;
        return ret;
    }
}

impl BingoCard {
    fn new(numbers: Vec<usize>) -> Self {
        Self {
            numbers,
            marked: [false; 25],
        }
    }

    fn coliter(&self, column: usize) -> BingoColumnIter {
        BingoColumnIter {
            bingo: &self,
            column,
            i: 0,
        }
    }

    fn rowiter(&self, row: usize) -> &[bool] {
        &self.marked[5 * row..5 * row + 5]
    }

    fn mark(&mut self, n: usize) {
        if let Some(pos) = self.numbers.iter().position(|&x| x == n) {
            self.marked[pos] = true;
        }
    }

    fn check(&self) -> Option<usize> {
        let bingo = (0..5)
            .map(|x| self.coliter(x).all(|y| y) || self.rowiter(x).iter().all(|y| *y))
            .any(|x| x);
        if bingo {
            Some(
                self.numbers
                    .iter()
                    .zip(self.marked.iter())
                    .filter_map(|(&n, &mark)| if !mark { Some(n) } else { None })
                    .sum(),
            )
        } else {
            None
        }
    }
}

impl fmt::Debug for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            write!(f, "{:?}", &self.numbers[i * 5..i * 5 + 5]).unwrap();
            writeln!(f, "   {:?}", &self.rowiter(i)).unwrap();
        }
        write!(f, "{:?}", &self.numbers[4 * 5..4 * 5 + 5]).unwrap();
        writeln!(f, "   {:?}", &self.rowiter(4))
    }
}

fn get_bingo_card(lines: &mut io::Lines<io::BufReader<File>>) -> Option<BingoCard> {
    let _ = lines.next()?;
    let mut numbers: Vec<usize> = Vec::new();
    for l in lines.take(5) {
        numbers.extend(
            l.unwrap()
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok()),
        );
    }
    Some(BingoCard::new(numbers))
}

fn main() {
    let mut lines = read_lines("inputs/input4.txt").unwrap();
    let numbers: Option<Vec<usize>> = lines
        .next()
        .unwrap()
        .ok()
        .map(|x| x.split(',').map(|y| y.parse().unwrap()).collect());

    let mut card_bingo_time = Vec::new();
    let mut j = 0;
    while let Some(mut bingo_card) = get_bingo_card(&mut lines) {
        for (i, n) in numbers.as_ref().unwrap().iter().enumerate() {
            bingo_card.mark(*n);
            if let Some(rest) = bingo_card.check() {
                println!("{}", j);
                println!("{:?}", numbers);
                println!("{:?}", bingo_card);
                card_bingo_time.push((i, n * rest));
                j += 1;
                break;
            }
        }
    }
    let min = card_bingo_time.iter().min_by_key(|x| x.0).unwrap();
    let idx = card_bingo_time.iter().position(|x| x.0 == min.0).unwrap();
    let max = card_bingo_time.iter().max_by_key(|x| x.0).unwrap();
    let idx_max = card_bingo_time.iter().position(|x| x.0 == max.0).unwrap();
    println!("#{:#?}, {:?}", idx, min);
    println!("#{:#?}, {:?}", idx_max, max);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
