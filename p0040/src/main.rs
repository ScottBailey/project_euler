/*

https://projecteuler.net

An irrational decimal fraction is created by concatenating the
positive integers:

0.123456789101112131415161718192021...
             ^

It can be seen that the 12th digit of the fractional part is 1.

If dn represents the nth digit of the fractional part, find the value
of the following expression.

d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

*/


fn solve() -> u64 {
    let mut s = String::with_capacity(10_000_020);
    let mut n = 0_u64;
    while s.len() <= 1_000_000 {
        s += &n.to_string();
        n += 1;
    }

    let v = s.as_bytes();
    (v[1]-b'0') as u64 * (v[10]-b'0') as u64 * (v[100]-b'0') as u64
        * (v[1_000]-b'0') as u64 * (v[10_000]-b'0') as u64 *
        (v[100_000]-b'0') as u64 * (v[1_000_000]-b'0') as u64
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
