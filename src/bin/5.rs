use regex::Regex;
use std::io;
use std::io::BufRead;

type Coord = usize;

fn run() -> io::Result<i32> {
    let mut grid = [[0; 1000]; 1000];
    let mut cnt = 0;
    let stdin = io::stdin();
    let parser = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let m = parser.captures(line.trim()).unwrap();
        let mut x1: Coord = m[1].parse().unwrap();
        let mut y1: Coord = m[2].parse().unwrap();
        let x2: Coord = m[3].parse().unwrap();
        let y2: Coord = m[4].parse().unwrap();
        println!("\"{}\" {} {} -- {} {}", line, x1, y1, x2, y2);
        loop {
            grid[x1][y1] += 1;
            if grid[x1][y1] == 2 {
                cnt += 1;
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            if x1 < x2 {
                x1 += 1;
            } else if x1 > x2 {
                x1 -= 1;
            }
            if y1 < y2 {
                y1 += 1;
            } else if y1 > y2 {
                y1 -= 1;
            }
        }
    }
    Ok(cnt)
}

fn main() -> io::Result<()> {
    println!("intersections = {}", run()?);
    Ok(())
}
