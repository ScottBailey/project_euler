/*

https://projecteuler.net


There are exactly ten ways of selecting three from five, 12345:

    123, 124, 125, 134, 135, 145, 234, 235, 245, and 345

In combinatorics, we use notation: (5 choose 3) = 10. (modified from original due to graphics.

In general,

(n choose r) = n! / (r! * ( (n-r)! ) ) where r <= n , n! = n * n-1 * ... * 3 * 2 * 1, and 0! = 1

It is not until n=23, that a value exceeds one-million: (23 choose 10) = 1144066.

How many, not necessarily distinct, values of (n choose r) for 1 <= n <= 100, are greater than one-million?

NOTES:


*/

use num_bigint::BigUint;

fn solve() -> u64 {

    let mut factorial = Vec::<BigUint>::with_capacity(101);
    {
        // load factorial with all the values from 0 to 100
        factorial.push(BigUint::from(1_u64)); // this is 0!
        let mut n = 1_u64;
        while n <= 100 {
            factorial.push(factorial.last().unwrap().clone() * n);
            n += 1;
        }
    }

    let mut rv = 0; // return value: count of values > 1,000,000

    // nCr for n in [1,100], r <= n
    for n in 1..=100 {
        for r in 1..=n {
            let result = factorial[n].clone() / (factorial[r].clone() * factorial[n-r].clone());
            if result > BigUint::from(1_000_000_u64) {
                rv += 1;
            }
        }
    }

    rv
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


#[test]
fn test_bcd() {
    let mut n;

    n = sb::math::to_bcd(1);
    assert!(n == 0x01);
    n = sb::math::from_bcd(n);
    assert!(n == 1);

    n = sb::math::to_bcd(90);
    assert!(n == 0x09_00);
    n = sb::math::from_bcd(n);
    assert!(n == 90);

    n = sb::math::to_bcd(100);
    assert!(n == 0x01_00_00);
    n = sb::math::from_bcd(n);
    assert!(n == 100);

}
