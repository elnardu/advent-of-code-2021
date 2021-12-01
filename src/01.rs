use itertools::Itertools;
use std::fs;
use std::io;

struct NumIter<T: io::BufRead> {
    reader: T,
    buf: String,
}

impl<T: io::BufRead> NumIter<T> {
    fn new(reader: T) -> Self {
        Self {
            reader,
            buf: String::with_capacity(32),
        }
    }
}

impl<T: io::BufRead> Iterator for NumIter<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        let read = self.reader.read_line(&mut self.buf).unwrap();
        if read == 0 {
            None
        } else {
            Some(self.buf[..self.buf.len() - 1].parse().unwrap())
        }
    }
}

fn part1() {
    let input = fs::File::open("./input/01").unwrap();
    let reader = io::BufReader::new(input);
    let iter = NumIter::new(reader);
    let result: usize = iter
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();

    println!("Part 1 result: {}", result);
}

fn part2() {
    let input = fs::File::open("./input/01").unwrap();
    let reader = io::BufReader::new(input);
    let iter = NumIter::new(reader);
    let result: usize = iter
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| if a < b { 1 } else { 0 })
        .sum();

    println!("Part 2 result: {}", result);
}

fn main() {
    part1();
    part2();
}
