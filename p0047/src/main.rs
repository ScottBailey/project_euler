/*

https://projecteuler.net

The first two consecutive numbers to have two distinct prime factors are:

   14 = 2 × 7
   15 = 3 × 5

The first three consecutive numbers to have three distinct prime factors are:

   644 = 2² × 7 × 23
   645 = 3 × 5 × 43
   646 = 2 × 17 × 19.

Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?



NOTES:

*/


fn solve() -> u64 {

    let need = 4;  // test with values of 2 and 3 before final run with 4

    let mut n = 5;
    let mut count = 0; // the number of passes

    // a vector to hold primes, preload it with the first value
    let mut primes = sb::math::prime_to(n);

    loop {
        // add to primes if it's too small
        if primes.last().unwrap() < &n {
            sb::math::prime_next( &mut primes );
        }

        // find the unique factors of n
        let mut factors = std::vec::Vec::<u64>::new(); // temp factors
        {
            let mut m = n;
            while m > 1 {
                if factors.len() > need {
                    break;
                }
                for i in &primes {
                    if m%i == 0 {
                        if let Err(j) = factors.binary_search(i) {
                            factors.insert(j,*i);
                        }
                        m /= i;
                    }
                }
            }
        }

        // is factors the correct size?
        if factors.len() != need {
            count = 0;
        }
        else {
            count += 1;
            if count == need {
                return 1 + n - need as u64;
            }
        }

        // go to the next number
        n += 1;
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
