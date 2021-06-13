/*

https://projecteuler.net

It can be seen that the number, 125874, and its double, 251748,
contain exactly the same digits, but in a different order.

Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and
6x, contain the same digits.

NOTES:

*/

fn count_digits(mut n : u64) -> u64 {

    let mut rv = 0;

    while n > 0 {
        rv += 1 << (n%10)*4;
        n /= 10;
    }

    rv
}


fn solve() -> u64 {

    let mut n = 0;
    loop {

        n += 1;

        let target = count_digits(n);
        if target != count_digits(n*2) {
            continue;
        }
        if target != count_digits(n*3) {
            continue;
        }
        if target != count_digits(n*4) {
            continue;
        }
        if target != count_digits(n*5) {
            continue;
        }
        if target != count_digits(n*6) {
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
