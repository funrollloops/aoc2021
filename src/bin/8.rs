use std::io;
use std::io::BufRead;

fn parse(tok: &str) -> u8 {
    let mut d: u8 = 0;
    for byte in tok.bytes() {
        d |= 1 << (byte - b'a');
    }
    d
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut total = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts = line.split_once("|").unwrap();
        let mut digits: [u8; 10] = [0; 10]; // Digit -> segment mappings.
        // First assign "easy" digits.
        digits[8] = 0x7f;
        for tok in parts.0.trim().split_whitespace() {
            match tok.len() {
                2 => digits[1] = parse(tok),
                3 => digits[7] = parse(tok),
                4 => digits[4] = parse(tok),
                _ => continue
            }
        }
        // Now infer remaining digits.
        let eg = digits[8] & !(digits[7] | digits[4]);
        for tok in parts.0.trim().split_whitespace() {
            let u = parse(tok);
            if tok.len() == 5 {
                if u & eg == eg { digits[2] = u; }
                else if u & digits[1] == digits[1] { digits[3] = u; }
                else { digits[5] = u; }
            } else if tok.len() == 6 {
                if u & eg != eg { digits[9] = u; }
                else if u & digits[1] == digits[1] { digits[0] = u; }
                else { digits[6] = u; }
            }
        }
        let mut num = 0;
        for tok in parts.1.trim().split_whitespace() {
            let d = parse(tok);
            num = num*10 + digits.iter().position(|i| *i == d).unwrap();
        }
        println!("num={} digits={:?} line={:?}", num, digits, line);
        total += num;
    }
    println!("total={}", total);
    Ok(())
}
