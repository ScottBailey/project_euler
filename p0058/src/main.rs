/*

https://projecteuler.net

Starting with 1 and spiralling anticlockwise in the following way, a
square spiral with side length 7 is formed.

  37 36 35 34 33 32 31
  38 17 16 15 14 13 30
  39 18  5  4  3 12 29
  40 19  6  1  2 11 28
  41 20  7  8  9 10 27
  42 21 22 23 24 25 26
  43 44 45 46 47 48 49

It is interesting to note that the odd squares lie along the bottom
right diagonal, but what is more interesting is that 8 out of the 13
numbers lying along both diagonals are prime; that is, a ratio of 8/13
≈ 62%.

If one complete new layer is wrapped around the spiral above, a square
spiral with side length 9 will be formed. If this process is
continued, what is the side length of the square spiral for which the
ratio of primes along both diagonals first falls below 10%?


NOTES:

First cut:
  Solution: 26241
  Elasped time: 214,633,819 us


*/


fn solve() -> u64 {
    // get a vector of primes up to the value 100
    let mut primes = sb::math::prime_to(100);

    let mut count = 0_u64; // count of corner primes
    let mut total = 1_u64; // total count of corners (includes the center)

    let mut last = 1_u64;  // initial value

    let mut sz = 0; // this is the size to add to get to the next corner
    loop {
        // Increment the corner size
        sz += 2;
        // Ensure we have a prime in the list larger than our test number.
        while primes.last().unwrap() < &(4*sz + last) {
            sb::math::prime_next(&mut primes);
        }
        // iterate over the corners
        for _ in 0..4 { // This could be unwrapped?
            last += sz;
            if let Ok(_) = primes.binary_search(&last) {
                count += 1;
            }
        }
        total += 4;
        // find the fraction of primes
        let fraction = count as f64 / total as f64;
        //println!("{} / {} = {}", count, total, fraction);
        if fraction < 0.1 {
            return sz + 1;
        }
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
