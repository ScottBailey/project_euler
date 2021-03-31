// https://projecteuler.net/problem=10

/*

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

*/

fn main() {

    //let max = 10;
    let max = 2_000_000;

    let primes = sb::math::prime_to(max);

    let mut sol = 0;
    for n in primes {
        sol += n;
    }

    println!("{}", sol);
}
