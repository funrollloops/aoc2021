use regex::Regex;
use std::io;
use std::io::BufRead;

fn run() -> io::Result<i32> {
    let mut grid = [[0; 1000]; 1000];
    let mut cnt = 0;
    let stdin = io::stdin();
    let parser = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let m = parser.captures(line.trim()).unwrap();
        let mut x1: i32 = m[1].parse().unwrap();
        let mut y1: i32 = m[2].parse().unwrap();
        let x2: i32 = m[3].parse().unwrap();
        let y2: i32 = m[4].parse().unwrap();
        println!("\"{}\" {} {} -- {} {}", line, x1, y1, x2, y2);
        loop {
            let pos = &mut grid[x1 as usize][y1 as usize];
            *pos += 1;
            if *pos == 2 {
                cnt += 1;
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            x1 += (x2-x1).signum();
            y1 += (y2-y1).signum();
        }
    }
    Ok(cnt)
}

fn main() -> io::Result<()> {
    println!("intersections = {}", run()?);
    Ok(())
}
