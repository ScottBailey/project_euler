// https://projecteuler.net/problem=5

/*

2520 is the smallest number that can be divided by each of the numbers
from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all
of the numbers from 1 to 20?

*/

fn main() {

    let max = 20;

    let primes = sb::math::prime_to(max);

    let mut factors = std::vec::Vec::<u64>::new();
    factors.reserve((max+1) as usize);
    for _i in 0..=*primes.last().unwrap_or(&max)+1 {
        factors.push(0);
    }

    for i in 2..max {
        let f = sb::math::factors(i);
        for p in &primes {
            let mut count = 0;
            for n in &f {
                if *n == *p {
                    count += 1;
                }
            }
            if count > 0 {
                if factors[*p as usize] < count {
                    factors[*p as usize] = count;
                }
            }
        }
    }

    let mut sol = 1;
    for i in 2..factors.len()-1 {
        if factors[i] > 0 {
            sol *= u64::pow(i as u64,factors[i] as u32);
        }
    }

    println!("{}", sol);
}
