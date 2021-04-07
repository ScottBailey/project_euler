/*

https://projecteuler.net

145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the
factorial of their digits.

Note: As 1! = 1 and 2! = 2 are not sums they are not included.


NOTES:


*/


fn solve() -> u64 {

    // factorials for digits 0 to 9
    //                   0  1  2  3   4    5    6     7      8       9
    let factorial : [u64; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    let max = 7 * factorial[9]; // set an upper limit

    let mut rv = 0;

    for val in 10..max {  // values below 10 are not summed (e.g. 1, 2)
        let mut sum = 0;  // sum of the factorials of the individual digits
        let mut n = val;  // temp for finding individual digits
        while n > 0 {
            sum += factorial[ (n%10) as usize];
            n /= 10;
        }

        if val == sum { // test for our magic number!
            rv += val;
            //println!("{} {}, {}", val, sum, rv);
        }
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
