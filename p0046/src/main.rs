/*

https://projecteuler.net

It was proposed by Christian Goldbach that every odd composite number
can be written as the sum of a prime and twice a square.

9 = 7 + 2×12
15 = 7 + 2×22
21 = 3 + 2×32
25 = 7 + 2×32
27 = 19 + 2×22
33 = 31 + 2×12

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as the sum
of a prime and twice a square?

NOTES:

*/


fn solve() -> u64 {

    // a vector to hold primes, preload it with the first value
    let mut primes = Vec::<u64>::new();
    sb::math::prime_next(&mut primes);   // a function to add the next prime to the existing list

    // a vector of 2 x n^2, preload with the first value
    let mut squares = Vec::<u64>::new();
    squares.push(2);

    let mut n = 3;
    loop {
        // add to primes if it's too small
        if primes.last().unwrap() < &n {
            sb::math::prime_next( &mut primes );
        }
        // add to squares if it's too small
        if squares.last().unwrap() < &n {
            let i = squares.len() as u64 + 1;
            squares.push( 2 * i * i );
        }

        // we only look at composites, skip over primes
        if let Err(_) = primes.binary_search(&n) {

            let mut found = false;
            // For every "square"...
            for sq in &squares {
                // ...assuming the square is less than n...
                if sq >= &n {
                    break;
                }
                // ...find out what the prime value wuld need to be and search for it!
                let target = n - sq;
                if let Err(_) = primes.binary_search(&target) {
                    continue; // not prime, go to the next "square"
                }
                // We found the value, set our flag and break out of the loop
                found = true;
                break;
            }

            // if we didn't find a value, then we found the solution!
            if !found {
                return n;
            }
        }

        // go to the next odd number
        n += 2;
    }
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
