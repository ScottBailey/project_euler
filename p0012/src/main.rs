// https://projecteuler.net/problem=12

/*

The sequence of triangle numbers is generated by adding the natural
numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7
= 28. The first ten terms would be:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

Let us list the factors of the first seven triangle numbers:

 1: 1
 3: 1,3
 6: 1,2,3,6
10: 1,2,5,10
15: 1,3,5,15
21: 1,3,7,21
28: 1,2,4,7,14,28
We can see that 28 is the first triangle number to have over five divisors.

What is the value of the first triangle number to have over five hundred divisors?

*/

/*

This brute force method takes a long time. I suspect there is a faster way to check.

*/



fn main() {

    let start_time = std::time::Instant::now();

    let mut n = 0;
    let mut index = 1;
    loop {
        n += index;
        index += 1;

        let factors = sb::math::divisors(n);
        /*
        print!("{}: {}", n, factors[0]);
        for i in 1..factors.len() {
            print!(", {}", factors[i]);
        }
        println!();

        if factors.len() >= 6 {
            break;
        }
         */
        if factors.len() > 500 {
            println!("\nSolution: {} has {} factors!", n, factors.len());
            break;
        }
    }

    println!("\nElasped time: {} us", start_time.elapsed().as_micros());
}
