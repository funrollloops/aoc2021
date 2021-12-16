use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    const NUM_DIGITS: usize = 12;
    let mut num_lines = 0;
    let mut ones: [i32; NUM_DIGITS] = [0; NUM_DIGITS];

    for line_or in stdin.lock().lines() {
        let line = line_or.unwrap();
        if line.is_empty() { continue; }
        assert!(line.len() == NUM_DIGITS);
        num_lines += 1;
        // println!("{}", line);
        for i in 0..NUM_DIGITS {
            if line.chars().nth(i).unwrap() == '1' {
                ones[i] += 1;
            }
        }
    }
    let mut g: i64 = 0;
    let mut e: i64 = 0;
    print!("𝛾=");
    for i in 0..NUM_DIGITS {
        g = g << 1;
        e = e << 1;
        if ones[i]  > num_lines / 2 {
            g += 1;
            print!("1");
        } else {
            e += 1;
            print!("0");
        }
    }
    print!("\n");
    println!("g={} e={} e*g={}", g, e, g*e);
    Ok(())
}
