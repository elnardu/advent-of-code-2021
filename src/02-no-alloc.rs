use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

mod alloc;
use alloc::PanicAlloc;

#[global_allocator]
static GLOBAL: PanicAlloc = PanicAlloc;

struct BufReader<T, const N: usize> {
    inner: T,
    buf: [u8; N],
    pos: usize,
    cap: usize,
}

impl<T: io::Read, const N: usize> BufReader<T, N> {
    fn new(reader: T) -> Self {
        Self {
            inner: reader,
            buf: [0u8; N],
            pos: 0,
            cap: 0,
        }
    }
}

impl<T: io::Read, const N: usize> io::Read for BufReader<T, N> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut our_buf = self.fill_buf()?;
        let nread = our_buf.read(buf)?;
        self.consume(nread);
        Ok(nread)
    }
}

impl<T: io::Read, const N: usize> io::BufRead for BufReader<T, N> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        // pretty much this https://doc.rust-lang.org/src/std/io/buffered/bufreader.rs.html#313-331
        if self.pos >= self.cap {
            self.cap = self.inner.read(&mut self.buf)?;
            self.pos = 0
        }
        Ok(&self.buf[self.pos..self.cap])
    }

    fn consume(&mut self, amt: usize) {
        self.pos += amt;
    }
}

fn read_line<T: io::BufRead>(buf_read: &mut T, str_buf: &mut [u8]) -> io::Result<usize> {
    let mut i = 0;
    loop {
        let buf = buf_read.fill_buf()?;
        if buf.len() == 0 {
            // nothing left
            return Ok(0);
        }

        let mut j = 0;
        let mut got_string = false;

        while i < str_buf.len() && j < buf.len() {
            if buf[j] != b'\n' {
                str_buf[i] = buf[j];
                i += 1;
                j += 1;
            } else {
                got_string = true;
                j += 1;
                break;
            }
        }

        if i >= str_buf.len() {
            panic!("Buffer is too small to fit the line")
        }

        buf_read.consume(j);
        if got_string {
            return Ok(i);
        }
    }
}

fn part1() {
    let input = fs::File::open("./input/02").unwrap();
    let result: usize;
    without_alloc!({
        let mut buf_read = BufReader::<_, 128>::new(input);
        let mut str_buf = [0u8; 128];

        let mut horiz_pos = 0;
        let mut depth = 0;

        loop {
            let size = read_line(&mut buf_read, &mut str_buf).unwrap();
            if size == 0 {
                break;
            }

            let string = std::str::from_utf8(&str_buf[0..size]).unwrap();

            if let Some(string) = string.strip_prefix("forward ") {
                let num = usize::from_str(string).unwrap();
                horiz_pos += num;
            } else if let Some(string) = string.strip_prefix("down ") {
                let num = usize::from_str(string).unwrap();
                depth += num;
            } else if let Some(string) = string.strip_prefix("up ") {
                let num = usize::from_str(string).unwrap();
                depth -= num;
            } else {
                panic!("Invalid step")
            }
        }

        result = horiz_pos * depth;
    });
    println!("Part 1 result: {}", result);
}

fn part2() {
    let input = fs::File::open("./input/02").unwrap();
    let result: i64;
    without_alloc!({
        let mut buf_read = BufReader::<_, 128>::new(input);
        let mut str_buf = [0u8; 128];

        let mut horiz_pos = 0;
        let mut depth = 0;
        let mut aim = 0;

        loop {
            let size = read_line(&mut buf_read, &mut str_buf).unwrap();
            if size == 0 {
                break;
            }

            let string = std::str::from_utf8(&str_buf[0..size]).unwrap();

            if let Some(string) = string.strip_prefix("forward ") {
                let num = i64::from_str(string).unwrap();
                horiz_pos += num;
                depth += aim * num;
            } else if let Some(string) = string.strip_prefix("down ") {
                let num = i64::from_str(string).unwrap();
                aim += num;
            } else if let Some(string) = string.strip_prefix("up ") {
                let num = i64::from_str(string).unwrap();
                aim -= num;
            } else {
                panic!("Invalid step")
            }
        }

        result = horiz_pos * depth;
    });
    println!("Part 2 result: {}", result);
}

fn main() {
    part1();
    part2();
}
