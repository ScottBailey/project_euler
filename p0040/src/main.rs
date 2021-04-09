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

NOTES:

I use a Vec::<u32> (v) to receive Champernowne's constant. Then I use
another Vec (n) to hold the current number. Until v achieves the
necessary length, extends v with n and increment n. While memory
inefficient, it's easy to write and understand.

*/


fn increment(v : &mut Vec::<u32>) {
    for n in v.iter_mut().rev() {
        if *n < 9 {
            *n += 1;
            return;
        }
        *n = 0;
    }
    v.insert(0,1);
}

fn solve() -> u32 {
    let mut v = Vec::<u32>::with_capacity(10_000_020);
    let mut n = Vec::<u32>::with_capacity(32);
    n.push(0);
    while v.len() <= 1_000_000 {
        v.extend(&n);
        increment(&mut n);
    }

    v[1] * v[10] * v[100] * v[1000] * v[10_000] * v[100_000] * v[1_000_000]
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
