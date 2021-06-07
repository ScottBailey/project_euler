/*

https://projecteuler.net

The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.

Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.


NOTES:

*/


fn mypow(n : u64) -> u64 {
    let mut rv = n;
    for _ in 1..n {
        if rv == 0 { // exit early if we get to 0 value
            break;
        }
        rv *= n;
        rv %= 10_000_000_000;
    }
    rv
}


fn solve() -> u64 {
    let mut rv = 0;
    for i in 1..=1000 {
        rv += mypow(i);
        rv %= 10_000_000_000;
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
