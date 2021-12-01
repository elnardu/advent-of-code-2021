use itertools::Itertools;
use std::alloc::{GlobalAlloc, Layout, System};
use std::fs;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

static ARE_ALLOCATIONS_ALLOWED: AtomicBool = AtomicBool::new(true);

macro_rules! without_alloc {
    ($b:block) => {
        ARE_ALLOCATIONS_ALLOWED.store(false, Ordering::Release);
        {
            $b
        }
        ARE_ALLOCATIONS_ALLOWED.store(true, Ordering::Release);
    };
}

struct PanicAlloc;

unsafe impl GlobalAlloc for PanicAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if ARE_ALLOCATIONS_ALLOWED.load(Ordering::Acquire) {
            System.alloc(layout)
        } else {
            ARE_ALLOCATIONS_ALLOWED.store(true, Ordering::Release);
            panic!("No allocations allowed!");
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ARE_ALLOCATIONS_ALLOWED.load(Ordering::Acquire) {
            System.dealloc(ptr, layout)
        } else {
            ARE_ALLOCATIONS_ALLOWED.store(true, Ordering::Release);
            panic!("No deallocations allowed!");
        }
    }
}

#[global_allocator]
static GLOBAL: PanicAlloc = PanicAlloc;

struct NumIter<T> {
    reader: T,
    buf: [u8; 128],
    last_read: usize,
    index: usize,
    acc: usize,
}

impl<T: io::Read> NumIter<T> {
    fn new(reader: T) -> Self {
        Self {
            reader,
            buf: [0; 128],
            last_read: 0,
            index: 0,
            acc: 0,
        }
    }
}

impl<T: io::Read> Iterator for NumIter<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while self.index < self.last_read {
                let x = self.buf[self.index];
                self.index += 1;
                match x {
                    b'0'..=b'9' => {
                        self.acc *= 10;
                        self.acc += (x - b'0') as usize;
                    }
                    b'\n' => {
                        let res = self.acc;
                        self.acc = 0;
                        return Some(res);
                    }
                    _ => unreachable!(),
                }
            }

            let result = self.reader.read(&mut self.buf).unwrap();
            if result == 0 {
                return None;
            }

            self.last_read = result;
            self.index = 0;
        }
    }
}

fn part1() {
    let input = fs::File::open("./input/01").unwrap();
    let result: usize;
    without_alloc!({
        let iter = NumIter::new(input);
        result = iter
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .sum();
    });
    println!("Part 1 result: {}", result);
}

fn part2() {
    let input = fs::File::open("./input/01").unwrap();
    let result: usize;
    without_alloc!({
        let iter = NumIter::new(input);
        result = iter
            .tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .sum();
    });
    println!("Part 2 result: {}", result);
}

fn main() {
    part1();
    part2();
}
