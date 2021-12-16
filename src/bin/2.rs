use std::io;
use std::io::BufRead;

use regex::Regex;

fn main() -> io::Result<()> {
    let re: Regex = Regex::new("(\\w+) (\\d+)").unwrap();
    let stdin = io::stdin();
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut horiz: i64 = 0;
    for line_or in stdin.lock().lines() {
        let line = line_or.unwrap();
        let cap = re.captures(&line).unwrap();
        let arg0 = cap[1].chars().nth(0).unwrap();
        let arg1: i64 = cap[2].parse::<i64>().unwrap();
        println!("{}\t{} {}", line, arg0, arg1);
        if arg0 == 'f' { horiz += arg1; depth += aim*arg1; }
        else if arg0 == 'd' { aim += arg1; }
        else if arg0 == 'u' { aim -= arg1; }
        else {
            println!("arg0 invalid={}", arg0);  
            panic!();
        }
        println!("\t\tdepth={} * horiz={} => {}", depth, horiz, depth * horiz);
    }
    println!("depth={} * horiz={} => {}", depth, horiz, depth * horiz);
    Ok(())
}
