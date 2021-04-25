/*

https://projecteuler.net

The number, 1406357289, is a 0 to 9 pandigital number because it is
made up of each of the digits 0 to 9 in some order, but it also has a
rather interesting sub-string divisibility property.

Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:

   d2d3d4=406 is divisible by 2
   d3d4d5=063 is divisible by 3
   d4d5d6=635 is divisible by 5
   d5d6d7=357 is divisible by 7
   d6d7d8=572 is divisible by 11
   d7d8d9=728 is divisible by 13
  d8d9d10=289 is divisible by 17

Find the sum of all 0 to 9 pandigital numbers with this property.

NOTES:

*/


trait IsPandigital {
    fn is_pandigital(&self) -> bool;
}
impl IsPandigital for u64 {
    fn is_pandigital(&self) -> bool {
        let mut v = [false; 10];
        let mut n = *self;
        let mut count = 0;
        while n > 0 {
            count += 1;
            v[ (n%10) as usize ] = true;
            n /= 10;
        }
        match count {
            1 => v[1],
            2 => v[1] && v[2],
            3 => v[1] && v[2] && v[3],
            4 => v[1] && v[2] && v[3] && v[4],
            5 => v[1] && v[2] && v[3] && v[4] && v[5],
            6 => v[1] && v[2] && v[3] && v[4] && v[5] && v[6],
            7 => v[1] && v[2] && v[3] && v[4] && v[5] && v[6] && v[7],
            8 => v[1] && v[2] && v[3] && v[4] && v[5] && v[6] && v[7] && v[8],
            9 => v[1] && v[2] && v[3] && v[4] && v[5] && v[6] && v[7] && v[8] && v[9],
            10 => v[1] && v[2] && v[3] && v[4] && v[5] && v[6] && v[7] && v[8] && v[9] && v[0],
            _ => false,
        }
    }
}
#[test]
fn test_pandigital() {
    assert!(123456789_u64.is_pandigital());
    assert!(1234567890_u64.is_pandigital());
    assert!(12345_u64.is_pandigital());
    assert!(!12645_u64.is_pandigital());
    assert!(!2345_u64.is_pandigital());
    assert!(!1234567891_u64.is_pandigital());
    assert!(!234560789_u64.is_pandigital());
}


trait IsUniqueDigits {
    fn is_uniquedigits(&self) -> bool;
}
impl IsUniqueDigits for u64 {
    fn is_uniquedigits(&self) -> bool {
        let mut v = [0_usize; 10];
        let mut n = *self;
        while n > 0 {
            v[ (n%10) as usize ] += 1;
            n /= 10;
        }
        for count in v.iter() {
            if *count > 1 {
                return false;
            }
        }
        true
    }
}
#[test]
fn test_uniquedigits() {
    assert!(123456789_u64.is_uniquedigits());
    assert!(1234567890_u64.is_uniquedigits());
    assert!(12345_u64.is_uniquedigits());
    assert!(12645_u64.is_uniquedigits());
    assert!(2345_u64.is_uniquedigits());
    assert!(!1234567891_u64.is_uniquedigits());
    assert!(!12345607890_u64.is_uniquedigits());
}


fn solve() -> u64 {
    0
}


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    let mut remain = elapsed;
    let mut s = String::new();
    if remain == 0 {
        s.insert(0,'0');
    }
    while remain > 0 {
        let temp = remain%1000;
        remain /= 1000;
        if remain > 0 {
            s = format!(",{:03}",temp) + &s;
        }
        else {
            s = format!("{}",temp) + &s;
        }
    }
    println!("Elasped time: {} us", s);
}
