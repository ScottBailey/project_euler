// https://projecteuler.net/problem=26

/*

A unit fraction contains 1 in the numerator. The decimal
representation of the unit fractions with denominators 2 to 10 are
given:

1/2	= 	0.5
1/3	= 	0.(3)
1/4	= 	0.25
1/5	= 	0.2
1/6	= 	0.1(6)
1/7	= 	0.(142857)
1/8	= 	0.125
1/9	= 	0.(1)
1/10	= 	0.1

Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It
can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest
recurring cycle in its decimal fraction part.

*/

fn solve() -> u64 {

    // values for divisors 10 and below are in the chart above, use them here:
    let mut cycle = 6;  // longest legth so far
    let mut num = 7;    // divisor generating the longest length so far

    for d in 11..1000 { // d is for divisor

        let mut n = 1_u32; // this is the numerator and remainder

        // v will contain ALL the fractional digits
        let mut v = std::vec::Vec::<u32>::with_capacity(500); // sized large because we can?
        loop {
            // did we get an even division?
            if n == 0 {
                break;
            }
            // store the number
            v.push(n);

            // test to see if the vector has a repeat
            let r = repeats(&v);
            if r > 0 {
                // test to see if the repeat is a new max
                if r > cycle {
                    // set the new max
                    cycle = r;
                    num = d;
                }
                break; // go on to the next divisor
            }

            // long division happens here:
            n *= 10;
            if n >= d {
                n %= d; // can't mod by zero
            }
        }
    }

    num as u64
}


/// return 0 if no repetitions, yet; otherwise the length of the repetition
fn repeats(v : &std::vec::Vec::<u32>) -> u32 {
    // starting backwards...

    let target = *v.last().unwrap(); // v is guaranteed at least one element
    // sanity check
    //if target == 0 {
    //return 0;
    //}

    for (i,n) in v.iter().enumerate() {
        if *n == target {
            let delta = v.len() - 1 - i;
            return delta as u32;
        }
    }

    return 0;
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
