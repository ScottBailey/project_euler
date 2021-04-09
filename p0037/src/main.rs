/*

https://projecteuler.net

The number 3797 has an interesting property. Being prime itself, it is
possible to continuously remove digits from left to right, and remain
prime at each stage: 3797, 797, 97, and 7. Similarly we can work from
right to left: 3797, 379, 37, and 3.

Find the sum of the only eleven primes that are both truncatable from
left to right and right to left.

NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

NOTES:

*/

// convert vector of digits least significant digit first to u64
fn vec_to_number(v : &Vec::<u64>) -> u64 {
    let mut rv = 0;

    for n in v.iter().rev() {
        rv *= 10;
        rv += *n;
    }

    rv
}


fn solve() -> u64 {

    let mut rv = 0;
    let mut count = 0;

    let mut primes = sb::math::prime_to(10);
    loop {
        sb::math::prime_next(&mut primes);
        let n = primes.last().unwrap();

        let mut n2 = Vec::<u64>::new();
        let mut temp = *n;
        let mut is_prime = true;
        loop {
            n2.push(temp%10);
            temp /= 10;
            if temp == 0 {
                break;
            }
            if let Err(_) = primes.binary_search(&temp) {
                is_prime = false;
                break;
            }
        }
        if !is_prime {
            continue;
        }
        loop {
            n2.pop();
            if  n2.is_empty() {
                break;
            }
            if let Err(_) = primes.binary_search(&vec_to_number(&n2)) {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
            rv += n;
            println!("{}", n);
            if count == 11 {
                return rv;
            }
        }
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
