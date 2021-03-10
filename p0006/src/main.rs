// https://projecteuler.net/problem=6

/*

The sum of the squares of the first ten natural numbers is,
   1^2+2^2+...+10^2=385

The square of the sum of the first ten natural numbers is,
   (1+2+...+10)^2=552=3025

Hence the difference between the sum of the squares of the first ten
natural numbers and the square of the sum is 3025−385=2640.

Find the difference between the sum of the squares of the first one
hundred natural numbers and the square of the sum.

*/

fn main() {

    let max : u64 = 100;

    let mut sum1 : u64 = 0;
    let mut sum2 : u64 = 0;
    for i in 1..=max {
        sum1 += i*i;
        sum2 += i;
    }
    sum2 *= sum2;

    println!("{} - {} = {}", sum2, sum1, sum2-sum1);
}
