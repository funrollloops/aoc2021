use std::io;
use std::io::BufRead;

use regex::Regex;

fn main() -> io::Result<()> {
    let re: Regex = Regex::new("(\\w+) (\\d+)").unwrap();
    let stdin = io::stdin();
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut horiz: i64 = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let cap = re.captures(line.trim()).unwrap();
        let arg1: i64 = cap[2].parse().unwrap();
        print!("{}\t{}\t{}", line, &cap[1], arg1);
        match &cap[1] {
            "forward" => { horiz += arg1; depth += aim*arg1 }
            "down" => aim += arg1,
            "up" => aim -= arg1,
            x => {
                println!("arg0 invalid={}", x);
                panic!();
            }
        }
        println!("\t\tdepth={} * horiz={} => {}", depth, horiz, depth * horiz);
    }
    println!("depth={} * horiz={} => {}", depth, horiz, depth * horiz);
    Ok(())
}
