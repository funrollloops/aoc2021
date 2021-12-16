use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut cnt: i32 = 0;
    let mut n0: i32 = 999999999;
    let mut n1: i32 = 999999999;
    let mut n2: i32 = 999999999;
    for line in stdin.lock().lines() {
        let val: i32 = line.unwrap().parse().unwrap();
        if val > n0 {
            println!("+ {}", val);
            cnt += 1;
        } else {
            println!("  {}", val);
        }
        n0 = n1;
        n1 = n2;
        n2 = val;
    }
    println!("=== increases: {}", cnt);
    Ok(())
}
