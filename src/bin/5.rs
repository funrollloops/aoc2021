use regex::Regex;
use std::io;
use std::io::BufRead;
use std::mem;

type Coord = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ALine {
    pos: Coord,
    range: (Coord, Coord)
}

#[derive(Debug)]
struct Input {
    horiz: Vec<ALine>,
    vert: Vec<ALine>,
}

fn parse_input() -> io::Result<Input> {
    let mut parsed = Input {
        horiz: Vec::new(),
        vert: Vec::new(),
    };
    let stdin = io::stdin();
    let parser = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let m = parser.captures(line.trim()).unwrap();
        let mut x1: Coord = m[1].parse().unwrap();
        let mut y1: Coord = m[2].parse().unwrap();
        let mut x2: Coord = m[3].parse().unwrap();
        let mut y2: Coord = m[4].parse().unwrap();
        println!("\"{}\" {} {} -- {} {}", line, x1, y1, x2, y2);
        if x1 == x2 {
            if y1 > y2 {
                mem::swap(&mut y1, &mut y2);
            }
            parsed.vert.push(ALine{pos: x1, range: (y1, y2)})
        } else if y1 == y2 {
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
            }
            parsed.horiz.push(ALine{pos: y1, range: (x1, x2)})
        } else {
            println!("ignoring non-horizontal/vertical line");
        }
    }
    parsed.horiz.sort();
    parsed.vert.sort();
    Ok(parsed)
}

fn intersections(vs: &Vec<ALine>, hs: &Vec<ALine>) -> i32 {
    let mut grid = [[0; 1000]; 1000];
    let mut cnt = 0;
    for h in hs {
        for x in h.range.0..=h.range.1 {
            grid[x][h.pos] += 1;
            if grid[x][h.pos] == 2  { cnt += 1; }
        }
    }
    for v in vs {
        for y in v.range.0..=v.range.1 {
            grid[v.pos][y] += 1;
            if grid[v.pos][y] == 2  { cnt += 1; }
        }
    }
    cnt
}

fn main() -> io::Result<()> {
    let input = parse_input()?;
    println!("{:?}", input);
    let intersections = intersections(&input.vert, &input.horiz);
    println!(
        "intersections={}",
        intersections,
    );
    Ok(())
}
