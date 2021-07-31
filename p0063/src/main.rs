/*

https://projecteuler.net

The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the
9-digit number, 134217728=8^9, is a ninth power.

How many n-digit positive integers exist which are also an nth power?

*/

//use math::round;

fn digit_count(mut n : u128) ->u128 {

    let mut rv = 0;
    while n > 0 {
        rv += 1;
        n /= 10;
    }

    rv
}


fn pow(x :u128, y :u128) -> u128 {

    if y == 0 {
        return 1;
    }
    let mut rv = x;
    for _ in 2..=y {
        rv *= x;
    }

    rv
}


fn solve() -> u128 {

    let mut rv = 0;

    /*
    let mut this_min = 0;
    for y in 1..21 {
        let this_max = pow(10,y)-1;

        let lo = round::ceil( (this_min as f64).powf(1.0/ (y as f64)), 0) as u128;

        let hi = (this_max as f64).powf(1.0/ (y as f64)) as u128;

        if hi < lo {
            break;
        }
        rv += 1+hi-lo;

        println!("y={}, {} {}, {}", y, lo, hi, rv);

        this_min = this_max+1;
    }
     */

    for y in 1.. {
        let mut found = false;
        for x in 1.. {
            let n = pow(x,y);
            let c = digit_count(n);
            if c == y {
                rv += 1;
                found = true;
            }
            if c > y {
                break;
            }
        }
        if !found {
            break;
        }
    }

    rv
}


fn main() {

    let start_time = std::time::Instant::now();

    let sol = solve();

    let elapsed = start_time.elapsed().as_micros();
    println!("\nSolution: {}", sol);

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
