// https://projecteuler.net/problem=20

/*

n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

*/

use num_bigint::BigUint;


fn main() {
    let start_time = std::time::Instant::now();

    //let sol = solve(10);
    let sol = solve(100);

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);
    println!("Elasped time: {} us", elapsed);

}


fn solve(n : u64) -> u64 {
    let mut num = BigUint::from(n);
    for i in 2..n {
        num *= i as u64;
    }
    let mut rv = 0;
    let res = num.to_str_radix(10);
    for c in res.bytes() {
        rv += (c-b'0') as u64;
    }
    return rv;
}
