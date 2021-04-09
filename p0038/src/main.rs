/*

https://projecteuler.net

Take the number 192 and multiply it by each of 1, 2, and 3:

192 × 1 = 192
192 × 2 = 384
192 × 3 = 576

By concatenating each product we get the 1 to 9 pandigital,
192384576. We will call 192384576 the concatenated product of 192 and
(1,2,3)

The same can be achieved by starting with 9 and multiplying by 1, 2,
3, 4, and 5, giving the pandigital, 918273645, which is the
concatenated product of 9 and (1,2,3,4,5).

What is the largest 1 to 9 pandigital 9-digit number that can be
formed as the concatenated product of an integer with (1,2, ... , n)
where n > 1?

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
        return count == 9 && v[1] && v[2] && v[3] && v[4] && v[5] && v[6] && v[7] && v[8] && v[9];
    }
}
#[test]
fn test_pandigital() {
    assert!(123456789_u64.is_pandigital());
    assert!(!123456780_u64.is_pandigital());
    assert!(!1234567891_u64.is_pandigital());
    assert!(!23456789_u64.is_pandigital());
}


trait Concatenate {
    fn concatenate(&mut self, other : Self) -> Self;
}
impl Concatenate for u64 {
    fn concatenate(&mut self, other : Self) -> Self {
        let mut temp = 10;
        while temp <= other {
            temp *= 10;
        }
        *self *= temp;
        *self += other;
        *self
    }
}
#[test]
fn test_concatenate() {
    assert!(0_u64.concatenate(10) == 10);
    assert!(9_u64.concatenate(10) == 910);
}


trait DigitCount {
    fn digit_count(&self) -> usize;
}
impl DigitCount for u64 {
    fn digit_count(&self) -> usize {
        let mut rv = 1_usize;
        let mut temp = 10;
        while temp <= *self {
            rv += 1;
            temp *= 10;
        }
        rv
    }
}
#[test]
fn test_len() {
    assert!(0_u64.digit_count() == 1);
    assert!(9_u64.digit_count() == 1);
    assert!(10_u64.digit_count() == 2);
    assert!(345_u64.digit_count() == 3);
}


fn solve() -> u64 {
    let mut max = 0;

    for n in 1..10_000_u64 { // max n is a 4 digit number + a 5 digit number, 10,000 results in 2 5 digit numbers
        let mut sol = 0_u64;
        for i in 1_u64.. {
            sol.concatenate(n*i);
            if sol.digit_count() == 9 {
                if sol.is_pandigital() {
                    if sol > max {
                        max = sol;
                    }
                    break;
                }
            }
            if sol.digit_count() > 10 {
                break;
            }
        }
    }

    max
}


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    if sol != 932718654 {
        panic!("expected {}",932718654);
    }


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
