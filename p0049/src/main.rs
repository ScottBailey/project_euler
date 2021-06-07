/*

https://projecteuler.net

The arithmetic sequence, 1487, 4817, 8147, in which each of the terms
increases by 3330, is unusual in two ways: (i) each of the three terms
are prime, and, (ii) each of the 4-digit numbers are permutations of
one another.

There are no arithmetic sequences made up of three 1-, 2-, or 3-digit
primes, exhibiting this property, but there is one other 4-digit
increasing sequence.

What 12-digit number do you form by concatenating the three terms in
this sequence?

NOTES:

*/

use std::collections::HashMap;


fn order_digits(mut n : u64) -> u64 {
    let mut digits = Vec::new();
    digits.reserve(4);

    digits.push( n%10 );
    n /= 10;
    digits.push( n%10 );
    n /= 10;
    digits.push( n%10 );
    n /= 10;
    digits.push( n%10 );

    digits.sort();

    digits[0]*1000 + digits[1]*100 + digits[2]*10 + digits[3]
}


fn solve() -> u64 {

    // get all the primes 4 digit and below
    let primes = sb::math::prime_to(10_000);

    // find the first 4 digit prime
    if let Err(first) = primes.binary_search(&1000) {
        // we are going to keep a hash map of ordered digits to primes
        // (e.g. 1487 -> [1487, 4817, 8147, ...]
        let mut map = HashMap::<u64,Vec<u64>>::new();

        // for all the 4 digit primes, put them in the hash map
        for i in first..primes.len() {
            let ordered = order_digits(primes[i]);
            let v = map.entry(ordered).or_insert(Vec::<u64>::new());
            v.push(primes[i]);
        }

        // now, for all the vectors that contain lists of permutaions
        // with at least 3 memebers...
        for (_,v) in map {
            if v.len() < 3 {
                continue;
            }

            // see if there is a sequence (note that the vector is
            // already in ascending order)
            for i in 0..(v.len()-2) {
                for j in (i+1)..(v.len()-1) {
                    let diff = v[j]-v[i];
                    let target = v[j]+diff;

                    if let Ok(_) = v.binary_search(&target) {
                        let rv = v[i]*100_000_000 + v[j]*10_000 + target;
                        if rv != 148_748_178_147 { // we already know this one exists
                            return rv;
                        }
                    }
                }
            }

        }
    }

    panic!("no solution found");
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
