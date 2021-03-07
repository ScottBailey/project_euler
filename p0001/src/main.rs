// https://projecteuler.net/problem=1

/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
 */

// algorithm?
// 3 and 5 loop:
//   for n=1..1000/3
//      keep n*3
//   for n=1..1000/5
//      keep n*5
// sort or merge the lists, removing duplicates
// sum the list

fn main() {
    let mut sum = 0;
    for n in 1..1000 {
        if n%3 == 0 {
            sum += n;
        }
        else if n%5 == 0 {
            sum += n;
        }
    }
    println!("{} ", sum);
}
