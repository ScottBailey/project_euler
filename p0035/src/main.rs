/*

https://projecteuler.net

The number, 197, is called a circular prime because all rotations of
the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31,
37, 71, 73, 79, and 97.

How many circular primes are there below one million?

NOTES:


*/

// return vector of digits least significant digit first
fn digits(n : u64) -> Vec::<u64> {
    let mut rv = Vec::<u64>::new();
    let mut temp = n;
    while temp > 0 {
        rv.push(temp%10);
        temp /= 10;
    }
    if rv.is_empty() {
        rv.push(0);
    }
    rv
}


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

    let primes = sb::math::prime_to(1_000_000);

    for n in &primes {
        let mut good = true;
        let mut v = digits(*n);
        for _ in 1..v.len() {
            v.rotate_right(1);
            let temp = vec_to_number(&v);
            if let Err(_) = primes.binary_search(&temp) {
                good = false;
                break;
            }
        }
        rv += good as u64;
    }

    rv
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
