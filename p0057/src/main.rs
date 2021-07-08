/*

https://projecteuler.net

It is possible to show that the square root of two can be expressed as
an infinite continued fraction.

 sqrt(2) = 1 + 1/(2 + 1/(2+ 1/(...

By expanding this for the first four iterations, we get:

  1 + 1/2 = 3/2 = 1.5

  1 + 1/(2 +1/2) = 7/5 = 1.4

  1 + 1/(2 +1/(2+1/2)) = 17/12 = 1.41666...

  1 + 1/(2 +1/(2+1/(2 +1/2)) = 41/29 = 1.41379...

The next three expansions are 90/70, 239/169, and 577/408, but the
eighth expansion, 1393/985, is the first example where the number of
digits in the numerator exceeds the number of digits in the
denominator.

In the first one-thousand expansions, how many fractions contain a
numerator with more digits than the denominator?


NOTES:

*/


use num_bigint::BigUint;

fn solve() -> u64 {

    let mut rv = 0; // return value

    let two = BigUint::from(2_u64);

    let mut numer = BigUint::from(1_u64);
    let mut denom = two.clone();

    // note: we aren't testing on the zero iteration

    for _ in 1..1000 {
        // Continue the fraction
        let mut temp_numer = denom.clone();
        temp_numer *= &two;
        temp_numer += numer;
        numer = denom;
        denom = temp_numer;

        // Temporarily add one...
        let mut temp_numer = numer.clone();
        temp_numer += &denom;
        // ...for the test:
        if temp_numer.to_radix_le(10).len() > denom.to_radix_le(10).len()  {
            rv += 1;
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
