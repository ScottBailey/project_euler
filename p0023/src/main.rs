// https://projecteuler.net/problem=23

/*

A perfect number is a number for which the sum of its proper divisors
is exactly equal to the number. For example, the sum of the proper
divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28
is a perfect number.

A number n is called deficient if the sum of its proper divisors is
less than n and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the
smallest number that can be written as the sum of two abundant numbers
is 24. By mathematical analysis, it can be shown that all integers
greater than 28123 can be written as the sum of two abundant
numbers. However, this upper limit cannot be reduced any further by
analysis even though it is known that the greatest number that cannot
be expressed as the sum of two abundant numbers is less than this
limit.

Find the sum of all the positive integers which cannot be written as
the sum of two abundant numbers.

*/

fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

    //println!("Elasped time: {} us", elapsed);

    let mut remain = elapsed;
    let mut s = String::new();
    if remain == 0 {
        s.insert_str(0,"0");
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


fn solve() -> u64 {

    let mut rv = 0_u64;

    // create a vector for abundant numbers
    let mut v = std::vec::Vec::<u64>::with_capacity(20000);

    for n in 1..=28123 {
        let sum : u64 = sb::math::proper_divisors(n).iter().sum();
        // store the number if it's abundant
        if sum > n {
            v.push(n);
        }
        if test_number(n,&v) {
            rv += n;
        }
    }

    return rv;
}


fn test_number(n : u64, v : &std::vec::Vec::<u64>) -> bool {
    let half = n/2;
    for i in v.iter() {
        if half < *i as u64 {
            return true;
        }
        let target = n - *i as u64;
        for j in v.iter().rev() {
            if target > *j as u64 {
                break;
            }
            if target == *j {
                return false;
            }
        }
    }
    true
}
