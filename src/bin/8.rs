use std::io;
use std::io::BufRead;


// 0 = abc efg    6
// 1 =   c  f  2
// 2 = a cde g   5
// 3 = a cd fg   5
// 4 =  bcd f  4
// 5 = ab d fg   5
// 6 = ab defg    6
// 7 = a c  f  3
// 8 = abcdefg 7
// 9 = abcd fg    6
//
// ## 5s: 
// 2 = a cde g
// 3 = a cd fg
// 5 = ab d fg
// adg common
// b or no c=>5 e=>2 otherwise => 3
//
// ## 6s:
// 0 = abc efg
// 6 = ab defg
// 9 = abcd fg
// abfg common, one each of cde missing
//
// 1 =   c  f
// 4 =  bcd f => bd
// 7 = a c  f => a
// 1 gives us cf
// 7-1 gives us a (useless, everything has a)
// 4-1 gives us bd
// 8-7-4 = eg
//
//  len=6
//      don't have both e and g? => 9
//      superset of 1? => zero
//      _ => 6
//  len=5
//      has e and g? => 2
//      superset of 1? => 3
//      _ => 5
//
//
// 
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
        // Unique.
        let mut one: u8 = 0;
        let mut four: u8 = 0;
        let mut seven: u8 = 0;
        const EIGHT: u8 = 0x7f;
        // TBD.
        let mut unk5: Vec<u8> = Vec::new();
        let mut unk6: Vec<u8> = Vec::new();

        for tok in parts.0.trim().split_whitespace() {
            let d = parse(tok);
            //println!("d={:08b} tok={}", d, tok);
            match tok.len() {
                2 => one = d,
                3 => seven = d,
                4 => four = d,
                5 => unk5.push(d),
                6 => unk6.push(d),
                7 => assert!(d == EIGHT),
                _ => panic!("bad token {}", tok),
            }
        }
        let eg = EIGHT & !(seven | four);
        assert!(eg.count_ones() == 2, "7={:08b} 4={:08b} 8={:08b} e={:08b}", seven, four, EIGHT, eg);
        assert!(unk5.len() == 3);
        assert!(unk6.len() == 3);
        let mut digits = [0, one, 0, 0, four, 0, 0, seven, EIGHT, 0];
        for u in unk5 {
            if u & eg == eg { digits[2] = u; }
            else if u & one == one { digits[3] = u; }
            else { digits[5] = u; }
        }
        for u in unk6 {
            if u & eg != eg { digits[9] = u; }
            else if u & one == one { digits[0] = u; }
            else { digits[6] = u; }
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
