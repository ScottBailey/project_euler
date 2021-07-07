/*

https://projecteuler.net

A googol (10^100) is a massive number: one followed by one-hundred
zeros; 100^100 is almost unimaginably large: one followed by
two-hundred zeros. Despite their size, the sum of the digits in each
number is only 1.

Considering natural numbers of the form, a^b, where a, b < 100, what
is the maximum digital sum?

NOTES:

*/


use num_bigint::BigUint;


fn solve() -> u64 {

    let mut max = 0;

    for a in (1..100_u64).rev() {
        for b in (1..100_u32).rev() {
            // Create the number: a^b
            let num = BigUint::from(a).pow(b);
            // Iterate over the digits of the number, summing them:
            let mut sum = 0_u64;
            let digits = num.to_radix_le(10);
            if ((digits.len() * 9) as u64) < max {
                if b == 99 {
                    return max;
                }
            }
            for n in digits {
                sum += n as u64;
            }
            // If this is the new max, take it.
            if sum > max {
                max = sum;
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
