/*

https://projecteuler.net

The primes 3, 7, 109, and 673, are quite remarkable. By taking any two
primes and concatenating them in any order the result will always be
prime. For example, taking 7 and 109, both 7109 and 1097 are
prime. The sum of these four primes, 792, represents the lowest sum
for a set of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes
concatenate to produce another prime.

*/

use primes::PrimeSet;


fn concat(a : u64, b : u64) -> u64 {

    let mut d = a;

    let mut c = b;
    while c > 0 {
        c /= 10;
        d *= 10;
    }
    d + b
}


fn solve() -> u64 {

    // We shouldn't expect sums larger than MAX
    const MAX : u64 = 50_000;
    let mut rv = MAX;

    // Use a library for getting primes
    let mut prime_set = primes::Sieve::new();

    // Iterate over primes all the primes with values less than 5*rv
    for i in 0.. {
        let n1 = prime_set.get(i); // get's the ith prime
        if (n1*5) > rv { // If the resulting value is excessive, we are done.
            break;
        }

        // Find and store all the reasonably sized primes that
        // concatenate to generate primes.
        let mut maybe = Vec::<u64>::new();
        let nsum = 4*n1;
        for j in (i+1).. {
            let n2 = prime_set.get(j);
            if nsum + n2 > rv {
                break;
            }
            if prime_set.is_prime(concat(n1,n2)) && prime_set.is_prime(concat(n2,n1)) {
                maybe.push(n2);
            }
        }

        // Make sure we have enough primes to test.
        if maybe.len() < 4 {
            continue;
        }

        // For all the members of maybe, try to find a set that
        // concatenate into primes.
        for i2 in 0..(maybe.len()-3) {
            let n2 = maybe[i2];
            let sum2 = n1 + n2;
            if sum2 + (3*n2) > rv { // sanity check size
                break;
            }

            for i3 in (i2+1)..(maybe.len()-2) {
                let n3 = maybe[i3];
                let sum3 = sum2 + n3;
                if sum3 + (2*n3) > rv { // sanity check size
                    break;
                }
                if !prime_set.is_prime(concat(n2,n3)) || !prime_set.is_prime(concat(n3,n2)) {
                    continue;
                }

                for i4 in (i3+1)..(maybe.len()-1) {
                    let n4 = maybe[i4];
                    let sum4 = sum3 + n4;
                    if sum4 + n4 > rv { // sanity check size
                        break;
                    }
                    if !prime_set.is_prime(concat(n2,n4)) || !prime_set.is_prime(concat(n4,n2))
                        || !prime_set.is_prime(concat(n3,n4)) || !prime_set.is_prime(concat(n4,n3)) {
                            continue;
                        }

                    for i5 in (i4+1)..maybe.len() {
                        let n5 = maybe[i5];
                        let sum5 = sum4 + n5;
                        if sum5 > rv { // sanity check size
                            break;
                        }

                        if !prime_set.is_prime(concat(n2,n5)) || !prime_set.is_prime(concat(n5,n2))
                            || !prime_set.is_prime(concat(n3,n5)) || !prime_set.is_prime(concat(n5,n3))
                            || !prime_set.is_prime(concat(n4,n5)) || !prime_set.is_prime(concat(n5,n4)) {
                                continue;
                            }

                        rv = sum5; // found it
                    }
                }
            }
        }
    }

    // Test to see if we found an answer.
    if rv == MAX {
        panic!("Failed to find an answer");
    }
    // Test to see if it's the right answer.
    if rv != 26033 {
        panic!("Found the wrong answer. Expected 26033, found {}", rv);
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
