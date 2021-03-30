// https://projecteuler.net/problem=21

/*

Let d(n) be defined as the sum of proper divisors of n (numbers less
than n which divide evenly into n).  If d(a) = b and d(b) = a, where a
≠ b, then a and b are an amicable pair and each of a and b are called
amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20,
22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284
are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

sol: 31626

*/

fn main() {
    let start_time = std::time::Instant::now();

    let sol = solve(10000);

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);
    println!("Elasped time: {} us", elapsed);
}


fn solve(n : u64) -> u64 {

    let mut v = std::vec::Vec::<u64>::with_capacity(1 + n as usize);
    for i in 0..=n {
        let temp = sb::math::proper_divisors(i as u64);
        let sum = sb::math::sum(&temp);
        v.push(sum);
    }

    let mut sum = 0_u64;

    for (i,j) in v.iter().enumerate() {
        if *j > i as u64 && *j <= n && v[*j as usize] == i as u64 {
            sum += i as u64;
            sum += j;
        }
    }

    sum
}
