/*

https://projecteuler.net

The prime 41, can be written as the sum of six consecutive primes:

     41 = 2 + 3 + 5 + 7 + 11 + 13

This is the longest sum of consecutive primes that adds to a prime
below one-hundred.

The longest sum of consecutive primes below one-thousand that adds to
a prime, contains 21 terms, and is equal to 953.

Which prime, below one-million, can be written as the sum of the most
consecutive primes?

NOTES:

*/


fn solve() -> u64 {

    const MAX : u64 = 1_000_000;  // test with 100 and 1000
    let primes = sb::math::prime_to(MAX); // get all the primes to MAX

    let mut rv = 0;
    let mut rv_count = 0;

    // start location inside the prime vector
    for first in 0..primes.len() {
        if primes[first] > MAX { // done
            break;
        }

        let mut sum = 0;

        // add primes from first until our sum exceeds MAX
        for i in first..primes.len() {
            sum += primes[i];
            if sum > MAX { // too big?
                break;
            }

            let count = i-first; // count of primes to create sum
            if count > rv_count { // only test if this count is larger than the best so far
                // if sum is a prime, then it's the best so far, store that
                if let Ok(_) = primes.binary_search(&sum) {
                    rv = sum;
                    rv_count = count;
                }
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
