use std::io;
use std::io::BufRead;

const NUM_DIGITS: usize = 12;

fn o2_rating(mut data: Vec<i32>) -> i32 {
    println!("=== o2");
    let mut bit = NUM_DIGITS;
    while data.len() > 1 {
        bit -= 1;
        let mask = 1 << bit;
        // Find most common bit at position.
        let mut num_ones = 0;
        for datum in &data {
            if datum & mask != 0 {
                num_ones += 1;
            }
        }
        let keep_if = if num_ones * 2 >= data.len() { mask } else { 0 };
        // Remove non-matching datums.
        let mut matched = 0;
        for i in 0..data.len() {
            if data[i] & mask == keep_if {
                data[matched] = data[i];
                matched += 1;
            }
        }
        println!(
            "n={} mask={} bit={} ones={} filter={} matched={}",
            data.len(),
            mask,
            bit,
            num_ones,
            keep_if != 0,
            matched
        );
        data.resize(matched, 0);
        assert!(!data.is_empty());
    }
    data[0]
}

fn co2_rating(mut data: Vec<i32>) -> i32 {
    println!("=== co2");
    let mut bit = NUM_DIGITS;
    while data.len() > 1 {
        bit -= 1;
        let mask = 1 << bit;
        // Find most common bit at position.
        let mut num_ones = 0;
        for datum in &data {
            if datum & mask != 0 {
                num_ones += 1;
            }
        }
        let keep_if = if num_ones * 2 >= data.len() { 0 } else { mask };
        // Remove non-matching datums.
        let mut matched = 0;
        for i in 0..data.len() {
            if data[i] & mask == keep_if {
                data[matched] = data[i];
                matched += 1;
            }
        }
        println!(
            "n={} mask={} bit={} ones={} filter={} matched={}",
            data.len(),
            mask,
            bit,
            num_ones,
            keep_if != 0,
            matched
        );
        data.resize(matched, 0);
        assert!(!data.is_empty());
    }
    data[0]
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut num_lines = 0;
    let mut ones: [i32; NUM_DIGITS] = [0; NUM_DIGITS];
    let mut data: Vec<i32> = Vec::new();

    for line_or in stdin.lock().lines() {
        let line = line_or.unwrap();
        if line.is_empty() {
            continue;
        }
        assert!(line.len() == NUM_DIGITS);
        num_lines += 1;
        // println!("{}", line);
        let mut datum: i32 = 0;
        for i in 0..NUM_DIGITS {
            datum <<= 1;
            if line.chars().nth(i).unwrap() == '1' {
                ones[i] += 1;
                datum |= 1;
            }
        }
        data.push(datum);
    }
    let mut g: i64 = 0;
    let mut e: i64 = 0;
    print!("𝛾=");
    for i in 0..NUM_DIGITS {
        g = g << 1;
        e = e << 1;
        if ones[i] > num_lines / 2 {
            g += 1;
            print!("1");
        } else {
            e += 1;
            print!("0");
        }
    }
    print!("\n");
    println!("g={} e={} e*g={}", g, e, g * e);

    let o2 = o2_rating(data.clone());
    let co2 = co2_rating(data);
    println!("o2={} co2={} o2*co2={}", o2, co2, o2 * co2);
    Ok(())
}
