/*

https://projecteuler.net

We shall say that an n-digit number is pandigital if it makes use of
all the digits 1 to n exactly once. For example, 2143 is a 4-digit
pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?

NOTES:

Using the is_pandigital() trait (with a small tweak to find
pandigitals with fewer digits) for p0038 and the prime_to() function
from previous efforts, simply create the vector of primes to 987654321
and iterate backwards until the first pandigital is found.

I took 396 seconds to solve, that's probably longer than it took to write and compile!

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
        if count == 0 || count > 9 {
            return false;
        }
        for i in 1..=count {
            if !v[i] {
                return false;
            }
        }
        true
    }
}
#[test]
fn test_pandigital() {
    assert!(123456789_u64.is_pandigital());
    assert!(!123456780_u64.is_pandigital());
    assert!(!1234567891_u64.is_pandigital());
    assert!(!23456789_u64.is_pandigital());
    assert!(123456_u64.is_pandigital());
    assert!(!123455_u64.is_pandigital());
    assert!(!12346_u64.is_pandigital());
}


fn solve() -> u64 {

    let v = sb::math::prime_to(987_654_321);
    println!("Found {} primes", v.len());
    for n in v.iter().rev() {
        if n.is_pandigital() {
            return *n;
        }
    }

    panic!("no solution found!");
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
