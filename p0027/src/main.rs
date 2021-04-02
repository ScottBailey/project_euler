// https://projecteuler.net/problem=27

/*

Euler discovered the remarkable quadratic formula:

     n^2 + n + 41

It turns out that the formula will produce 40 primes for the
consecutive integer values '0 <= n <= 39'. However, when
'n = 40, 40^2 + 40 + 41 = 40(40 + 1) +41' is divisible by 41,
and certainly when 'n = 41, 41^2 + 41 + 41' is clearly divisible
by 41.

The incredible formula 'n^2 - 79n + 1601' was discovered, which
produces 80 primes for the consecutive values '0 <= n <= 79'. The
product of the coefficients, −79 and 1601, is −126479.

Considering quadratics of the form:

   n^2 + an + b, where abs(a) < 1000 and abs(b) <= 1000

   where abs(n) is the modulus/absolute value of n
   e.g.  abs(11) = 11 and abs(-4) = 4

Find the product of the coefficients, a and b, for the quadratic
expression that produces the maximum number of primes for consecutive
values of n, starting with n = 0.

*/

fn solve() -> i64 {

    // find a bunch of primes.
    let prime_list = sb::math::prime_to(100_000);  // This generates adeuqately large primes.

    // b's range is [-1000,1000]; however, since n begins at 0, b must
    // be prime, so lets just copy the primes that matter...
    let mut b_list = std::vec::Vec::<i64>::with_capacity(200);
    for n in &prime_list {
        if *n > 1000 {
            break;
        }
        b_list.push(*n as i64);
    }

    // max values...
    let mut count = 0;
    let mut sol_a = 0;
    let mut sol_b = 0;

    for a in -999..1000 {
        for b in &b_list {
            let n = count_primes(&prime_list,a,*b);
            if n > count {
                count = n;
                sol_a = a;
                sol_b = *b;
            }
        }
    }
    // println!("{} {}  count={}", sol_a, sol_b, count);
    return sol_a * sol_b;
}


fn count_primes(prime_list : &std::vec::Vec::<u64>, a : i64, b : i64) -> u64 {
    let mut n = 0;
    loop {
        let temp = n*n + a*n + b;
        if temp < 0 {
            return n as u64;
        }
        // this could have been a match with Err(x) and Ok(x), but I wanted to write an 'if let'
        if let Err(_) = prime_list.binary_search(&(temp as u64)) {
            return n as u64;
        }
        n += 1; // Ok(_)
    }
}


fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    //println!("Elasped time: {} us", elapsed);

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
