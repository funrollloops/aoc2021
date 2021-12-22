use std::io;

fn read_next(buf: &mut String) -> io::Result<bool> {
    buf.clear();
    if io::stdin().read_line(buf)? == 0 {
        Ok(false)
    } else {
        buf.truncate(buf.trim_end().len());
        // Add an initial and final column of 9s to simplify find_lows.
        buf.insert(0, '9');
        buf.push('9');
        Ok(true)
    }
}

fn find_lows(bufs: &[String; 3]) -> i32 {
    let mut sum_risk_levels: i32 = 0;
    let b0 = bufs[0].as_bytes();
    let b1 = bufs[1].as_bytes();
    let b2 = bufs[2].as_bytes();
    for i in 1..b0.len() - 1 {
        let b = b1[i];
        if b < b0[i] && b < b2[i] && b < b1[i - 1] && b < b1[i + 1] {
            println!("found low={}", b1[i]);
            sum_risk_levels += (b1[i] - b'0') as i32 + 1;
        }
    }
    sum_risk_levels
}

fn main() -> io::Result<()> {
    let mut first_line = String::new();
    read_next(&mut first_line)?;
    // We add an initial and final row of 9s to simplify find_lows.
    let boundary = String::from_utf8(vec![b'9'; first_line.len()]).unwrap();

    let mut bufs: [String; 3] = [
        boundary.clone(),
        first_line,
        String::new(),
    ];
    let mut sum_risk_levels = 0;

    while read_next(&mut bufs[2])? {
        sum_risk_levels += find_lows(&bufs);
        bufs.rotate_left(1);
    }
    // Check last line.
    bufs[2] = boundary;
    sum_risk_levels += find_lows(&bufs);

    println!("total risk level={}", sum_risk_levels);
    Ok(())
}
