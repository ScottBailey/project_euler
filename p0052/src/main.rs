/*

https://projecteuler.net

It can be seen that the number, 125874, and its double, 251748,
contain exactly the same digits, but in a different order.

Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and
6x, contain the same digits.

NOTES:

*/

fn order_digits(mut n : u64) -> u64 {

    let mut digits = Vec::new();
    digits.reserve(10);

    while n > 0 {
        digits.push( n%10 );
        n /= 10;
    }

    // reverse sort so 0s are to the right (i.e. 2020 becomes 2200, instead of 0022, aka 22)
    digits.sort_by(|a, b| b.cmp(a));

    let mut rv = 0;
    for i in digits {
        rv *= 10;
        rv += i;
    }

    rv
}


fn solve() -> u64 {

    let mut n = 0;
    loop {

        n += 1;

        let target = order_digits(n);
        if target != order_digits(n*2) {
            continue;
        }
        if target != order_digits(n*3) {
            continue;
        }
        if target != order_digits(n*4) {
            continue;
        }
        if target != order_digits(n*5) {
            continue;
        }
        if target != order_digits(n*6) {
            continue;
        }

        return n;
    }
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
